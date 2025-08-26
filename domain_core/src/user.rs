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
use util_macros::Get;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::user_update::UserUpdate;
use crate::utils::Clock;

pub mod lessor;
pub mod organizer;
pub mod user_error;
pub mod user_update;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserGender {
    Male,
    Female,
    Nonbinary,
    PreferNotToSay,
}

impl UserGender {
    pub fn get_gender(gender: &str) -> Option<Self> {
        match gender {
            "male" => Some(Self::Male),
            "female" => Some(Self::Female),
            "non-binary" => Some(Self::Nonbinary),
            "prefer-not-to-say" => Some(Self::PreferNotToSay),
            _ => None,
        }
    }
}

impl std::fmt::Display for UserGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserGender::Male => write!(f, "male"),
            UserGender::Female => write!(f, "female"),
            UserGender::Nonbinary => write!(f, "non-binary"),
            UserGender::PreferNotToSay => write!(f, "prefer-not-to-say"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Ban,
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
pub struct User {
    #[builder(default)]
    #[garde(skip)]
    id: Option<i64>,

    #[garde(length(min = 5, max = 200))]
    username: String,
    #[garde(email)]
    email: String,
    #[garde(url)]
    avatar: String,
    #[garde(skip)]
    gender: UserGender,
    #[garde(length(min = 8, max = 50))]
    password: String,

    #[garde(skip)]
    #[builder(default)]
    introduce: Option<String>,

    #[garde(skip)]
    #[builder(default = "true")]
    is_show: bool,

    #[garde(skip)]
    #[builder(default = "false")]
    is_delete: bool,

    #[garde(skip)]
    #[builder(default = "UserStatus::Active")]
    status: UserStatus,

    #[garde(skip)]
    createtime: DateTime<Utc>,
    #[garde(skip)]
    updatetime: DateTime<Utc>,
}

impl User {
    pub fn update_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn update_email(
        mut self,
        new_email: String,
        time: &impl Clock,
    ) -> Self {
        self.email = new_email;
        self.updatetime = time.now();
        self
    }

    pub fn ban_user(mut self, time: &impl Clock) -> Self {
        self.updatetime = time.now();
        self.status = UserStatus::Ban;
        self
    }

    pub fn delete_user(mut self, time: &impl Clock) -> Self {
        self.updatetime = time.now();
        self.is_delete = true;
        self
    }

    pub fn update_gender(
        mut self,
        gender: UserGender,
        time: impl Clock,
    ) -> Self {
        self.gender = gender;
        self.updatetime = time.now();
        self
    }

    pub fn update_user(
        mut self,
        update: UserUpdate,
        time: &impl Clock,
    ) -> DomainCoreResult<Self> {
        update.valid_update()?;

        field_fill!(self, update, username, email, gender, avatar, password);

        self.introduce = update.introduce;
        self.updatetime = time.now();
        Ok(self)
    }
}

impl User {
    pub fn can_login(&self) -> bool {
        !self.is_delete && self.status == UserStatus::Active
    }
}

impl UserBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> {
        require_field!(self.username, "username");
        require_field!(self.email, "email");
        require_field!(self.avatar, "avatar");
        require_field!(self.gender, "gender");
        require_field!(self.createtime, "createtime");
        require_field!(self.updatetime, "updatetime");

        //todo Need to validate field is valid after build.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_core_error::DomainCoreError;
    use crate::user::user_update::UserUpdate;
    use crate::utils::Clock; // 确保 Clock trait 被导入
    use chrono::{Duration, TimeZone, Utc};

    // --- Test Helpers: MockClock and User Builder ---

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

        /// Advances the mock clock's time by a specified duration.
        fn advance(&mut self, duration: Duration) {
            self.current_time += duration;
        }
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.current_time
        }
    }

    /// A helper function to reduce boilerplate when creating a User for tests.
    fn create_test_user_builder() -> UserBuilder {
        let now = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        UserBuilder::default()
            .username("test_user".to_string())
            .email("test@example.com".to_string())
            .password("a_valid_password_longer_than_8".to_string())
            .avatar("https://example.com/avatar.png".to_string())
            .gender(UserGender::PreferNotToSay)
            .createtime(now)
            .updatetime(now)
    }

    // --- Builder and Creation Tests ---

    #[test]
    fn test_build_user_success() {
        let builder = create_test_user_builder();
        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_build_user_fail_missing_required_field() {
        let mut builder = create_test_user_builder();
        builder.username = None;
        let result = builder.build();
        assert!(
            matches!(result, Err(DomainCoreError::MissingField(field)) if field == "username")
        );
    }

    // --- Entity Method and Business Logic Tests ---

    #[test]
    fn test_can_login_logic() {
        let clock = MockClock::new(Utc::now());
        let user = create_test_user_builder().build().unwrap();

        // Active user can log in
        assert!(user.can_login());

        // Banned user cannot log in
        let banned_user = user.clone().ban_user(&clock);
        assert!(!banned_user.can_login());

        // Deleted user cannot log in
        let deleted_user = user.delete_user(&clock);
        assert!(!deleted_user.can_login());
    }

    #[test]
    fn test_ban_user_updates_status_and_time_deterministically() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let mut clock = MockClock::new(start_time);
        let user = create_test_user_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        // Advance the clock to a new, known time
        clock.advance(Duration::seconds(30));

        let banned_user = user.ban_user(&clock);

        assert_eq!(banned_user.status(), &UserStatus::Ban);
        assert_eq!(banned_user.updatetime(), &clock.now()); // Assert against the new, controlled time
        assert_ne!(banned_user.updatetime(), &start_time);
    }

    #[test]
    fn test_delete_user_updates_flag_and_time_deterministically() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let mut clock = MockClock::new(start_time);
        let user = create_test_user_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        clock.advance(Duration::minutes(1));

        let deleted_user = user.delete_user(&clock);

        assert!(deleted_user.is_delete());
        assert_eq!(deleted_user.updatetime(), &clock.now());
    }

    #[test]
    fn test_update_user_success_and_updates_time() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let mut clock = MockClock::new(start_time);
        let user = create_test_user_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        let update_data = UserUpdate {
            username: Some("new_username_is_long_enough".to_string()),
            introduce: Some("new intro".to_string()),
            ..Default::default()
        };

        clock.advance(Duration::hours(1));

        let updated_user = user.update_user(update_data, &clock).unwrap();

        assert_eq!(updated_user.username(), "new_username_is_long_enough");
        assert_eq!(updated_user.introduce(), &Some("new intro".to_string()));
        assert_eq!(updated_user.updatetime(), &clock.now());
    }

    #[test]
    fn test_update_user_validation_failure_does_not_update() {
        let start_time = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();
        let clock = MockClock::new(start_time);
        let user = create_test_user_builder()
            .updatetime(clock.now())
            .build()
            .unwrap();

        // This update will fail validation because the username is too short.
        let invalid_update_data = UserUpdate {
            username: Some("shor".to_string()),
            ..Default::default()
        };

        let result = user.clone().update_user(invalid_update_data, &clock);

        // Assert that the update failed
        assert!(result.is_err());
    }
}
