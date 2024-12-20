use std::sync::Arc;

use super::database::db_conn::ConnDb;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<ConnDb>,
}
