use actix_web::{web::Json, Responder};

use impostro_shared::api::GetMembers;

pub async fn get_members(
    Json(GetMembers {
        session,
        group_name,
    }): Json<GetMembers>,
    data: crate::DataTy,
) -> impl Responder {
    let guard = data.lock().unwrap();
    match guard.get_session(session) {
        Some(session) => match session.get_group(group_name) {
            Some(group) => Json(Some(group.clone())),
            None => Json(None),
        },
        None => Json(None),
    }
}
