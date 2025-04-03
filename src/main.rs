use actix_web::{web, HttpServer, App, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(|| async {"hello world"}))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}