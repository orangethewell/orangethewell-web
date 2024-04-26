pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240404_125849_create_role_permissions_table;
mod m20240404_130258_seed_permissions;
mod m20240404_130811_seed_admin_role;
mod m20240404_131352_create_posts_table;
mod m20240404_150409_create_images_table;
mod m20240406_123952_create_notification_table;
mod m20240412_023852_seed_admin_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240404_125849_create_role_permissions_table::Migration),
            Box::new(m20240404_130258_seed_permissions::Migration),
            Box::new(m20240404_130811_seed_admin_role::Migration),
            Box::new(m20240404_131352_create_posts_table::Migration),
            Box::new(m20240404_150409_create_images_table::Migration),
            Box::new(m20240406_123952_create_notification_table::Migration),
            Box::new(m20240412_023852_seed_admin_user::Migration),
        ]
    }
}
