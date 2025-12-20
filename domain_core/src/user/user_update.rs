use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::user_error::UserError;
use crate::user::HashedPassword;

use super::UserGender;

#[derive(Debug, Clone, PartialEq, Eq, Default, Validate)]
pub struct UserUpdate {
    #[garde(length(min = 5, max = 200))]
    pub username: Option<String>,

    #[garde(email)]
    pub email: Option<String>,

    #[garde(skip)]
    pub password: Option<HashedPassword>,

    #[garde(url)]
    pub avatar: Option<String>,

    #[garde(length(min = 6, max = 200))]
    pub introduce: Option<String>,

    #[garde(skip)]
    pub gender: Option<UserGender>,
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

    pub fn valid_update(&self) -> DomainCoreResult<()> {
        if self.is_empty() {
            return Err(DomainCoreError::MustIncludeFieldForUpdate);
        }

        if let Err(e) = self.validate() {
            let mut err_msg = String::new();
            for (path, errors) in e.iter() {
                err_msg.push_str(
                    format!("{}:{};", path, errors.message()).as_str(),
                );
            }

            if !err_msg.is_empty() {
                return Err(UserError::FieldValidatedFail(err_msg).into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_core_error::DomainCoreError;
    use crate::user::user_error::UserError;
    use crate::user::UserGender;

    // --- State and Constructor Tests ---

    #[test]
    fn test_new_is_empty_by_default() {
        let update = UserUpdate::new();
        assert!(update.is_empty(), "A new UserUpdate should be empty");
    }

    #[test]
    fn test_is_empty_is_false_if_any_field_is_set() {
        let mut update = UserUpdate::new();
        update.username = Some("test".to_string());
        assert!(!update.is_empty(), "Should not be empty if username is set");

        let mut update = UserUpdate::new();
        update.gender = Some(UserGender::Male);
        assert!(!update.is_empty(), "Should not be empty if gender is set");
    }

    // --- `valid_update` Method Tests ---

    #[test]
    fn test_valid_update_fails_on_empty_struct() {
        let update = UserUpdate::new();
        let result = update.valid_update();

        assert!(matches!(
            result,
            Err(DomainCoreError::MustIncludeFieldForUpdate)
        ));
    }

    #[test]
    fn test_valid_update_succeeds_with_valid_data() {
        let update = UserUpdate {
            username: Some("a_valid_username".to_string()),
            email: Some("valid@example.com".to_string()),
            ..Default::default()
        };
        assert!(update.valid_update().is_ok());
    }

    #[test]
    fn test_valid_update_succeeds_with_only_gender() {
        // Gender is not validated by garde, so it should pass on its own
        let update = UserUpdate {
            gender: Some(UserGender::Nonbinary),
            ..Default::default()
        };
        assert!(update.valid_update().is_ok());
    }

    // --- Individual Field Validation Failure Tests ---

    #[test]
    fn test_valid_update_fails_on_short_username() {
        let update = UserUpdate {
            username: Some("shor".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();
        assert!(matches!(
            result,
            Err(DomainCoreError::UserError(UserError::FieldValidatedFail(_)))
        ));
    }

    #[test]
    fn test_valid_update_fails_on_invalid_email() {
        let update = UserUpdate {
            email: Some("not-an-email".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();
        assert!(matches!(
            result,
            Err(DomainCoreError::UserError(UserError::FieldValidatedFail(_)))
        ));
    }

    #[test]
    fn test_valid_update_fails_on_short_password() {
        let update = UserUpdate {
            password: Some("short".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();
        assert!(matches!(
            result,
            Err(DomainCoreError::UserError(UserError::FieldValidatedFail(_)))
        ));
    }

    #[test]
    fn test_valid_update_fails_on_invalid_avatar_url() {
        let update = UserUpdate {
            avatar: Some("not a url".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();
        assert!(matches!(
            result,
            Err(DomainCoreError::UserError(UserError::FieldValidatedFail(_)))
        ));
    }

    #[test]
    fn test_valid_update_fails_on_short_introduce() {
        let update = UserUpdate {
            introduce: Some("short".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();
        assert!(matches!(
            result,
            Err(DomainCoreError::UserError(UserError::FieldValidatedFail(_)))
        ));
    }

    #[test]
    fn test_error_message_contains_path_and_message() {
        let update = UserUpdate {
            email: Some("invalid-email".to_string()),
            ..Default::default()
        };
        let result = update.valid_update();

        if let Err(DomainCoreError::UserError(UserError::FieldValidatedFail(
            msg,
        ))) = result
        {
            assert!(
                msg.contains("email:"),
                "Error message should contain the field path 'email'"
            );
            assert!(
                msg.contains("not a valid email"),
                "Error message should contain the validation error"
            );
        } else {
            panic!("Expected FieldValidatedFail error");
        }
    }
}
