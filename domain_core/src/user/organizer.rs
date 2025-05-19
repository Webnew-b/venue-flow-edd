use chrono::{DateTime, Utc};
use garde::Validate;
use derive_builder::Builder;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::User;

#[derive(Debug,Clone,Builder,PartialEq,Eq,Validate)]
#[builder(
    pattern = "owned",
    build_fn(
        validate = "Self::validate_builder",
        name = "build",
        error = "DomainCoreError"
    )
)]
pub struct Organizer {

    #[garde(skip)]
    user:User,

    #[builder(default)]
    #[garde(skip)]
    id:Option<i64>,

    //todo Validate Phone numder is correct format, use custom derive
    #[garde(skip)]
    phone:String,

    #[garde(skip)]
    #[builder(default = "false")]
    is_delete:bool,

    #[garde(skip)]
    createtime:DateTime<Utc>,
    #[garde(skip)]
    updatetime:DateTime<Utc>,
}

impl Organizer {
    
    pub fn set_id(mut self, id: Option<i64>) -> Self {
        self.id = id;
        self
    }

    pub fn set_delete(mut self) -> Self {
        self.is_delete = true;
        self
    }
}

impl Organizer {
    pub fn user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn phone(&self) -> &str {
        &self.phone
    }

    pub fn is_delete(&self) -> bool {
        self.is_delete
    }

    pub fn createtime(&self) -> DateTime<Utc> {
        self.createtime
    }

    pub fn updatetime(&self) -> DateTime<Utc> {
        self.updatetime
    }
}


impl OrganizerBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> { 
        require_field!(self.user,"user");
        require_field!(self.phone,"phone");
        require_field!(self.createtime,"createtime");
        require_field!(self.updatetime,"updatetime");
        
        Ok(())
    }
}
