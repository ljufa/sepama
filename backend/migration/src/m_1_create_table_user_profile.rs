use entity::{sea_orm::Statement, user_profile::*};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_1_create_table_user_profile"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "create table user_profile
        (
            id                 integer GENERATED BY DEFAULT AS IDENTITY not null primary key,
            auth_id            text                                     not null unique,
            date_created       timestamp                                not null default current_timestamp,
            firstname          text                                     not null,
            lastname           text                                     not null,
            address            jsonb,
            preferred_language text,
            date_of_birth      date,
            status             text check (status in ('PROFILE_INCOMPLETE', 'PROFILE_COMPLETE')) default 'PROFILE_INCOMPLETE'
        )";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
