use std::fmt::Display;

use serde::{Deserialize, Serialize};
use util_macros::IteralDisplay;

use crate::app_event::AppEventList;

pub mod use_case;
pub mod querys;
pub mod commands;
pub mod app_error;
pub mod app_event;

#[derive(Debug,Clone,PartialEq, Eq, IteralDisplay)]
pub enum AppUseCase {
    UserRegistrantion,
    UserLogin,
    BasicUserProfile,
    
    CreateVenue,
    ManageVenueStatus,
    ViewMyVenues,
    EditVenue,
    
    CreateRentalRequest,
    ViewRentalRequests,
    ProcessRentalRequests,
    CancelRentalRequest,
    UpdateRentalTime,
    

    GetData(String),
}

impl Display for AppUseCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetData(s) => write!(f,"get {} data",s),
            _ => write!(f,"{}",self.iteral_display())
        }
    }
}

#[derive(Debug,Clone)]
pub struct Outcome<T> {
    pub data:T,
    pub from_case:AppUseCase,
    pub events:AppEventList
}

impl<T> Outcome<T> {
    pub fn new(data:T,from_case:AppUseCase) -> Self {
        Self { data, from_case, events: AppEventList::new() }
    }

    pub fn new_with_events(
        data:T,
        from_case:AppUseCase,
        events:AppEventList
    ) -> Self {
        Self { data, from_case, events }
    }
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
