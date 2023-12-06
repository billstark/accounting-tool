use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        manager
            .create_table(
                Table::create()
                .table(TransactionType::Table)
                .if_not_exists()
                .col(ColumnDef::new(TransactionType::Id).integer().not_null().primary_key().auto_increment())
                .col(ColumnDef::new(TransactionType::TypeName).string_len(32).not_null())
                .col(ColumnDef::new(TransactionType::DisplayText).string_len(64).not_null())
                .col(ColumnDef::new(TransactionType::Description).text())
                .col(ColumnDef::new(TransactionType::Status).boolean().not_null().default(SimpleExpr::Value(Value::Bool(Some(true)))))
                .col(ColumnDef::new(TransactionType::CreatedAt).timestamp_with_time_zone().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                .col(ColumnDef::new(TransactionType::UpdatedAt).timestamp_with_time_zone().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(TransactionType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TransactionType {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "type_name")]
    TypeName,
    #[sea_orm(iden = "desc")]
    Description,
    #[sea_orm(iden = "display_text")]
    DisplayText,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
    #[sea_orm(iden = "status")]
    Status,
}
