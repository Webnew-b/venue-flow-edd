use std::path::PathBuf;

use domain::domain_error::{domain_venue_error::DomainVenueError, DomainError};
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::VenueStatus;
use serde::{Deserialize, Serialize};

pub struct CreateVenueCommand {
    pub user_id: i64,
    pub name: String,
    pub address: String,
    pub images: Vec<VenueImageCommand>,
    pub capacity: i32,
    pub description: String,
}

pub struct VenueImageCommand {
    pub title: String,
    pub image: String,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VenueImageRes {
    pub id: i64,
    pub title: String,
    pub uri: String,
    pub comment: Option<String>,
}

impl TryFrom<VenueImage> for VenueImageRes {
    type Error = DomainError;

    fn try_from(value: VenueImage) -> Result<Self, Self::Error> {
        let id = value.id.ok_or(DomainVenueError::ImageIdInexist)?;
        let res = Self {
            id,
            title: value.title,
            uri: value.uri,
            comment: value.comment,
        };
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateVenueRes {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub images: Vec<VenueImageRes>,
    pub capacity: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum VenueStatusRes {
    Published,
    UnPublished,
}
impl From<VenueStatus> for VenueStatusRes {
    fn from(value: VenueStatus) -> Self {
        match value {
            VenueStatus::Published => VenueStatusRes::Published,
            VenueStatus::Unpublished => VenueStatusRes::UnPublished,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ManageVenueRes {
    pub id: i64,
    pub status: VenueStatusRes,
}

pub struct UpdateVenueCommand {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub capacity: Option<i32>,
    pub description: Option<String>,
}

pub struct ImageUploadCommand {
    pub venue_id: i64,
    pub title: String,
    pub image: PathBuf,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageUploadRes {
    pub venue_id: i64,
    pub images: Vec<VenueImageRes>,
}

pub struct ImageDeleteCommand {
    pub image_id: Vec<i64>,
    pub venue_id: i64,
}
