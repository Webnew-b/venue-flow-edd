use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use super::venue_error::VenueError;

#[derive(Debug,Clone,PartialEq,Eq,Default,Validate)]
pub struct VenueUpdate {
    #[garde(length(min = 5,max = 200))]
    pub name:Option<String>,

    #[garde(skip)]
    pub address:Option<String>,

    #[garde(range(min=1,max=50000))]
    pub capacity:Option<i32>,

    #[garde(length(min = 6,max = 200))]
    pub description:Option<String>,
}


impl VenueUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none()
        && self.address.is_none()
        && self.capacity.is_none()
        && self.description.is_none()
    }

    pub fn is_vaild_update_command(&self)
        -> DomainCoreResult<()>
    {
        if self.is_empty() {
            return Err(DomainCoreError::MustIncludeFieldForUpdate);
        }

        if let Err(e) = self.validate() {
            let mut err_msg = String::new();
            for (path,errors) in e.iter(){
                err_msg.push_str(format!("{}:{};",path,errors.message()).as_str());
            }

            if !err_msg.is_empty() {
                return Err(VenueError::FieldValidatedFail(err_msg).into());
            }
        }

        
        Ok(())
    }
}
