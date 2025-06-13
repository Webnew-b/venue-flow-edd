use serde::{Deserialize, Serialize};

use crate::app_event::AppEventList;

pub mod use_case;
pub mod querys;
pub mod commands;
pub mod app_error;
pub mod app_event;

#[derive(Debug)]
pub enum AppUseCase {
    UserRegistrantion,
    UserLogin,
    BasicUserProfile,
    CreateVenue,
    ManageVenueStatus,
    ViewMyVenues,
    EditVenue,
    ViewRentalRequests,
    ProcessRentalRequests,

    GetData(String),
}

#[derive(Debug)]
pub struct Outcome<T> {
    pub data:T,
    pub from_case:AppUseCase,
    pub events:Vec<AppEventList>
}

// todo move all code to the "web" lib which about CustomResponse.
#[derive(Serialize)]
pub struct CustomResponse<T: Serialize + for<'de> Deserialize<'de>> {
    pub message: String,
    pub code: u16,
    pub body: Option<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de>> CustomResponse<T> {
    pub fn new(msg:&str,code:CodeEnum,res:Option<T>)-> Self{
        Self{
            code:get_code(code),
            message: msg.to_string(),
            body:res
        }
    }
    pub fn success(resp: Option<T>) -> Self {
        Self {
            code: get_code(CodeEnum::Success),
            message: "Success".to_string(),
            body: resp,
        }
    }

}

#[derive(Debug,Clone)]
pub enum CodeEnum {
    Success,

    NotFound,
    FileInvaild,
    ServiceError,
    Other,

    Ban,
    Unauthorized,
}

pub fn get_code(code: CodeEnum) -> u16 {
    match code {
        CodeEnum::Success => 0,
        CodeEnum::Other => 1,
        CodeEnum::ServiceError => 2,
        CodeEnum::NotFound => 3,

        CodeEnum::FileInvaild=>1000,

        CodeEnum::Ban => 2000,
        CodeEnum::Unauthorized => 2001,
    }
}
