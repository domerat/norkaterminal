pub use sea_orm_migration::prelude::*;

pub mod m20261006_000001_users;
pub mod m20261006_000002_scheduled;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261006_000001_users::Migration),
            Box::new(m20261006_000002_scheduled::Migration)
        ] 
    }
}
