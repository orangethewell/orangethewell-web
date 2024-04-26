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
                    .table(PostMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostMetadata::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostMetadata::Title).string().not_null())
                    .col(ColumnDef::new(PostMetadata::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(PostMetadata::ShortDesc).string())
                    .col(ColumnDef::new(PostMetadata::WriterId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-post_user_foreign_key")
                            .from(PostMetadata::Table, PostMetadata::WriterId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(PostMetadata::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PostMetadata::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PostMetadata::ContentPath).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(PostMetadata::Table)
                    .drop_foreign_key(Alias::new("FK-post_user_foreign_key"))
                    .to_owned()
            )
        .await?;

        manager
            .drop_table(Table::drop().table(PostMetadata::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
enum PostMetadata {
    Table,
    Id,
    Title,
    Slug,
    ShortDesc,
    WriterId,
    CreatedAt,
    UpdatedAt,
    ContentPath,
}
