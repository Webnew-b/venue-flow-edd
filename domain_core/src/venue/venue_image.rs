use chrono::{DateTime, Utc};
use garde::Validate;

use crate::domain_core_error::DomainCoreResult;
use crate::venue::venue_error::VenueError;

#[derive(PartialEq, Eq, Clone, Debug, Validate)]
pub struct VenueImage {
    #[garde(skip)]
    pub id: Option<i64>,

    #[garde(skip)]
    pub venue_id: i64,

    #[garde(length(min = 6, max = 200))]
    pub title: String,

    #[garde(url)]
    pub uri: String,

    #[garde(length(min = 6, max = 500))]
    pub comment: Option<String>,

    #[garde(skip)]
    pub createtime: DateTime<Utc>,
}

impl VenueImage {
    pub fn is_vaild_update_command(&self) -> DomainCoreResult<()> {
        if let Err(e) = self.validate() {
            let mut err_msg = String::new();
            for (path, errors) in e.iter() {
                err_msg.push_str(
                    format!("{}:{};", path, errors.message()).as_str(),
                );
            }

            if !err_msg.is_empty() {
                return Err(VenueError::FieldValidatedFail(err_msg).into());
            }
        }

        Ok(())
    }

    pub fn new(
        venue_id: i64,
        title: String,
        uri: String,
        comment: Option<String>,
        createtime: DateTime<Utc>,
    ) -> DomainCoreResult<Self> {
        let venue = Self {
            id: None,
            venue_id,
            title,
            uri,
            comment,
            createtime,
        };

        venue.is_vaild_update_command()?;

        Ok(venue)
    }

    pub fn update_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}
