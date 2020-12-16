use actix_web::{middleware::Logger, App, HttpServer};
#[cfg(feature = "ui")]
use actix_web_static_files;

#[cfg(feature = "ui")]
use angular_example_frontend::generate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        let mut app = App::new().wrap(Logger::default());
        #[cfg(feature = "ui")]
        {
            let generated = generate();
            app = app.service(
                actix_web_static_files::ResourceFiles::new("/", generated)
                    .resolve_not_found_to_root(),
            );
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
