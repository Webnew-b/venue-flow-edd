

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::rental::RentalStatus;

#[derive(Debug,Clone,PartialEq,Eq,Default)]
pub struct RentalUpdate {
    pub activity_type:Option<String>,

    pub request_comments:Option<String>,

    pub status:Option<RentalStatus>,
}


impl RentalUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.activity_type.is_none()
        && self.request_comments.is_none()
        && self.status.is_none()
    }

    pub fn is_vaild_update_command(&self)
        -> DomainCoreResult<()>
    {
        if self.is_empty() {
            return Err(DomainCoreError::MustIncludeFieldForUpdate);
        }

        Ok(())
    }
}
