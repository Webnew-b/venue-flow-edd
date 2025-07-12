macro_rules! require_field {
    ($field:expr,$name:expr) => {
        if $field.is_none() {
            return Err(DomainCoreError::MissingField($name.to_string()));
        }
    };
}

macro_rules! field_fill {
    ($target:expr,$source:expr,$($field:ident),+) => {
        $(
            if let Some(item) = $source.$field {
                $target.$field = item;
            }
        )+
    };
}

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use garde::Validate;
use util_macros::Get;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::utils::Clock;
use crate::venue::venue_image::VenueImage;
use crate::venue::venue_update::VenueUpdate;

pub mod venue_update;
pub mod venue_image;
pub mod allow_activity;
pub mod venue_error;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum VenueStatus {
    Published,
    Unpublished,
}

#[derive(Debug,Clone,Builder,PartialEq,Eq,Validate,Get)]
#[builder(
    pattern = "owned",
    build_fn(
        validate = "Self::validate_builder",
        name = "build",
        error = "DomainCoreError"
    )
)]
pub struct Venue {

    #[builder(default)]
    #[garde(skip)]
    id:Option<i64>,

    #[garde(skip)]
    lessor_id:i64,

    #[garde(length(min=5,max=200))]
    name:String,

    #[garde(skip)]
    address:String,

    #[garde(skip)]
    images:Vec<VenueImage>,

    #[garde(range(min=1,max=50000))]
    capacity:i32,

    #[garde(skip)]
    #[builder(default)]
    description:Option<String>,

    #[garde(skip)]
    #[builder(default = "true")]
    is_show:bool,

    #[garde(skip)]
    #[builder(default = "false")]
    is_delete:bool,

    #[garde(skip)]
    #[builder(default = "VenueStatus::Unpublished")]
    status:VenueStatus,

    #[garde(skip)]
    createtime:DateTime<Utc>,
    #[garde(skip)]
    updatetime:DateTime<Utc>,
}



impl Venue {
    pub fn list_venue(mut self,time:&impl Clock) -> Self {
        self.updatetime = time.now();
        self.status = VenueStatus::Published;
        self
    }

    pub fn unlist_venue(mut self,time:&impl Clock) -> Self {
        self.updatetime = time.now();
        self.status = VenueStatus::Unpublished;
        self
    }

    pub fn delete_venue(mut self,time:&impl Clock) -> Self {
        self.updatetime = time.now();
        self.is_delete = true;
        self
    }

    pub fn update_venue(
        mut self,
        update:VenueUpdate,
        time:&impl Clock
    ) -> DomainCoreResult<Self>  {
        update.is_vaild_update_command()?;
       
        field_fill!(
            self,
            update,
            name,
            address,
            capacity
        );

        self.description = update.description;
        self.updatetime = time.now();
        Ok(self)
    }

    pub fn update_images(
        mut self,
        images:Vec<VenueImage>,
        time:&impl Clock
    ) -> Self{
        self.images = images;
        self.updatetime = time.now();
        self
    }

    
}

impl VenueBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> { 

        require_field!(self.lessor_id, "lessor_id");
        require_field!(self.name, "name");
        require_field!(self.address, "address");
        require_field!(self.images, "images");
        require_field!(self.capacity, "capacity");
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::venue::venue_error::VenueError;
    use chrono::{TimeZone, Utc};

    // 依然使用 MockClock 来保证时间的可控性
    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    // 辅助函数保持不变
    fn create_test_venue() -> Venue {
        let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        VenueBuilder::default()
            .lessor_id(1)
            .name("Initial Venue Name".to_string())
            .address("123 Main St".to_string())
            .images(vec![])
            .capacity(100)
            .createtime(now)
            .updatetime(now)
            .build()
            .expect("Failed to build test venue")
    }

    #[test]
    fn test_update_venue_success() {
        let venue = create_test_venue();
        let clock = MockClock { now: Utc.with_ymd_and_hms(2025, 7, 12, 13, 0, 0).unwrap() };
        
        // 使用符合 `garde` 校验规则的数据
        let update_data = VenueUpdate {
            name: Some("A Valid Updated Venue Name".to_string()), // length > 5
            capacity: Some(250),
            description: Some("This is a valid description with enough length.".to_string()), // length > 6
            address: Some("456 New Street".to_string()),
        };

        // 确认更新命令本身是合法的
        assert!(update_data.is_vaild_update_command().is_ok());
        
        let updated_venue = venue.update_venue(update_data, &clock).unwrap();

        assert_eq!(updated_venue.name(), "A Valid Updated Venue Name");
        assert_eq!(*updated_venue.capacity(), 250);
        assert_eq!(updated_venue.description(), &Some("This is a valid description with enough length.".to_string()));
        assert_eq!(*updated_venue.updatetime(), clock.now);
    }

    #[test]
    fn test_update_venue_fails_on_empty_update() {
        let venue = create_test_venue();
        let clock = MockClock { now: Utc.with_ymd_and_hms(2025, 7, 12, 14, 0, 0).unwrap() };
        
        // 创建一个所有字段都为 None 的 VenueUpdate
        let empty_update = VenueUpdate::new();
        
        let result = venue.update_venue(empty_update, &clock);

        // 根据 `is_vaild_update_command` 的逻辑，这应该返回 `MustIncludeFieldForUpdate` 错误
        assert!(result.is_err(), "Update should fail when no fields are provided");
        let error = result.err().unwrap();
        assert!(matches!(error, DomainCoreError::MustIncludeFieldForUpdate), "Error should be MustIncludeFieldForUpdate");
    }

    #[test]
    fn test_update_venue_fails_on_garde_validation() {
        let venue = create_test_venue();
        let clock = MockClock { now: Utc.with_ymd_and_hms(2025, 7, 12, 14, 30, 0).unwrap() };

        // 创建一个字段内容不符合 `garde` 规则的 VenueUpdate
        let invalid_update = VenueUpdate {
            name: Some("shor".to_string()), // `name` 长度小于5，校验失败
            ..Default::default()
        };
        
        let result = venue.update_venue(invalid_update, &clock);
        
        // 这应该返回 `FieldValidatedFail` 错误
        assert!(result.is_err(), "Update should fail when a field fails validation");
        let error = result.err().unwrap();
        assert!(matches!(error, DomainCoreError::VenueError(VenueError::FieldValidatedFail(_))), "Error should be FieldValidatedFail");
    }

    #[test]
    fn test_update_images() {
        let venue = create_test_venue();
        let clock = MockClock { now: Utc.with_ymd_and_hms(2025, 7, 12, 15, 0, 0).unwrap() };
        
        // 使用 VenueImage::new 构造函数，它已经包含了内在的校验逻辑
        let new_images = vec![
            VenueImage::new(
                "A Great Title".to_string(), // valid length
                "https://example.com/image1.png".to_string(), // valid url
                None
            ).unwrap(),
            VenueImage::new(
                "Another Awesome Title".to_string(),
                "https://example.com/image2.jpg".to_string(),
                Some("A descriptive comment for the image.".to_string()) // valid length
            ).unwrap(),
        ];

        let updated_venue = venue.update_images(new_images.clone(), &clock);

        assert_eq!(updated_venue.images().len(), 2, "Venue should have 2 images after update");
        assert_eq!(*updated_venue.images(), new_images, "Images should be updated correctly");
        assert_eq!(*updated_venue.updatetime(), clock.now, "Update time should be changed");
    }
}
