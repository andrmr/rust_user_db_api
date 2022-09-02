use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    Salt,
    Data,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Username).primary_key().string().not_null())
                    .col(ColumnDef::new(User::Id).integer().auto_increment().not_null())
                    .col(ColumnDef::new(User::Password).text().not_null())
                    .col(ColumnDef::new(User::Salt).text().not_null())
                    .col(ColumnDef::new(User::Data).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
