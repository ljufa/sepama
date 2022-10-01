pub use sea_schema::migration::prelude::*;

mod m_1_create_table_user_profile;
mod m_2_create_table_mandate;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_1_create_table_user_profile::Migration),
            Box::new(m_2_create_table_mandate::Migration),
        ]
    }
}
