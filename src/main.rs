use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local};
use env_logger;
use std::env;
use std::io::Write;
use log::{info, LevelFilter};

async fn redirect() -> impl Responder {
    info!("redirecting...");


    let redir = env::var("REDIR").unwrap_or_else(|_| "https://example.com".to_string());
    HttpResponse::MovedPermanently()
        .insert_header(("Location", redir))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    info!("Main started");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(redirect))
            // You can add more routes here if needed
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

