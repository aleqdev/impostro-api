use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

pub mod api;
pub mod data;

pub type DataTy = Data<Mutex<data::ImpostroData>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Mutex::new(data::ImpostroData::default())))
            .route(
                "sessions",
                web::get().to(api::get_sessions_fn),
            )
            .route(
                "groups",
                web::get().to(api::get_groups_fn),
            )
            .route(
                "members",
                web::get().to(api::get_members_fn),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
