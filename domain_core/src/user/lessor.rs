use chrono::{DateTime, Utc};
use derive_builder::Builder;
use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::utils::Clock;

use super::User;

#[derive(Debug, Clone, Builder, PartialEq, Eq, Validate)]
#[builder(
    pattern = "owned",
    build_fn(
        validate = "Self::validate_builder",
        name = "build",
        error = "DomainCoreError"
    )
)]
pub struct Lessor {
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

impl Lessor {
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

impl Lessor {
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

impl LessorBuilder {
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
    use crate::user::{User, UserBuilder, UserGender, UserStatus};
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
        let now = Utc.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap();
        UserBuilder::default()
            .username("test_user_for_lessor".to_string())
            .email("lessor@example.com".to_string())
            .password("a_valid_password_for_lessor".to_string())
            .avatar("https://example.com/lessor.png".to_string())
            .gender(UserGender::Male)
            .createtime(now)
            .updatetime(now)
            .build()
            .unwrap()
    }

    /// Helper to create a valid LessorBuilder for tests.
    fn create_test_lessor_builder() -> LessorBuilder {
        let now = Utc.with_ymd_and_hms(2025, 6, 14, 10, 0, 0).unwrap();
        LessorBuilder::default()
            .user(create_test_user())
            .phone("0987654321".to_string())
            .createtime(now)
            .updatetime(now)
    }

    // --- Builder and Creation Tests ---

    #[test]
    fn test_build_lessor_success_with_defaults() {
        let builder = create_test_lessor_builder();
        let lessor_result = builder.build();

        assert!(lessor_result.is_ok());
        let lessor = lessor_result.unwrap();

        // Assert default values are set correctly
        assert_eq!(lessor.id(), None);
        assert_eq!(lessor.is_delete(), false);
    }

    #[test]
    fn test_build_lessor_fail_missing_user() {
        let mut builder = create_test_lessor_builder();
        builder.user = None;
        let result = builder.build();
        assert!(
            matches!(result, Err(DomainCoreError::MissingField(field)) if field == "user")
        );
    }

    #[test]
    fn test_build_lessor_fail_missing_phone() {
        let mut builder = create_test_lessor_builder();
        builder.phone = None;
        let result = builder.build();
        assert!(
            matches!(result, Err(DomainCoreError::MissingField(field)) if field == "phone")
        );
    }

    // --- Entity Method Tests ---

    #[test]
    fn test_set_id() {
        let lessor = create_test_lessor_builder().build().unwrap();
        assert_eq!(lessor.id(), None);

        let lessor_with_id = lessor.set_id(Some(202));
        assert_eq!(lessor_with_id.id(), Some(202));
    }

    #[test]
    fn test_set_delete_updates_flag_and_time() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 14, 10, 0, 0).unwrap();
        let mut clock = MockClock::new(start_time);
        let lessor = create_test_lessor_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        assert!(!lessor.is_delete());

        // Advance the clock to a new, known time
        clock.advance(Duration::days(1));

        let deleted_lessor = lessor.set_delete(&clock);

        assert!(deleted_lessor.is_delete());
        assert_eq!(deleted_lessor.updatetime(), clock.now());
        assert_ne!(deleted_lessor.updatetime(), start_time);
    }

    // --- Getter Method Tests ---

    #[test]
    fn test_getters_return_correct_values() {
        let now = Utc.with_ymd_and_hms(2025, 6, 14, 10, 0, 0).unwrap();
        let user = create_test_user();
        let lessor = create_test_lessor_builder()
            .id(Some(88))
            .phone("555-5678".to_string())
            .user(user.clone())
            .createtime(now)
            .build()
            .unwrap();

        assert_eq!(lessor.id(), Some(88));
        assert_eq!(lessor.phone(), "555-5678");
        assert_eq!(lessor.user(), &user);
        assert_eq!(lessor.createtime(), now);
        assert_eq!(lessor.updatetime(), now);

        // Test user_mut by actually mutating the user
        let mut mutable_lessor = lessor.clone();
        let user_ref_mut = mutable_lessor.user_mut();

        // Use a mock clock to test a state change on the nested user
        let clock = MockClock::new(now);
        *user_ref_mut = user_ref_mut.clone().ban_user(&clock);

        assert_eq!(mutable_lessor.user().status(), &UserStatus::Ban);
    }
}
