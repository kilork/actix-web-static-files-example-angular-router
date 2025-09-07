use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_static_files::ResourceFiles;
use angular_example_frontend::generate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .wrap(Logger::default())
            .service(ResourceFiles::new("/", generated).resolve_not_found_to_root())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
