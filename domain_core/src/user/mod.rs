macro_rules! require_field {
    ($field:expr,$name:expr) => {
        if $field.is_none() {
            return Err(DomainCoreError::MissingField($name.to_string()));
        }
    };
}

macro_rules! field_fill {
    ($target:expr,$source:expr,$($field:ident),+) => {
        $(
            if let Some(item) = $source.$field {
                $target.$field = item;
            }
        )+
    };
}

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use garde::Validate;

use crate::domain_core_error::{DomainCoreError, DomainCoreResult};
use crate::user::user_update::UserUpdate;

pub mod lessor;
pub mod organizer;
pub mod user_update;
pub mod user_error;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum UserGender {
    Male,
    Female,
    Nonbinary,
    PreferNotToSay,
}

impl UserGender {
    pub fn get_gender(gender:&str) -> Option<Self>{
        match gender {
            "male" => Some(Self::Male),
            "female" => Some(Self::Female),
            "non-binary" => Some(Self::Nonbinary),
            "prefer-not-to-say" => Some(Self::PreferNotToSay),
            _ => None,
        }
    }
    
}

impl std::fmt::Display for UserGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserGender::Male => write!(f,"male"),
            UserGender::Female => write!(f,"female"),
            UserGender::Nonbinary => write!(f,"non-binary"),
            UserGender::PreferNotToSay => write!(f,"prefer-not-to-say"),
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum UserStatus {
    Active,
    Ban,
}

#[derive(Debug,Clone,Builder,PartialEq,Eq,Validate)]
#[builder(
    pattern = "owned",
    build_fn(
        validate = "Self::validate_builder",
        name = "build",
        error = "DomainCoreError"
    )
)]
pub struct User {

    #[builder(default)]
    #[garde(skip)]
    id:Option<i64>,

    #[garde(length(min=5,max=200))]
    username:String,
    #[garde(email)]
    email:String,
    #[garde(url)]
    avatar:String,
    #[garde(skip)]
    gender:UserGender,
    #[garde(length(min=8,max=50))]
    password:String,

    #[garde(skip)]
    #[builder(default)]
    introduce:Option<String>,

    #[garde(skip)]
    #[builder(default = "true")]
    is_show:bool,

    #[garde(skip)]
    #[builder(default = "false")]
    is_delete:bool,

    #[garde(skip)]
    #[builder(default = "UserStatus::Active")]
    status:UserStatus,

    #[garde(skip)]
    createtime:DateTime<Utc>,
    #[garde(skip)]
    updatetime:DateTime<Utc>,
}

impl User {
    pub fn update_email(
        mut self,
        new_email:String,
        updatetime:DateTime<Utc>
    ) -> Self {
        self.email = new_email;
        self.updatetime = updatetime;
        self
    }

    pub fn ban_user(mut self,updatetime:DateTime<Utc>) -> Self {
        self.updatetime = updatetime;
        self.status = UserStatus::Ban;
        self
    }

    pub fn delete_user(mut self,updatetime:DateTime<Utc>) -> Self {
        self.updatetime = updatetime;
        self.is_delete = true;
        self
    }

    pub fn update_gender(
        mut self,
        gender:UserGender,
        updatetime:DateTime<Utc>
    ) -> Self {
        self.gender = gender;
        self.updatetime = updatetime;
        self
    }

    pub fn update_user(
        mut self,
        update:UserUpdate,
    ) -> DomainCoreResult<Self>  {
        update.is_vaild_update_command()?;
       
        field_fill!(
            self,
            update,
            username,
            email,
            gender,
            avatar,
            password
        );

        self.introduce = update.introduce;
        Ok(self)
    }

    
}

impl User {

    pub fn can_login(&self) -> bool {
        !self.is_delete &&
        self.status == UserStatus::Active
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn avatar(&self) -> &str {
        &self.avatar
    }

    pub fn gender(&self) -> &UserGender {
        &self.gender
    }

    pub fn introduce(&self) -> Option<&String> {
        self.introduce.as_ref()
    }

    pub fn is_show(&self) -> bool {
        self.is_show
    }

    pub fn is_delete(&self) -> bool {
        self.is_delete
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn createtime(&self) -> DateTime<Utc> {
        self.createtime
    }

    pub fn updatetime(&self) -> DateTime<Utc> {
        self.updatetime
    }
}

impl UserBuilder {
    fn validate_builder(&self) -> DomainCoreResult<()> { 
        require_field!(self.username,"username");
        require_field!(self.email,"email");
        require_field!(self.avatar,"avatar");
        require_field!(self.gender,"gender");
        require_field!(self.createtime,"createtime");
        require_field!(self.updatetime,"updatetime");

        //todo Need to validate field is valid after build.
        Ok(())
    }
}
