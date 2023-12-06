use sea_orm_migration::prelude::*;

use crate::m20231206_000001_create_txn_type::TransactionType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::TransactionType).integer())
                    .col(ColumnDef::new(Transaction::Amount).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Transaction::Location).text())
                    .col(ColumnDef::new(Transaction::TransactionTime).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Transaction::CreatedAt).timestamp_with_time_zone().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(Transaction::CreatedBy).string_len(16).not_null())
                    .foreign_key(ForeignKey::
                        create().name("fk-transaction-type_id")
                        .from(Transaction::Table, Transaction::TransactionType)
                        .to(TransactionType::Table, TransactionType::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Transaction {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "txn_type")]
    TransactionType,
    #[sea_orm(iden = "amount")]
    Amount,
    #[sea_orm(iden = "txn_time")]
    TransactionTime,
    #[sea_orm(iden = "location")]
    Location,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "created_by")]
    CreatedBy,
}

