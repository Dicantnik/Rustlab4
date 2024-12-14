use rocket::serde::json::Json;
use crate::dtos::responses::MessageOnlyResponse;

#[catch(401)]
pub fn unauthorized() -> Json<MessageOnlyResponse> {
    Json(MessageOnlyResponse {
        message: "User unauthorized!".to_string(),
    })
}