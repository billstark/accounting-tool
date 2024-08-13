use entity::transaction_type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

use crate::m20231206_000001_create_txn_type::TransactionType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _db = manager.get_connection();


        transaction_type::ActiveModel { 
            id: Set(0),
            type_name: Set("dining".to_owned()),
            display_text: Set("Dining".to_owned()),
            desc: Set(Some("Breakfast, lunch, dinner and all dining related.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        transaction_type::ActiveModel { 
            id: Set(1),
            type_name: Set("shopping".to_owned()),
            display_text: Set("Shopping".to_owned()),
            desc: Set(Some("Buy things, either online or offline.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        transaction_type::ActiveModel { 
            id: Set(2),
            type_name: Set("grocery".to_owned()),
            display_text: Set("Grocery".to_owned()),
            desc: Set(Some("Supermarkets and other daily spends online.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        transaction_type::ActiveModel { 
            id: Set(3),
            type_name: Set("travel".to_owned()),
            display_text: Set("Travel".to_owned()),
            desc: Set(Some("Traveling.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        transaction_type::ActiveModel { 
            id: Set(4),
            type_name: Set("entertainment".to_owned()),
            display_text: Set("Entertainment".to_owned()),
            desc: Set(Some("Buy games, services, and others.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        transaction_type::ActiveModel { 
            id: Set(5),
            type_name: Set("transport".to_owned()),
            display_text: Set("Transportation".to_owned()),
            desc: Set(Some("Taxi, public transportation.".to_owned())),
            status: Set(true),
            created_at: Default::default(),
            updated_at: Default::default()
        }.insert(_db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let drop = Query::delete()
            .from_table(TransactionType::Table)
            .cond_where(
                Cond::all()
                .add(Expr::col(TransactionType::Id).gte(1))
                .add(Expr::col(TransactionType::Id).lte(6))
            ).to_owned();
        
        manager.exec_stmt(drop).await
    }
}

