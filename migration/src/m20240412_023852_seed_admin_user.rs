use sea_orm_migration::{prelude::*, sea_orm::*};

use std::env;

use crate::m20220101_000001_create_table::User;
use crate::m20240404_125849_create_role_permissions_table::UserRoles;
use chrono::{Utc, FixedOffset};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        dotenv::dotenv().ok();

        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHasher,
                SaltString
            },
            Argon2
        };

        let secret = env::var("SECRET_KEY").unwrap();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new_with_secret(
            secret.as_bytes(), 
            argon2::Algorithm::default(),
            argon2::Version::default(),
            argon2::Params::default(),
        ).unwrap();

        let insert_user = Query::insert()
            .into_table(User::Table)
            .columns([User::Username, User::Email, User::Password, User::CreatedAt, User::UpdatedAt])
            .values_panic([
                "Administrador".into(),
                "admin@localhost".into(),
                argon2.hash_password("admin".as_bytes(), &salt).unwrap().to_string().into(),
                Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")).into(),
                Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")).into()])
            .to_owned();
        
        use entities::prelude::User as UserTable;
        use entities::prelude::Role as RoleTable;
        use entities::role::Model as RoleModel;
        use entities::user::Model as UserModel;
        use entities::user;
        use entities::role;

        manager.exec_stmt(insert_user.to_owned()).await?;

        let admin: UserModel = UserTable::find().filter(user::Column::Username.eq("Administrador")).one(manager.get_connection()).await.unwrap().unwrap();
        let admin_role: RoleModel = RoleTable::find().filter(role::Column::Name.eq("Administrador")).one(manager.get_connection()).await.unwrap().unwrap();

        let mut insert_user_role = Query::insert();
        insert_user_role.into_table(UserRoles::Table);
        insert_user_role.columns([UserRoles::RoleId, UserRoles::UserId]);
        insert_user_role.values_panic([admin_role.id.into(), admin.id.into()]);
        
        
        manager.exec_stmt(insert_user_role.to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(User::Table)
            .cond_where(Expr::col(User::Username).eq("Administrador"))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
