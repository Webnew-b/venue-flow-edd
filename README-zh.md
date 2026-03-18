# 场地租赁系统（Venue-Flow）

[English](./README.md) | 中文

一个在真实系统中实现 **Effect-Driven Design (EDD)** 的现代后端架构实验项目。



# 项目介绍

Venue-Flow 是一个使用 **Rust** 编写的后端服务，它实现了一个场地租赁平台，同时也是一个 **Effect-Driven Design（EDD）架构实验验证项目**。

与普通的 Demo 项目不同，本仓库的主要目标是 **验证 EDD 是否能够在真实工程环境中运行**，包括：

- 业务编排能力
- 副作用隔离能力
- 系统可测试性
- 架构演进能力

场地租赁系统本身只是 **用于验证该架构的一种业务领域示例**。



# 项目状态

**版本：** `0.2.0-alpha`

当前项目已经完成 **EDD 架构验证的第一阶段里程碑**。

当前状态：

- 核心业务功能已经全部实现
- **Pure Layer（core / domain / app）架构** 已稳定
- 业务逻辑功能完整
- API 功能可用
- 业务逻辑单元测试已实现

但仍有部分工程组件正在集成中：

- 集成测试目前通过 API 工具和 `curl` 手动执行
- `k6` 压力测试脚本已经存在，但尚未集成到仓库中
- Event 运行时与异步 worker 系统仍处于部分实现阶段

当前版本的重点是验证 **Effect-Driven Design 在工程中的可行性**，而不是生产环境部署。



# 什么是 Effect-Driven Design（EDD）

Effect-Driven Design 是一种以 **识别与组织副作用（Side Effects）** 为核心的架构方法。

在传统架构中，业务逻辑往往与各种外部操作混杂在一起，例如：

- 数据库写入
- 网络请求
- 日志记录
- 消息发送

EDD 提出了一种不同的设计思路：

> **业务逻辑负责做出决策
>  副作用负责执行这些决策**

这种架构将 **业务决策逻辑（decision logic）** 与 **副作用执行（effect execution）** 进行分离。

核心原则：

```
业务逻辑 → 声明副作用
基础设施 → 执行副作用
```

这种分离带来了：

- 更确定性的业务逻辑
- 更容易进行测试
- 更清晰的系统演进能力



# 本项目验证的架构假设

本仓库用于验证 EDD 的几个重要假设。

## 1. 完全可测试的业务逻辑

通过隔离副作用，Pure Layer 可以实现接近 **100% 的确定性测试**。



## 2. 显式的副作用建模

所有副作用都通过 **UseCase 返回的事件（Event）** 表达。

示例：

```
Outcome {
    data: user,
    from_case: AppUseCase::RegisterUser,
    events: vec![
        AppEvent::SendWelcomeEmail { email },
        AppEvent::AssignABTag { user_id }
    ]
}
```

UseCase 描述的是 **应该发生什么**，而不是 **如何执行**。



## 3. 基于 Trait 的副作用抽象

所有外部操作通过 trait 进行抽象。

示例：

```
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<()>;
}
```

这种方式可以实现：

- 依赖注入
- Mock 测试
- 基础设施替换



## 4. 事件驱动的副作用执行

外部副作用通过事件机制与业务逻辑解耦。

```
UseCase → Outcome → Event → Worker → 外部系统
```

优势：

- 更高的系统弹性
- 更清晰的失败处理
- 支持异步执行



## 5. 架构可演进性

当需要新增副作用时，无需修改现有业务逻辑。



# 架构概览

EDD 按 **副作用边界（Side Effect Boundary）** 来组织系统。

```
core → domain → app → event → infra
```

各层职责：

| 层级   | 职责                   | 副作用等级 |
| ---- | ---- | ---- |
| core   | 实体、值对象、业务校验 | 0          |
| domain | 行为契约（trait、DTO） | 1          |
| app    | UseCase 业务编排       | 2          |
| event  | 事件调度与执行         | 2-3        |
| infra  | 真实副作用实现         | 3          |

# 核心业务功能

系统模拟一个场地租赁平台。

## 用户管理

- 用户注册
- 登录
- 联系方式更新
- 角色管理

## 场地管理

- 发布场地
- 编辑场地信息
- 管理场地状态
- 查看场地列表

## 租赁流程

- 浏览可租用场地
- 提交租赁申请
- 接受 / 拒绝租赁请求



# 测试策略

EDD 支持 **按架构层级设计测试策略**。

| 层级   | 测试方式          |
| ----- | ----- |
| core   | 纯单元测试        |
| domain | 契约测试          |
| app    | UseCase Mock 测试 |
| event  | 事件 Schema 验证  |
| infra  | 集成测试          |

Pure Layer（`core/domain/app`）可以 **在没有任何外部依赖的情况下进行测试**。

示例：

```
#[tokio::test]
async fn test_register_user() {
    let repo = MockUserRepository::new();
    let result = register_user(input, &repo).await;

    assert!(result.is_ok());
}
```



# 性能测试

本项目使用 `k6` 进行压力测试。

压力测试不仅用于验证吞吐量和延迟，还用于验证 **Effect-Driven Design 的一个重要架构假设**：

> 在高并发压力下，业务正确性应该保持稳定，同时架构本身的额外开销应保持在可接受范围内。

当前的压测结果得出了两个关键结论：

### 1. 未发现业务状态违例

在测试的状态机场景中，在并发压力下：

- 所有非法状态转换都被正确拒绝
- 业务流程保持稳定

这表明 **EDD 成功将业务规则约束在纯编排层中，而不是依赖副作用执行顺序来保证正确性**。



### 2. EDD 的架构成本在当前阶段是可接受的

虽然在写密集路径上仍然存在瓶颈，但：

- 系统在功能层面仍然可用
- 压力主要集中在副作用边界，而不是业务逻辑本身

换句话说：

**EDD 并没有为系统引入与其架构清晰度和正确性保障不成比例的性能损耗。**



## 结论

在当前阶段，性能测试为 EDD 提供了一个初步的工程验证：

- 能在高压下保持业务正确性
- 更容易定位系统瓶颈
- 在当前系统规模下不会造成明显性能损失



## 未来计划

- 自动化性能基准测试
- 架构性能对比（EDD vs 传统分层架构）



# 快速开始

## 环境要求

```
Rust
PostgreSQL
Redis
```



## 克隆仓库

```
git clone https://github.com/webnew-b/venue-flow.git
cd venue-flow
```



## 配置环境变量

```
cp .env.example .env
```



## 执行数据库迁移

```
cargo run --bin migrate
```



## 启动服务

```
cd run/
bash start.sh
```



## 运行测试

```
cargo test
```



# 贡献

该仓库目前作为 **架构实验项目**，暂不开放 PR 合并。

欢迎提交 Issue 或参与讨论。



# License

Apache License 2.0



# 作者

Maintained by **Lexon**
