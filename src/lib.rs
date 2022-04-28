use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde::{Deserialize};
async fn health_check() -> HttpResponse{
    HttpResponse::Ok().finish()
}



#[derive(Deserialize)]
    struct UserData{
        name: String,
        email: String, 
    }
async fn subscribe(_form: web::Form<UserData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}