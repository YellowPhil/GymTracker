use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Schema},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_table(schema.create_table_from_entity(entity::user::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(entity::training::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
        // manager
        // .drop_table(Table::drop().table(Post::Table).to_owned())
        // .await
    }
}
