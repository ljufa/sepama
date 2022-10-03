use actix_web::dev::ServiceRequest;
use actix_web::web::{get, head, post, scope, Data};
use actix_web::{App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use migration::{Migrator, MigratorTrait};
use entity::sea_orm;
use std::env;

use backend::{auth, handlers, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let address =
        std::env::var("SERVER_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let static_dir = std::env::var("STATIC_FILES_DIR").unwrap_or_else(|_| "web_ui/web".to_string());
    let conn = sea_orm::Database::connect(&database_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { connection: conn };
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(Data::new(state.clone()))
            .service(
                scope("/api")
                    .wrap(auth)
                    .service(
                        scope("/profile")
                            .route("", head().to(handlers::profile::profile_exists))
                            .route("", post().to(handlers::profile::set_user_profile))
                            .route("", get().to(handlers::profile::get_user_profile)),
                    )
                    .service(
                        scope("/mandates")
                            .route("", get().to(handlers::mandate::get_mandates))
                            .route("", post().to(handlers::mandate::save_mandate)),
                    ),
            )
            .service(
                actix_files::Files::new("/", static_dir.as_str())
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(address)?
    .run()
    .await
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    match auth::get_token_data(credentials.token()).await {
        Ok(_) => Ok(req),
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}
