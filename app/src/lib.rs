use std::fmt::Display;

use util_macros::IteralDisplay;

use crate::app_event::AppEventList;

pub mod app_error;
pub mod app_event;
pub mod commands;
pub mod querys;
pub mod use_case;

#[derive(Debug, Clone, PartialEq, Eq, IteralDisplay)]
pub enum AppUseCase {
    UserRegistrantion,
    UserLogin,
    BasicUserProfile,

    CreateVenue,
    ManageVenueStatus,
    ViewMyVenues,
    EditVenue,
    UploadVenueImage,
    DeleteVenueImage,

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
            Self::GetData(s) => write!(f, "get {} data", s),
            _ => write!(f, "{}", self.iteral_display()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Outcome<T> {
    pub data: T,
    pub from_case: AppUseCase,
    pub events: AppEventList,
}

impl<T> Outcome<T> {
    pub fn new(data: T, from_case: AppUseCase) -> Self {
        Self {
            data,
            from_case,
            events: AppEventList::new(),
        }
    }

    pub fn new_with_events(
        data: T,
        from_case: AppUseCase,
        events: AppEventList,
    ) -> Self {
        Self {
            data,
            from_case,
            events,
        }
    }
}
