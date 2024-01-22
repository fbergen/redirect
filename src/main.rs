use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local};
use env_logger;
use std::io::Write;
use log::{info, LevelFilter};

async fn redirect() -> impl Responder {
    info!("redirecting...");
    HttpResponse::MovedPermanently()
        .insert_header(("Location", "https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2w2hCzte8YuK2dQyKyRB1LYUktZ8HFSIjHfadmCOrDpsO9nHCp-6LjOuwNBY2IJuiCGHdIbLHz"))
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

