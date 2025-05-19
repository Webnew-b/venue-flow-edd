pub trait UserValidation {
    fn valid_email(&self,email:&str) -> bool;
    fn valid_username(&self,username:&str) -> bool;
}
