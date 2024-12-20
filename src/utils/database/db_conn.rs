use mongodb::Client;

use crate::{error::db_error::DbResult, utils};

use super::user_crud::UserCRUD;

#[derive(Debug)]
pub struct ConnDb {
    pub user: UserCRUD,
}

impl ConnDb {
    pub async fn init() -> DbResult<Self> {
        let db_uri = (*utils::constants::DbUrl).clone();
        let client = Client::with_uri_str(db_uri).await.unwrap();
        let db = client.database("actix_web");

        Ok(Self {
            user: UserCRUD {
                user: db.collection("users"),
            },
        })
    }
}
