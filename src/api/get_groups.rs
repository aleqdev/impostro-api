use actix_web::{
    web::Json, Responder,
};

use impostro_shared::session::SessionId;

pub async fn get_groups(Json(session): Json<SessionId>, data: crate::DataTy) -> impl Responder {
    let guard = data.lock().unwrap();  
    match guard.get_session(session) {
        Some(session) => Json(Some(session.groups.clone())),
        None => Json(None)
    }
}
