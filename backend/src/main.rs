use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::web::{get, head, post, scope};
use actix_web::{guard, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
// use migration::Migrator;
use migration::{Migrator, MigratorTrait};

use std::env;

use dotenv::dotenv;
use entity::sea_orm;

use backend::{auth, handlers, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let address =
        std::env::var("SERVER_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let allowed_cors_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:8000".into());

    let conn = sea_orm::Database::connect(&database_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { connection: conn };
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        let cors = Cors::default()
            .allowed_origin(allowed_cors_origin.as_str())
            .allow_any_header()
            .allow_any_method();
        App::new()
            .data(state.clone())
            .wrap(auth)
            .wrap(cors)
            .service(
                scope("/api")
                    .service(
                        scope("/profile")
                            .guard(guard::Header("content-type", "application/json"))
                            .route("", head().to(handlers::profile::profile_exists))
                            .route("", post().to(handlers::profile::set_user_profile))
                            .route("", get().to(handlers::profile::get_user_profile)),
                    )
                    .service(
                        scope("/mandates")
                            .route(
                                "",
                                get()
                                    .guard(guard::Header("content-type", "application/json"))
                                    .to(handlers::mandate::get_mandates),
                            )
                            .route(
                                "",
                                post()
                                    .guard(guard::Header(
                                        "content-type",
                                        "application/json; charset=utf-8",
                                    ))
                                    .to(handlers::mandate::save_mandate),
                            ),
                    ),
            )
    })
    .bind(address)?
    .run()
    .await
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    match auth::get_token_data(credentials.token()).await {
        Ok(_) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
