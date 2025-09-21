use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QrCode::Table)
                    .if_not_exists()
                    .col(pk_uuid(QrCode::Id))
                    .col(string_len(QrCode::Link, 512))
                    .col(string_len(QrCode::Passphrase, 255))
                    .col(timestamp(QrCode::CreatedAt))
                    .col(timestamp_null(QrCode::ModifiedAt  ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_table(Table::drop().table(QrCode::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum QrCode {
    Table,
    Id,
    Link,
    Passphrase,
    CreatedAt,
    ModifiedAt,
}
