
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
use crate::rental::rental_error::RentalError;
use crate::rental::rental_update::RentalUpdate;

pub mod rental_error;
pub mod rental_update;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum RentalStatus {
    Pending,  
    Accepted, 
    Rejected, 
    Finished, 
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
pub struct Rental {

    #[builder(default)]
    #[garde(skip)]
    id:Option<i64>,

    #[garde(skip)]
    venue_id:i64,

    #[garde(skip)]
    organizer_id:i64,


    #[garde(skip)]
    start_time:DateTime<Utc>,
    #[garde(skip)]
    end_time:DateTime<Utc>,

    #[garde(length(min=5,max=200))]
    activity_type:String,

    #[garde(skip)]
    request_comments:Option<String>,


    #[garde(skip)]
    #[builder(default = "RentalStatus::Pending")]
    status:RentalStatus,

    #[garde(skip)]
    createtime:DateTime<Utc>,
    #[garde(skip)]
    updatetime:DateTime<Utc>,
}

impl Rental {
    pub fn accepet_rental(mut self,updatetime:DateTime<Utc>) -> Self {
        self.updatetime = updatetime;
        self.status = RentalStatus::Accepted;
        self
    }

    pub fn reject_rental(mut self,updatetime:DateTime<Utc>) -> Self {
        self.updatetime = updatetime;
        self.status = RentalStatus::Rejected;
        self
    }

    pub fn finish_request(mut self,updatetime:DateTime<Utc>) -> Self {
        self.updatetime = updatetime;
        self.status = RentalStatus::Finished;
        self
    }

    pub fn set_rental_date(
        mut self,
        updatetime:DateTime<Utc>,
        now:DateTime<Utc>,
        start_time:DateTime<Utc>,
        end_time:DateTime<Utc>,
        ) -> DomainCoreResult<Self> {

        if start_time < now {
            return Err(RentalError::RentalStartTimeMustBeFuture.into());
        }

        if start_time >= end_time {
            let start = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
            let end = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
            return Err(RentalError::InvalidRentalTime(start,end).into());
        }

        self.start_time = start_time;
        self.end_time = end_time;
        self.updatetime = updatetime;
        Ok(self)
    }

    pub fn update_venue(
        mut self,
        update:RentalUpdate,
    ) -> DomainCoreResult<Self>  {
        update.is_vaild_update_command()?;
       
        field_fill!(
            self,
            update,
            activity_type
        );

        self.request_comments = update.request_comments;

        Ok(self)
    }

    
}



impl RentalBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> { 
        require_field!(self.venue_id, "venue_id");
        require_field!(self.organizer_id, "organizer_id");
        require_field!(self.start_time, "start_time");
        require_field!(self.end_time, "end_time");
        require_field!(self.activity_type, "activity_type");
        require_field!(self.status, "status");
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");
        Ok(())
    }
}
