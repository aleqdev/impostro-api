use actix_web::{web::Json, Responder};

use impostro_shared::api::ValidateSessionId;

pub async fn validate_session_id(
    Json(ValidateSessionId {
        session
    }): Json<ValidateSessionId>,
    data: crate::DataTy,
) -> impl Responder {
    let guard = data.lock().unwrap();
    match guard.get_session(session) {
        Some(_) => Json(true),
        None => Json(false),
    }
}
