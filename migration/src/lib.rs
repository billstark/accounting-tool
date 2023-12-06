pub use sea_orm_migration::prelude::*;

mod m20231206_000001_create_txn_type;
mod m20231206_105605_create_txn;
mod m20231206_111026_seed_txn_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231206_000001_create_txn_type::Migration),
            Box::new(m20231206_105605_create_txn::Migration),
            Box::new(m20231206_111026_seed_txn_type::Migration),
        ]
    }
}
