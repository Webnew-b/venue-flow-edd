use std::path::Path;

use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::VenueStatus;
use serde::{Deserialize, Serialize};


pub struct CreateVenueCommand<'image> {
    pub user_id:i64,
    pub name:String,
    pub address:String,
    pub images:Vec<VenueImageCommand<'image>>,
    pub capacity:i32,
    pub description:Option<String>,
}

pub struct VenueImageCommand<'image>{
    pub title:String,
    pub image:&'image Path,
    pub comment:Option<String>,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct VenueImageRes {
    pub title:String,
    pub uri:String,
    pub comment:Option<String>,
}

impl From<VenueImage> for VenueImageRes {
    fn from(value: VenueImage) -> Self {
        Self {
            title:value.title,
            uri:value.uri,
            comment:value.comment
        }
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub struct CreateVenueRes {
    pub id:i64,
    pub name:String,
    pub address:String,
    pub images:Vec<VenueImageRes>,
    pub capacity:i32,
    pub description:Option<String>,
}

#[derive(Serialize,Deserialize,PartialEq, Eq,Clone,Debug)]
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

#[derive(Serialize,Deserialize,Clone)]
pub struct ManageVenueRes {
    pub id:i64,
    pub status:VenueStatusRes
}


pub struct UpdateVenueCommand<'image> {
    pub id:i64, 
    pub name:Option<String>,
    pub address:Option<String>,
    pub images:Vec<VenueImageCommand<'image>>,
    pub capacity:Option<i32>,
    pub description:Option<String>,
}
