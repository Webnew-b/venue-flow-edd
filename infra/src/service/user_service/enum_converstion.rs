use domain_core::user::{UserGender as CoreUserGender, UserStatus as CoreUserStatus};

use crate::database::entities::sea_orm_active_enums::{UserGender, UserStatus};

pub(crate) fn user_status_to_db(value:CoreUserStatus) -> UserStatus {
    match value {
        CoreUserStatus::Active => UserStatus::Active,
        CoreUserStatus::Ban => UserStatus::Ban,
    }
}

pub(crate) fn user_status_to_domain(value:UserStatus) -> CoreUserStatus {
    match value {
        UserStatus::Active => CoreUserStatus::Active,
        UserStatus::Ban => CoreUserStatus::Ban,
    }
}


pub(crate) fn user_gender_to_db(value:CoreUserGender) -> UserGender {
    match value {
        CoreUserGender::Male => UserGender::Male,
        CoreUserGender::Female => UserGender::Female,
        CoreUserGender::Nonbinary => UserGender::Nonbinary,
        CoreUserGender::PreferNotToSay => UserGender::PreferNotToSay,
    }
}


pub(crate) fn user_gender_to_domain(value:UserGender) -> CoreUserGender {
    match value {
        UserGender::Male => CoreUserGender::Male,
        UserGender::Female => CoreUserGender::Female,
        UserGender::Nonbinary => CoreUserGender::Nonbinary,
        UserGender::PreferNotToSay => CoreUserGender::PreferNotToSay,
    }
}
