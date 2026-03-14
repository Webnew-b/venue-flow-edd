use std::path::PathBuf;

use fake::faker::address::en::{SecondaryAddress, StreetName};
use fake::faker::currency::en::CurrencyName;
use fake::faker::filesystem::en::FilePath;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;

pub mod rental_common;
pub mod user_common;
pub mod util_common;
pub mod venue_common;

pub fn fake_number() -> i64 {
    (0..10000000).fake()
}

pub fn fake_number_i32() -> i32 {
    (0..49999).fake()
}

pub fn fake_password() -> String {
    Password(6..30).fake::<String>()
}

pub fn fake_password_with_range(range: std::ops::Range<usize>) -> String {
    Password(range).fake::<String>()
}

pub fn fake_username() -> String {
    let mut name: String;
    loop {
        name = Username().fake::<String>();
        if name.chars().count() > 5 && name.chars().count() < 30 {
            return name;
        }
    }
}

pub fn fake_email() -> String {
    FreeEmail().fake::<String>()
}

pub fn fake_name() -> String {
    CurrencyName().fake::<String>()
}

pub fn fake_address() -> String {
    format!(
        "{} {}",
        StreetName().fake::<String>(),
        SecondaryAddress().fake::<String>()
    )
}

pub fn fake_file_path() -> PathBuf {
    FilePath().fake::<PathBuf>()
}
