use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Permission::Name).string().not_null())
                    .col(ColumnDef::new(Permission::Description).string())
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Role::Name).string().not_null())
                    .col(ColumnDef::new(Role::Description).string())
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(RolePermissions::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-role_permission_foreign_key")
                            .from(RolePermissions::Table, RolePermissions::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(RolePermissions::PermissionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-permission_role_foreign_key")
                            .from(RolePermissions::Table, RolePermissions::PermissionId)
                            .to(Permission::Table, Permission::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            ).await?;

            manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(UserRoles::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-user_role_foreign_key")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(UserRoles::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-role_user_foreign_key")
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(RolePermissions::Table)
                    .drop_foreign_key(Alias::new("FK-role_permission_foreign_key"))
                    .drop_foreign_key(Alias::new("FK-permission_role_foreign_key"))
                    .to_owned()
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserRoles::Table)
                    .drop_foreign_key(Alias::new("FK-user_role_foreign_key"))
                    .drop_foreign_key(Alias::new("FK-role_user_foreign_key"))
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
        
    }
}

#[derive(DeriveIden)]
pub enum Permission {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
pub enum RolePermissions {
    Table,
    Id,
    RoleId,
    PermissionId,
}

#[derive(DeriveIden)]
pub enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
}