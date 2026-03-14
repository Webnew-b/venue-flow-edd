// tests/derive_get_test.rs

// 导入你的派生宏

#[cfg(test)]
mod derive_get_tests {
    use util_macros::Get;

    // 测试具名结构体
    #[test]
    fn test_named_struct_getters() {
        #[derive(Debug, PartialEq, Get)]
        struct Point {
            x:      i32,
            y:      String,
            active: bool,
        }

        let p = Point {
            x:      10,
            y:      "hello".to_string(),
            active: true,
        };

        // 断言 getter 返回的是对原始数据的引用
        assert_eq!(*p.x(), 10);
        assert_eq!(*p.y(), "hello");
        assert_eq!(*p.active(), true);

        // 进一步确认是引用（可选，通过比较指针地址）
        let x_ref_internal: &i32 = &p.x;
        let x_ref_getter: &i32 = p.x();
        assert_eq!(x_ref_internal as *const i32, x_ref_getter as *const i32);

        let y_ref_internal: &String = &p.y;
        let y_ref_getter: &String = p.y();
        assert_eq!(
            y_ref_internal as *const String,
            y_ref_getter as *const String
        );
    }

    // 测试元组结构体
    #[test]
    fn test_tuple_struct_get_all() {
        #[derive(Debug, PartialEq, Get)]
        struct Color(u8, u16, String);

        let c1 = Color(255, 500, "red".to_string());

        // 根据你当前的宏实现，get_all 返回的是值
        // 对于非 Copy 类型 (String)，这意味着值会被移动
        let (r, g, s_val) = c1.get_all();
        assert_eq!(*r, 255);
        assert_eq!(*g, 500);
        assert_eq!(s_val, "red");

        // 注意：由于 String 是非 Copy 类型，c1 中的 String 字段在调用 get_all() 后已被移走。
        // 如果尝试再次使用 c1 中的 String (例如再次调用 c1.get_all() 或直接访问 c1.2)，
        // 会导致编译错误 "use of moved value: `c1.2`"。
        // 这是一个重要的行为，你的宏的使用者需要了解。
        // 如果希望避免移动，get_all 也应该返回引用元组（如我们之前讨论的）。

        // 例如，下面的代码（如果取消注释）会导致编译失败，因为 c1.2 (String) 被移走了：
        // fn consume_color_string(_s: String) {}
        // consume_color_string(c1.2); // Error: use of moved value

        // 如果字段都是 Copy 类型，则没有移动问题
        #[derive(Debug, PartialEq, Get)]
        struct Point2D(i32, i32);
        let pt = Point2D(10, 20);
        let (x_val, y_val) = pt.get_all();
        assert_eq!(*x_val, 10);
        assert_eq!(*y_val, 20);
        // pt 仍然可用，因为 i32 是 Copy
        assert_eq!(pt.0, 10);
    }

    // 测试带泛型的具名结构体
    #[test]
    fn test_generic_named_struct_getters() {
        #[derive(Debug, PartialEq, Get)]
        struct GenericBox<T, U> {
            value: T,
            label: U,
        }

        let gb_int_str = GenericBox {
            value: 123,
            label: "test_label".to_string(),
        };
        assert_eq!(*gb_int_str.value(), 123);
        assert_eq!(*gb_int_str.label(), "test_label");

        let gb_f64_bool = GenericBox {
            value: 3.14159,
            label: false,
        };
        assert_eq!(*gb_f64_bool.value(), 3.14159);
        assert_eq!(*gb_f64_bool.label(), false);
    }

    // 测试带泛型的元组结构体
    #[test]
    fn test_generic_tuple_struct_get_all() {
        #[derive(Debug, PartialEq, Get)]
        struct GenericPair<K, V>(K, V);

        let gp_str_i32 = GenericPair("key1".to_string(), 42);
        let (k_val, v_val) = gp_str_i32.get_all(); // "key1" (String) 会被移动
        assert_eq!(k_val, "key1");
        assert_eq!(*v_val, 42);

        let gp_bool_f32 = GenericPair(true, 99.9f32);
        let (b_flag, f_val) = gp_bool_f32.get_all();
        assert_eq!(*b_flag, true);
        assert_eq!(*f_val, 99.9f32);
    }

    // 提醒：关于字段本身是引用的情况 (如 data: &'a str)
    // 当前宏为具名结构体生成 pub fn field(&self) -> &&'a str { &self.field }
    // 这返回的是引用的引用 (&&'a str)，通常期望的是 &'a str。
    // 这需要对宏进行更细致的处理，以识别字段是否已经是引用。
    // 例如：
    // #[derive(Debug, PartialEq, Get)]
    // struct Borrowed<'a> {
    //     reference: &'a str,
    // }
    // let text = "hello".to_string();
    // let b = Borrowed { reference: &text };
    // let _ref_ref_str: &&str = b.reference(); // 当前宏生成的类型
    // let _direct_ref_str: &str = b.reference;   // 期望的getter返回类型
}
