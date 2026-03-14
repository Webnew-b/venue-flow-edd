macro_rules! require_field {
    ($field:expr,$name:expr) => {
        if $field.is_none() {
            return Err(DomainCoreError::MissingField($name.to_string()));
        }
    };
}

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use garde::Validate;
use util_macros::Get;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::rental::rental_error::RentalError;
use crate::utils::Clock;

pub mod rental_error;
pub mod rental_update;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RentalStatus {
    Pending,
    Accepted,
    Rejected,
    Finished,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivityType {
    All,
    Exhibition,
    Seminar,
}

impl std::fmt::Display for RentalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RentalStatus::Pending => write!(f, "pending"),
            RentalStatus::Accepted => write!(f, "accepted"),
            RentalStatus::Rejected => write!(f, "rejected"),
            RentalStatus::Finished => write!(f, "finished"),
            RentalStatus::Canceled => write!(f, "canceled"),
        }
    }
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::All => write!(f, "all"),
            ActivityType::Exhibition => write!(f, "exhibition"),
            ActivityType::Seminar => write!(f, "seminar"),
        }
    }
}

#[derive(Debug, Clone, Builder, PartialEq, Eq, Validate, Get)]
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
    id: Option<i64>,

    #[garde(skip)]
    venue_id: i64,

    #[garde(skip)]
    organizer_id: i64,

    #[garde(skip)]
    start_time: DateTime<Utc>,
    #[garde(skip)]
    end_time:   DateTime<Utc>,

    #[garde(skip)]
    activity_type: ActivityType,

    #[builder(default)]
    #[garde(skip)]
    request_comments: Option<String>,

    #[garde(skip)]
    #[builder(default = "RentalStatus::Pending")]
    status: RentalStatus,

    #[garde(skip)]
    createtime: DateTime<Utc>,
    #[garde(skip)]
    updatetime: DateTime<Utc>,
}

impl Rental {
    pub fn update_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn accepet_rental(
        mut self,
        time: &impl Clock,
    ) -> DomainCoreResult<Self> {
        if self.status != RentalStatus::Pending {
            return Err(RentalError::RentalMustBePending.into());
        }
        self.updatetime = time.now();
        self.status = RentalStatus::Accepted;
        Ok(self)
    }

    pub fn cancel_rental(
        mut self,
        organizer_id: i64,
        time: &impl Clock,
    ) -> DomainCoreResult<Self> {
        if self.status != RentalStatus::Pending {
            return Err(RentalError::RentalMustBePending.into());
        }
        if self.organizer_id != organizer_id {
            return Err(
                RentalError::RentalNotOwnedOrganizer(organizer_id).into()
            );
        }
        self.updatetime = time.now();
        self.status = RentalStatus::Canceled;
        Ok(self)
    }

    pub fn reject_rental(
        mut self,
        time: &impl Clock,
    ) -> DomainCoreResult<Self> {
        if self.status != RentalStatus::Pending {
            return Err(RentalError::RentalMustBePending.into());
        }
        self.updatetime = time.now();
        self.status = RentalStatus::Rejected;
        Ok(self)
    }

    pub fn finish_request(mut self, time: &impl Clock) -> Self {
        self.updatetime = time.now();
        self.status = RentalStatus::Finished;
        self
    }

    pub fn set_rental_date(
        mut self,
        time: &impl Clock,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        organizer_id: i64,
    ) -> DomainCoreResult<Self> {
        if self.organizer_id != organizer_id {
            return Err(
                RentalError::RentalNotOwnedOrganizer(organizer_id).into()
            );
        }

        if start_time < time.now() {
            return Err(RentalError::RentalStartTimeMustBeFuture.into());
        }

        if start_time >= end_time {
            let start = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
            let end = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
            return Err(RentalError::InvalidRentalTime(start, end).into());
        }

        self.start_time = start_time;
        self.end_time = end_time;
        self.updatetime = time.now();
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
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone, Utc};

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn create_test_rental(organizer_id: i64, status: RentalStatus) -> Rental {
        let now = Utc.with_ymd_and_hms(2025, 8, 1, 12, 0, 0).unwrap();
        RentalBuilder::default()
            .venue_id(100)
            .organizer_id(organizer_id)
            .start_time(now + Duration::days(1))
            .end_time(now + Duration::days(2))
            .activity_type(ActivityType::All)
            .status(status)
            .createtime(now)
            .updatetime(now)
            .build()
            .expect("Failed to build test rental")
    }

    #[test]
    fn test_accept_rental_fails_if_not_pending() {
        let rental = create_test_rental(123, RentalStatus::Accepted);
        let clock = MockClock { now: Utc::now() };

        let result = rental.accepet_rental(&clock);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            DomainCoreError::RentalError(RentalError::RentalMustBePending)
        ));
    }

    #[test]
    fn test_cancel_rental_fails_for_wrong_organizer() {
        let rental = create_test_rental(123, RentalStatus::Pending);
        let clock = MockClock { now: Utc::now() };

        let result = rental.cancel_rental(999, &clock);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            DomainCoreError::RentalError(RentalError::RentalNotOwnedOrganizer(
                999
            ))
        ));
    }

    #[test]
    fn test_set_rental_date_fails_for_past_start_time() {
        let rental = create_test_rental(123, RentalStatus::Pending);
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2025, 8, 1, 13, 0, 0).unwrap(),
        };
        let past_start = clock.now - Duration::minutes(1);
        let new_end = clock.now + Duration::days(1);

        let result = rental.set_rental_date(&clock, past_start, new_end, 123);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            DomainCoreError::RentalError(
                RentalError::RentalStartTimeMustBeFuture
            )
        ));
    }
}
