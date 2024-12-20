use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

impl UserSchema {
    pub fn new(user: UserSchema) -> Self {
        UserSchema {
            id: None,
            name: user.name,
            email: user.email,
        }
    }
}
