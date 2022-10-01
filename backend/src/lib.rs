use entity::sea_orm::DatabaseConnection;

pub mod auth;
pub mod errors;
pub mod handlers;

#[derive(Debug, Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}
