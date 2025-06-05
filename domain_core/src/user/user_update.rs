use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::user_error::UserError;

use super::UserGender;


#[derive(Debug,Clone,PartialEq,Eq,Default,Validate)]
pub struct UserUpdate {
    #[garde(length(min = 5,max = 200))]
    pub username:Option<String>,

    #[garde(email)]
    pub email:Option<String>,

    #[garde(length(min = 8,max = 50))]
    pub password:Option<String>,

    #[garde(url)]
    pub avatar:Option<String>,

    #[garde(length(min = 6,max = 200))]
    pub introduce:Option<String>,

    #[garde(skip)]
    pub gender:Option<UserGender>,
}


impl UserUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.username.is_none()
        && self.email.is_none()
        && self.password.is_none()
        && self.avatar.is_none()
        && self.introduce.is_none()
        && self.gender.is_none()
    }

    pub fn valid_update(&self)
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
                return Err(UserError::FieldValidatedFail(err_msg).into());
            }
        }

        
        Ok(())
    }
}
