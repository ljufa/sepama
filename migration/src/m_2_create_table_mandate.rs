use entity::mandate::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, DbErr, Statement};


pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_2_create_table_mandate"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "create table mandate
        (
            id                     integer GENERATED BY DEFAULT AS IDENTITY not null primary key,
            api_id                 uuid                                     not null unique,
            user_profile_id        integer references user_profile (id)     not null,
            tags                   jsonb,
            status                 text                                     not null,
            unique_reference       text,
            display_name           text,
            date_created           timestamp                                not null default current_timestamp,
            creditor               jsonb                                    not null,
            bank_account           jsonb                                    not null,
            constraint status_check check (status IN ('ACTIVE', 'DELETED', 'CANCELED'))
        )";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).to_owned())
            .await
    }
}
