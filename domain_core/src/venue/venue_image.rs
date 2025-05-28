use garde::Validate;

use crate::domain_core_error::DomainCoreResult;
use crate::venue::venue_error::VenueError;


#[derive(PartialEq,Eq,Clone,Debug,Validate)]
pub struct VenueImage {
    #[garde(length(min=6,max=200))]
    pub title:String,

    #[garde(url)]
    pub uri:String,

    #[garde(length(min=6,max=500))]
    pub comment:Option<String>,
}

impl VenueImage {
    pub fn is_vaild_update_command(&self)
        -> DomainCoreResult<()>
    {
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

    pub fn new(
        title:String,
        uri:String,
        comment:Option<String>
    ) -> DomainCoreResult<Self> {
        
        let venue = Self {
            title,
            uri,
            comment
        };

        venue.is_vaild_update_command()?;

        Ok(venue)
    }
}
