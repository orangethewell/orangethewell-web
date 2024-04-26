use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notification::RecipientId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-notification_user_foreign_key")
                            .from(Notification::Table, Notification::RecipientId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Notification::Title).string().not_null())
                    .col(ColumnDef::new(Notification::Description).string())
                    .col(ColumnDef::new(Notification::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Notification::Read).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Notification::Table)
                    .drop_foreign_key(Alias::new("FK-notification_user_foreign_key"))
                    .to_owned()
            )
        .await?;

        manager
            .drop_table(Table::drop().table(Notification::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
enum Notification {
    Table,
    Id,
    RecipientId,
    Title,
    Description,
    CreatedAt,
    Read,
}
