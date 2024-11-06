use std::fmt;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateGroupRequest {
    pub group_owner_id: uuid::Uuid,
    #[validate(length(min = 2))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Group {
    pub group_id: uuid::Uuid,
    pub group_owner_id: uuid::Uuid,
    pub name: String,
}

impl From<db::models::Group> for Group {
    fn from(value: db::models::Group) -> Self {
        Self {
            group_id: value.id,
            group_owner_id: value.created_by_user,
            name: value.name,
        }
    }
}

pub struct ApproveJoin {
    pub candidate_id: uuid::Uuid,
    pub approved: bool,
    pub group_id: uuid::Uuid,
}
