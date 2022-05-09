use actix_web::{web::Json, Responder};

use impostro_shared::{api::CreateSession, session::Session};

pub async fn create_session(
    Json(CreateSession { pass }): Json<CreateSession>,
    data: crate::DataTy,
) -> impl Responder {
    let mut guard = data.lock().unwrap();
    match pass.as_str() {
        include_str!("PASS") => {
            let session = Session::new(&mut guard.session_counter);
            let json = Json(Some(session.id.clone()));
            guard.sessions.push(session);
            json
        },
        _ => Json(None),
    }
}
