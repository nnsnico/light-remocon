use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use light_remocon::service::health_check_service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route(
                "/hc",
                web::get().to(|| health_check_service::health_check()),
            )
            .route(
                "/hc/echo",
                web::get().to(|req| health_check_service::echo(req)),
            )
            .route("/fuga", web::post().to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:41410")?
    .run()
    .await
}
