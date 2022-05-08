use actix_web::{
    web::Json, Responder,
};

pub async fn get_sessions(Json(pass): Json<String>, data: crate::DataTy) -> impl Responder {
    let guard = data.lock().unwrap();  
    match pass.as_str() {
        include_str!("PASS") => Json(Some(guard.sessions.clone())),
        _ => Json(None),
    }
}
