use sea_orm_migration::prelude::*;

use crate::m20240404_125849_create_role_permissions_table::Permission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Permission::Table)
            .columns([Permission::Name, Permission::Description])
            .values_panic([
                "Escrever".into(),
                "Essa permissão concede ao usuário a habilidade de escrever, adicionar, remover e atualizar artigos publicados dentro do site.".into()])
                .values_panic([
                "Moderar".into(),
                "Essa permissão concede ao usuário a habilidade de moderar usuários, entregando permissões a outros usuários ou bloqueando interações destes com o site.".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Permission::Table)
            .cond_where(Expr::col(Permission::Name).eq("Escrever"))
            .cond_where(Expr::col(Permission::Name).eq("Moderar"))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}