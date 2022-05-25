use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(crate::ws::ImpostroWs {}, &req, stream);
    println!("{:#?}", &req);
    resp
}