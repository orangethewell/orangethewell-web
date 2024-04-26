use sea_orm_migration::{prelude::*, sea_orm::{ColumnTrait, EntityTrait, QueryFilter, SelectColumns}};

use crate::m20240404_125849_create_role_permissions_table::{Role, RolePermissions};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert_role = Query::insert()
            .into_table(Role::Table)
            .columns([Role::Name, Role::Description])
            .values_panic([
                "Administrador".into(),
                "Esse cargo padrão possui todas as permissões necessárias para gerenciar os recursos e conteúdos do site. É necessário cuidado pois a atribuição indevida desse cargo pode colocar o site em perigo.".into()])
            .to_owned();
        
        use entities::prelude::Role as RoleTable;
        use entities::role::Model as RoleModel;
        use entities::role;

        manager.exec_stmt(insert_role.to_owned()).await?;

        use entities::prelude::Permission as PermissionTable;
        use entities::permission::Model as PermissionModel;

        let admin: RoleModel = RoleTable::find().filter(role::Column::Name.eq("Administrador")).one(manager.get_connection()).await.unwrap().unwrap();
        let permissions: Vec<PermissionModel> = PermissionTable::find().all(manager.get_connection()).await.unwrap();

        let mut insert_permission_role = Query::insert();
        insert_permission_role.into_table(RolePermissions::Table);
        insert_permission_role.columns([RolePermissions::RoleId, RolePermissions::PermissionId]);

        for permission in permissions.iter() {
            insert_permission_role.values_panic([admin.id.into(), permission.id.into()]);
        }
        
        manager.exec_stmt(insert_permission_role.to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Role::Table)
            .cond_where(Expr::col(Role::Name).eq("Administrador"))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}