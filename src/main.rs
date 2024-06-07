use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid u16");
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    println!("Server running at http://{address}:{port}");

    HttpServer::new(|| App::new().service(hello))
        .bind((address, port))?
        .run()
        .await
}
