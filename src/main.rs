use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

pub mod api;

pub type DataTy = Data<Mutex<impostro_shared::ImpostroData>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(Mutex::new(
        impostro_shared::ImpostroData::default(),
    ));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("sessions", web::post().to(api::get_sessions_fn))
            .route("groups", web::post().to(api::get_groups_fn))
            .route("members", web::post().to(api::get_members_fn))
            .route("validate_session_id", web::post().to(api::validate_session_id_fn))
            .route("create_session", web::post().to(api::create_session_fn))
    })
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
