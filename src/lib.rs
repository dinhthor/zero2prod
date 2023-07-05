//! lib.rs

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World"); 
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// // We need to mark `run` as public.
// // It is no longer a binary entrypoint, 
// // so we don't need #[actix_web::main] anymore.
// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| { 
//         App::new()
//             // .route("/", web::get().to(greet)) 
//             // .route("/{name}", web::get().to(greet))
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword 
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { App::new()
    .route("/health_check", web::get().to(health_check))
            })
            .listen(listener)?
            .run();
        // No .await here!
    Ok(server) 
}