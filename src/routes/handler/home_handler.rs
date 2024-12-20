use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};

use crate::{
    model::request_model::RequestModel,
    utils::{api_response, app_state::AppState, database::schema::user_schema::UserSchema},
};

#[get("/hello/{name}")]
pub async fn home_handler(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[post("/user")]
pub async fn home_test(state: Data<AppState>, user: Json<UserSchema>) -> impl Responder {
    match state.db.user.new(user.into_inner()).await {
        Err(e) => HttpResponse::BadRequest().json(RequestModel {
            message: e.to_string(),
        }),
        Ok(r) => HttpResponse::Created().json(r),
    }
}
