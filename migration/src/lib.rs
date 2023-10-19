pub use sea_orm_migration::prelude::*;

mod m20231019_072332_add_agreement_table;
mod m20231019_073111_add_agreement_versions_table;
mod m20231019_082748_add_generate_agreement_acceptance_statuses;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231019_072332_add_agreement_table::Migration),
            Box::new(m20231019_073111_add_agreement_versions_table::Migration),
            Box::new(m20231019_082748_add_generate_agreement_acceptance_statuses::Migration),
        ]
    }
}
