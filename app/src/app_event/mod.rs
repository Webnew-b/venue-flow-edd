use std::fmt::Display;

use domain::event_trait::EventExecutionMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEventList {
    elements: Vec<AppEvent>,
}

impl AppEventList {
    pub fn new() -> Self {
        Self {
            elements: vec![AppEvent::LogUseCase],
        }
    }
    pub fn custom_new(first: AppEvent) -> Self {
        Self {
            elements: vec![first],
        }
    }

    pub fn concat(self, new_vec: Vec<AppEvent>) -> Self {
        Self {
            elements: [self.elements.as_slice(), new_vec.as_slice()].concat(),
        }
    }

    pub fn push(&mut self, item: AppEvent) {
        self.elements.push(item);
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get_elements(self) -> Vec<AppEvent> {
        self.elements
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AppEvent {
    LogUseCase,

    CanceledRentalRequest {
        organizer_email: String,
        organizer_name: String,
        organizer_id: i64,
        lessor_id: i64,
        lessor_name: String,
        lessor_email: String,
    },

    ApprovedRentalRequest {
        organizer_email: String,
        organizer_name: String,
        organizer_id: i64,
    },

    RejectedRentalRequest {
        organizer_email: String,
        organizer_name: String,
        organizer_id: i64,
    },
}

impl AppEvent {
    pub fn execution_mode(&self) -> EventExecutionMode {
        match self {
            Self::LogUseCase => EventExecutionMode::Immediate,
            Self::CanceledRentalRequest { .. }
            | Self::ApprovedRentalRequest { .. }
            | Self::RejectedRentalRequest { .. } => EventExecutionMode::Async,
        }
    }
}
