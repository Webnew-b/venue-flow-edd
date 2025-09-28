use chrono::{DateTime, Utc};
use derive_builder::Builder;
use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::User;
use crate::utils::Clock;

#[derive(Debug, Clone, Builder, PartialEq, Eq, Validate)]
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
    user: User,

    #[builder(default)]
    #[garde(skip)]
    id: Option<i64>,

    //todo Validate Phone numder is correct format, use custom derive
    #[garde(skip)]
    phone: String,

    #[garde(skip)]
    #[builder(default = "false")]
    is_delete: bool,

    #[garde(skip)]
    createtime: DateTime<Utc>,
    #[garde(skip)]
    updatetime: DateTime<Utc>,
}

impl Organizer {
    pub fn set_id(mut self, id: Option<i64>) -> Self {
        self.id = id;
        self
    }

    pub fn set_delete(mut self, time: &impl Clock) -> Self {
        self.is_delete = true;
        self.updatetime = time.now();
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
        require_field!(self.user, "user");
        require_field!(self.phone, "phone");
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_core_error::DomainCoreError;
    use crate::user::{User, UserBuilder, UserGender};
    use crate::utils::Clock;
    use chrono::{Duration, TimeZone, Utc};

    // --- Test Helpers: Mock Objects and Builders ---

    /// A controllable clock for deterministic testing.
    struct MockClock {
        current_time: DateTime<Utc>,
    }

    impl MockClock {
        fn new(start_time: DateTime<Utc>) -> Self {
            Self {
                current_time: start_time,
            }
        }
        fn advance(&mut self, duration: Duration) {
            self.current_time += duration;
        }
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.current_time
        }
    }

    /// Helper to create a valid User for tests.
    fn create_test_user() -> User {
        let now = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        UserBuilder::default()
            .username("test_user_for_organizer".to_string())
            .email("organizer@example.com".to_string())
            .password("a_valid_password_for_organizer".to_string())
            .avatar("https://example.com/organizer.png".to_string())
            .gender(UserGender::Female)
            .createtime(now)
            .updatetime(now)
            .build()
            .unwrap()
    }

    /// Helper to create a valid OrganizerBuilder for tests.
    fn create_test_organizer_builder() -> OrganizerBuilder {
        let now = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        OrganizerBuilder::default()
            .user(create_test_user())
            .phone("1234567890".to_string())
            .createtime(now)
            .updatetime(now)
    }

    // --- Builder and Creation Tests ---

    #[test]
    fn test_build_organizer_success_with_defaults() {
        let builder = create_test_organizer_builder();
        let organizer_result = builder.build();

        assert!(organizer_result.is_ok());
        let organizer = organizer_result.unwrap();

        // Assert default values are set correctly
        assert_eq!(organizer.id(), None);
        assert_eq!(organizer.is_delete(), false);
    }

    #[test]
    fn test_build_organizer_fail_missing_user() {
        let mut builder = create_test_organizer_builder();
        builder.user = None;
        let result = builder.build();
        assert!(
            matches!(result, Err(DomainCoreError::MissingField(field)) if field == "user")
        );
    }

    #[test]
    fn test_build_organizer_fail_missing_phone() {
        let mut builder = create_test_organizer_builder();
        builder.phone = None;
        let result = builder.build();
        assert!(
            matches!(result, Err(DomainCoreError::MissingField(field)) if field == "phone")
        );
    }

    // --- Entity Method Tests ---

    #[test]
    fn test_set_id() {
        let organizer = create_test_organizer_builder().build().unwrap();
        assert_eq!(organizer.id(), None);

        let organizer_with_id = organizer.set_id(Some(101));
        assert_eq!(organizer_with_id.id(), Some(101));
    }

    #[test]
    fn test_set_delete_updates_flag_and_time() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let mut clock = MockClock::new(start_time);
        let organizer = create_test_organizer_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        assert!(!organizer.is_delete());

        // Advance the clock to a new, known time
        clock.advance(Duration::minutes(5));

        let deleted_organizer = organizer.set_delete(&clock);

        assert!(deleted_organizer.is_delete());
        assert_eq!(deleted_organizer.updatetime(), clock.now());
        assert_ne!(deleted_organizer.updatetime(), start_time);
    }

    // --- Getter Method Tests ---

    #[test]
    fn test_getters_return_correct_values() {
        let now = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let user = create_test_user();
        let organizer = create_test_organizer_builder()
            .id(Some(99))
            .phone("555-1234".to_string())
            .user(user.clone())
            .createtime(now)
            .build()
            .unwrap();

        assert_eq!(organizer.id(), Some(99));
        assert_eq!(organizer.phone(), "555-1234");
        assert_eq!(organizer.user(), &user);
        assert_eq!(organizer.createtime(), now);
        assert_eq!(organizer.updatetime(), now);

        // Test user_mut by actually mutating the user
        let mut mutable_organizer = organizer.clone();
        let user_ref_mut = mutable_organizer.user_mut();

        // Since User methods now take a Clock, we need one here to test a state change
        let clock = MockClock::new(now);
        *user_ref_mut = user_ref_mut.clone().ban_user(&clock);

        assert_eq!(
            mutable_organizer.user().status(),
            &crate::user::UserStatus::Ban
        );
    }
}
