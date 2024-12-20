use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestModel {
    pub message: String,
}
