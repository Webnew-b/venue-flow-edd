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

#[derive(Debug,Clone,Builder,PartialEq,Eq,Validate)]
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

        Ok(self)
    }

    
}

impl Venue {
    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn lessor_id(&self) -> i64 {
        self.lessor_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn images(&self) -> &[VenueImage] {
        &self.images
    }

    pub fn capacity(&self) -> i32 {
        self.capacity
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn is_show(&self) -> bool {
        self.is_show
    }

    pub fn is_delete(&self) -> bool {
        self.is_delete
    }

    pub fn status(&self) -> &VenueStatus {
        &self.status
    }

    pub fn createtime(&self) -> DateTime<Utc> { 
        self.createtime
    }

    pub fn updatetime(&self) -> DateTime<Utc> {
        self.updatetime
    }
}

impl VenueBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> { 

        require_field!(self.lessor_id, "lessor_id");
        require_field!(self.name, "name");
        require_field!(self.address, "address");
        require_field!(self.images, "images");
        require_field!(self.capacity, "capacity");
        require_field!(self.is_show, "is_show");
        require_field!(self.is_delete, "is_delete");
        require_field!(self.status, "status");
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");

        Ok(())
    }
}
