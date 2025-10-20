use actix_web::{web, Scope};

pub mod login;
pub mod logout;
pub mod register;
pub mod register_lessor;
pub mod register_organizer;

pub fn index() -> Scope {
    web::scope("/user")
        .service(self::login::login)
        .service(self::register::register)
}
