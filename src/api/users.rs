use leptos::*;
use std::sync::Arc;
#[cfg(feature = "ssr")]
use tokio::sync::Mutex;

#[cfg(feature = "ssr")]
use leptos_axum::extract;

#[cfg(feature = "ssr")]
use sea_orm::*;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::notifications::{push_notification, NotificationModel};
use super::roles_permissions::{get_role, PermissionModel, RoleModel};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[cfg(feature = "ssr")]
impl From<entities::user::Model> for UserModel {
    fn from(value: entities::user::Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            password: value.password,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

// User Create/Read/Update/Delete

/// Registers a new user to database, date objects from `UserModel` are ignored.
#[server(CreateUser, "/api/users")]
pub async fn create_user(new_user: UserModel) -> Result<UserModel, ServerFnError> {
    if let Some(user) = user_logged_in().await? {
        if user_have_permission(user, "Moderar".to_string()).await? {
            return create_user_guard(new_user).await;
        } else {
            return Err(ServerFnError::new(
                "User doesn't have the permission to execute this operation.",
            ));
        }
    } else {
        return Err(ServerFnError::new("User is not logged in."));
    }
}

#[cfg(feature = "ssr")]
pub async fn create_user_guard(new_user: UserModel) -> Result<UserModel, ServerFnError> {
    use crate::AppState;

    use entities::user;

    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let secret = state.secret_key.clone().into_bytes();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new_with_secret(
        secret.as_slice(),
        argon2::Algorithm::default(),
        argon2::Version::default(),
        argon2::Params::default(),
    )
    .unwrap();

    let password_hash = argon2
        .hash_password(&new_user.password.into_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user =
        user::ActiveModel {
            username: Set(new_user.username),
            email: Set(new_user.email),
            password: Set(password_hash),
            created_at: Set(Utc::now()
                .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
            updated_at: Set(Utc::now()
                .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
            ..Default::default()
        };

    let new_user = UserModel::from({
        match new_user.insert(&state.conn).await {
            Ok(user) => user,
            Err(db_err) => return Err(ServerFnError::new(format!(
                "A error happened while trying to register a new user, try again later. DbErr: {}",
                db_err.to_string()
            ))),
        }
    });
    Ok(new_user)
}

#[server(ReadUsers, "/api/users")]
pub async fn get_all_users() -> Result<Vec<UserModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::User;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let mut users: Vec<UserModel> = User::find()
        .all(&state.conn)
        .await
        .unwrap_or_default()
        .iter()
        .map(|model| UserModel::from(model.clone()))
        .collect();

    for user in users.iter_mut() {
        user.password = String::new();
    }

    Ok(users)
}

#[server(ReadUser, "/api/users")]
pub async fn get_user(user_id: i32) -> Result<Option<UserModel>, ServerFnError> {
    match get_user_guard(user_id).await {
        Ok(user_exists) => match user_exists {
            Some(mut user) => {
                user.password = String::new();
                Ok(Some(user))
            }
            None => Ok(None),
        },
        Err(msg) => Err(msg),
    }
}

/// Get a user from database based on it's id, hide sensitive content like password
#[cfg(feature = "ssr")]
pub async fn get_user_guard(user_id: i32) -> Result<Option<UserModel>, ServerFnError> {
    use crate::AppState;
    println!("Reading user...");
    use entities::prelude::User;
    use entities::user;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    println!("Querying user...");
    let user =
        match User::find()
            .filter(user::Column::Id.eq(user_id))
            .one(&state.conn)
            .await
        {
            Ok(user) => match user {
                Some(model) => Some({
                    println!("User found!");
                    let user = UserModel::from(model);
                    user
                }),
                None => None,
            },
            Err(db_err) => return Err(ServerFnError::new(format!(
                "A error happened while trying to make this request, try again later. DbErr: {}",
                db_err.to_string()
            ))),
        };

    Ok(user)
}

#[server(UpdateUser, "/api/users")]
pub async fn update_user(updated_user: UserModel) -> Result<UserModel, ServerFnError> {
    if let Some(user) = user_logged_in().await? {
        if user == updated_user.id || user_have_permission(user, "Moderar".to_string()).await? {
            return update_user_guard(updated_user).await;
        } else {
            return Err(ServerFnError::new(
                "User doesn't have the permission to execute this operation.",
            ));
        }
    } else {
        return Err(ServerFnError::new("User is not logged in."));
    }
}

#[cfg(feature = "ssr")]
pub async fn update_user_guard(updated_user: UserModel) -> Result<UserModel, ServerFnError> {
    use crate::AppState;

    use entities::prelude::User;
    use entities::user;

    use argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    };

    let user = get_user_guard(updated_user.id).await?.unwrap();

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let mut user_model: user::ActiveModel =
        match User::find()
            .filter(user::Column::Id.eq(updated_user.id))
            .one(&state.conn)
            .await
        {
            Ok(user) => user,
            Err(db_err) => return Err(ServerFnError::new(format!(
                "A error happened while trying to make this request, try again later. DbErr: {}",
                db_err.to_string()
            ))),
        }
        .unwrap()
        .into();

    drop(state);

    if updated_user == user {
        return Ok(UserModel::from(user));
    }

    if updated_user.username != user.username {
        push_notification(NotificationModel {
            title: "Changes on username has been made".to_string(),
            description: Some(format!(
                "Someone has changed your username from \"<b>{}</b>\" to \"<b>{}</b>\"",
                user.username, updated_user.username
            )),
            recipient_id: user.id,
            created_at: Utc::now()
                .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")),
            ..Default::default()
        })
        .await?;
        user_model.username = Set(updated_user.username)
    }

    if updated_user.email != user.email {
        push_notification(NotificationModel {
            title: "Changes on your email has been made".to_string(),
            description: Some(format!(
                "Someone has changed your email from \"<b>{}</b>\" to \"<b>{}</b>\"",
                user.email, updated_user.email
            )),
            recipient_id: user.id,
            created_at: Utc::now()
                .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")),
            ..Default::default()
        })
        .await?;

        user_model.email = Set(updated_user.email)
    }

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    if !updated_user.password.is_empty() {
        let secret = state.secret_key.clone().into_bytes();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new_with_secret(
            secret.as_slice(),
            argon2::Algorithm::default(),
            argon2::Version::default(),
            argon2::Params::default(),
        )
        .unwrap();

        let password_hash = PasswordHash::new(&user.password).unwrap();

        if let Err(_) = argon2.verify_password(&updated_user.password.as_bytes(), &password_hash) {
            let password_hash = argon2
                .hash_password(&updated_user.password.into_bytes(), &salt)
                .unwrap();
            user_model.password = Set(password_hash.to_string());
        }
    }

    user_model.updated_at =
        Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")));

    match user_model.update(&state.conn).await {
        Ok(model) => Ok(UserModel::from(model)),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened while trying to make this request, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}

#[server(DeleteUser, "/api/users")]
pub async fn delete_user(user_id: i32) -> Result<Option<UserModel>, ServerFnError> {
    if let Some(user) = user_logged_in().await? {
        if user == user_id || user_have_permission(user, "Moderar".to_string()).await? {
            return delete_user_guard(user_id).await;
        } else {
            return Err(ServerFnError::new(
                "User doesn't have the permission to execute this operation.",
            ));
        }
    } else {
        return Err(ServerFnError::new("User is not logged in."));
    }
}

#[cfg(feature = "ssr")]
pub async fn delete_user_guard(user_id: i32) -> Result<Option<UserModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::User;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let user = match User::find_by_id(user_id).one(&state.conn).await {
        Ok(user_exists) => match user_exists {
            Some(user) => user,
            None => return Ok(None),
        },
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting the user, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    let deleted_user = UserModel::from(user.clone());
    match user.delete(&state.conn).await {
        Ok(_) => Ok(Some(deleted_user)),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when deleting the user, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}

// User related operations

/// Query user by email
#[cfg(feature = "ssr")]
pub async fn get_user_by_email(email: String) -> Result<Option<UserModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::User;
    use entities::user;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let user =
        match User::find()
            .filter(user::Column::Email.eq(email))
            .one(&state.conn)
            .await
        {
            Ok(user) => match user {
                Some(model) => Some({
                    let mut user = UserModel::from(model);
                    user
                }),
                None => None,
            },
            Err(db_err) => return Err(ServerFnError::new(format!(
                "A error happened while trying to make this request, try again later. DbErr: {}",
                db_err.to_string()
            ))),
        };

    Ok(user)
}

/// Get all roles assigned to user.
#[server(GetUserRoles, "/api/users")]
pub async fn get_user_roles(user_id: i32) -> Result<Vec<RoleModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::UserRoles;
    use entities::user_roles;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let user_role_ids: Vec<i32> = UserRoles::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .all(&state.conn)
        .await
        .unwrap_or_default()
        .iter()
        .map(|model| model.role_id)
        .collect();

    let mut roles = vec![];

    for role_id in user_role_ids {
        let user_role = get_role(role_id).await?.unwrap();

        roles.push(user_role);
    }

    Ok(roles)
}

/// Get the unique permissions for User, iterating over it's roles.
#[server(GetUserPermissions, "/api/users")]
pub async fn get_user_permissions(user_id: i32) -> Result<Vec<PermissionModel>, ServerFnError> {
    let user_roles = get_user_roles(user_id).await?;

    let mut unique_permissions = vec![];

    for role in user_roles {
        for permission in role.children {
            if !unique_permissions.contains(&permission) {
                unique_permissions.push(permission)
            }
        }
    }

    Ok(unique_permissions)
}

/// Check if the user is logged in.
#[server(IsUserLoggedIn, "/api/users")]
pub async fn user_logged_in() -> Result<Option<i32>, ServerFnError> {
    use tower_sessions::Session;

    let session: Session = extract().await?;
    if let Some(user) = session
        .get::<i32>("id")
        .await
        .expect("Error with connection")
    {
        let user = get_user(user).await?.unwrap();
        return Ok(Some(user.id));
    } else {
        return Ok(None);
    }
}

/// Check if the user is logged in.
#[server(LoginUser, "/api/users")]
pub async fn login_user(email: String, password: String) -> Result<(), ServerFnError> {
    use tower_sessions::Session;

    let session: Session = extract().await?;
    if let Some(user) = get_user_by_email(email).await? {
        use crate::AppState;
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };

        let state: Arc<Mutex<AppState>> = use_context().unwrap();
        let state = state.as_ref().lock().await;

        let secret = state.secret_key.clone().into_bytes();

        let argon2 = Argon2::new_with_secret(
            secret.as_slice(),
            argon2::Algorithm::default(),
            argon2::Version::default(),
            argon2::Params::default(),
        )
        .unwrap();

        let password_hash = PasswordHash::new(&user.password).unwrap();

        if let Ok(_) = argon2.verify_password(&password.as_bytes(), &password_hash) {
            session
                .insert("id", user.id)
                .await
                .expect("Error with connection");
            Ok(())
        } else {
            return Err(ServerFnError::new(format!("Incorrect Password")));
        }
    } else {
        return Err(ServerFnError::new(format!("This account doesn't exists")));
    }
}

/// Check if any role assigned to user have the permission requested.
#[server(UserHavePermission, "/api/users")]
pub async fn user_have_permission(
    user_id: i32,
    permission_name: String,
) -> Result<bool, ServerFnError> {
    let user_permissions = get_user_permissions(user_id).await?;

    for permission in user_permissions {
        if permission.name == permission_name {
            return Ok(true);
        }
    }

    Ok(false)
}
