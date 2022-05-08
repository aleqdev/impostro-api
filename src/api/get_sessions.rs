use actix_web::{web::Json, Responder};
use impostro_shared::api::GetSessions;

pub async fn get_sessions(
    Json(GetSessions { pass }): Json<GetSessions>,
    data: crate::DataTy,
) -> impl Responder {
    let guard = data.lock().unwrap();
    match pass.as_str() {
        include_str!("PASS") => Json(Some(guard.sessions.clone())),
        _ => Json(None),
    }
}
