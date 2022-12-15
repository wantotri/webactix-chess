mod lobby;
mod message;
mod model;
mod webserver;
mod ws;
pub mod game;
pub mod error;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("SERVER_PORT").unwrap_or_else(|_| "7878".to_string()).parse().unwrap();

    // let tera = Tera::new("/home/ubuntu/webactix/templates/**/*").unwrap();
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let chess_ws_server = lobby::Lobby::default().start();

    println!("Web Actix server start on {}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(webserver::index)
            .service(webserver::game)
            .service(webserver::staticfiles)
            .app_data(web::Data::new(chess_ws_server.clone()))
            .service(ws::start_connection)
    })
    .bind((host, port))?
    .run()
    .await
}
