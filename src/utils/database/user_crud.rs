use mongodb::{results::InsertOneResult, Collection};

use crate::error::db_error::{DbErr, DbResult};

use super::schema::user_schema::UserSchema;

#[derive(Debug)]
pub struct UserCRUD {
    pub user: Collection<UserSchema>,
}

impl UserCRUD {
    pub async fn new(&self, user: UserSchema) -> DbResult<InsertOneResult> {
        match self.user.insert_one(UserSchema::new(user)).await {
            Ok(r) => Ok(r),
            Err(e) => Err(DbErr::CanNotDo {
                error: e.to_string(),
                collection: "User collection".to_string(),
                action: "create".to_string(),
            }),
        }
    }
}
