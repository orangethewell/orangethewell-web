//! ## Roles and Permissions
//! This module contains tools to deal with role and permission management. At all, permissions
//! are static, because they are intrinsic connected with the application functionality, but
//! roles are dynamic, so you can't just assign and unsign roles to a user, but you also can
//! create new roles with new permissions.

use leptos::*;
use std::sync::Arc;
#[cfg(feature = "ssr")]
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct GenericNDModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub children: Vec<GenericNDModel>,
}

pub type PermissionModel = GenericNDModel;
pub type RoleModel = GenericNDModel;

#[cfg(feature = "ssr")]
impl From<entities::role::Model> for RoleModel {
    fn from(value: entities::role::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            children: vec![],
        }
    }
}

#[cfg(feature = "ssr")]
impl From<entities::permission::Model> for PermissionModel {
    fn from(value: entities::permission::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            children: vec![],
        }
    }
}

// Role Create/Read/Update/Delete

/// Creates a new role with some permissions
#[server(CreateRole, "/api/roles")]
pub async fn create_role(new_role: RoleModel) -> Result<RoleModel, ServerFnError> {
    use crate::AppState;

    use entities::{role, role_permissions};
    use sea_orm::{ActiveModelTrait, Set, TransactionTrait};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let transaction = match state.conn.begin().await {
        Ok(transaction) => transaction,
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened while starting a new transaction over database. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    let role = role::ActiveModel {
        name: Set(new_role.name),
        description: Set(new_role.description),
        ..Default::default()
    };

    let role = match role.insert(&state.conn).await {
        Ok(role) => role,
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error occured when inserting a new role to database. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    for permission in new_role.children {
        let role_permission = role_permissions::ActiveModel {
            role_id: Set(role.id),
            permission_id: Set(permission.id),
            ..Default::default()
        };

        match role_permission.save(&transaction).await {
            Ok(_) => (),
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when saving permission-role relationship, try again later. DbErr: {}", db_err.to_string())
                )
            )
        }
    }

    match transaction.commit().await {
        Ok(_) => Ok(RoleModel::from(role)),
        Err(db_err) => return Err(
            ServerFnError::new(
                format!("A error happened when creating the role and assigning permissions, try again later. DbErr: {}", db_err.to_string())
            )
        )
    }
}

#[server(ReadRoles, "/api/roles")]
pub async fn get_all_roles() -> Result<Vec<RoleModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::{Permission, Role, RolePermissions};
    use entities::role_permissions;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let mut roles: Vec<RoleModel> = Role::find()
        .all(&state.conn)
        .await
        .unwrap_or_default()
        .iter()
        .map(|model| RoleModel::from(model.clone()))
        .collect();

    for role in roles.iter_mut() {
        let permission_ids: Vec<i32> = match RolePermissions::find()
            .filter(role_permissions::Column::RoleId.eq(role.id))
            .all(&state.conn)
            .await {
                Ok(permission_ids) => permission_ids.iter().map(|perm| perm.permission_id).collect(),
                Err(db_err) => return Err(
                    ServerFnError::new(
                        format!("A error happened when requesting the role's assigned permissions, try again later. DbErr: {}", db_err.to_string())
                    )
                )
            };

        let mut permissions = vec![];

        for id in permission_ids {
            permissions.push(match Permission::find_by_id(id).one(&state.conn).await {
                Ok(permission) => PermissionModel::from(permission.unwrap()),
                Err(db_err) => return Err(
                    ServerFnError::new(
                        format!("A error happened when requesting a role's assigned permission, try again later. DbErr: {}", db_err.to_string())
                    )
                )
            })
        }
        role.children = permissions;
    }

    Ok(roles)
}

/// Read the Role specified by it ID together with it's permissions.
#[server(ReadRole, "/api/roles")]
pub async fn get_role(role_id: i32) -> Result<Option<RoleModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::{Permission, Role, RolePermissions};
    use entities::role_permissions;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let mut role = RoleModel::from(
        match Role::find_by_id(role_id).one(&state.conn).await {
            Ok(role_exists) => match role_exists {
                Some(role) => role,
                None => return Ok(None)
            },
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting the role and assigned permissions, try again later. DbErr: {}", db_err.to_string())
                )
            )
        }
    );

    let permission_ids: Vec<i32> = match RolePermissions::find()
        .filter(role_permissions::Column::RoleId.eq(role.id))
        .all(&state.conn)
        .await {
            Ok(permission_ids) => permission_ids.iter().map(|perm| perm.permission_id).collect(),
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting the role's assigned permissions, try again later. DbErr: {}", db_err.to_string())
                )
            )
        };

    let mut permissions = vec![];

    for id in permission_ids {
        permissions.push(match Permission::find_by_id(id).one(&state.conn).await {
            Ok(permission) => PermissionModel::from(permission.unwrap()),
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting a role's assigned permission, try again later. DbErr: {}", db_err.to_string())
                )
            )
        })
    }

    role.children = permissions;
    Ok(Some(role))
}

/// Update a Role based on it's model.
#[server(UpdateRole, "/api/roles")]
pub async fn update_role(updated_role: RoleModel) -> Result<RoleModel, ServerFnError> {
    use crate::AppState;

    use entities::prelude::{Role, RolePermissions};
    use entities::{role, role_permissions};
    use sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set, TransactionTrait,
    };

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let transaction = match state.conn.begin().await {
        Ok(transaction) => transaction,
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened while starting a new transaction over database. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    let role = match Role::find_by_id(updated_role.id).one(&state.conn).await {
        Ok(role) => role,
        Err(db_err) => return Err(
            ServerFnError::new(
                format!("A error happened when requesting the role and assigned permissions, try again later. DbErr: {}", db_err.to_string())
            )
        )
    };

    let mut role: role::ActiveModel = role
        .expect("Unexpected 'None' value when updating role")
        .into();

    role.name = Set(updated_role.name);
    role.description = Set(updated_role.description);

    let assigned_permissions = match RolePermissions::find()
        .filter(role_permissions::Column::RoleId.eq(role.id.clone().unwrap()))
        .all(&state.conn)
        .await {
            Ok(permission_ids) => permission_ids,
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting the role's assigned permissions, try again later. DbErr: {}", db_err.to_string())
                )
            )
        };

    for permission in assigned_permissions.iter() {
        if !updated_role
            .children
            .iter()
            .any(|relation| permission.id == relation.id)
        {
            permission
                .clone()
                .delete(&transaction)
                .await
                .expect("Unexpected invalid model on delete");
        }
    }

    for permission in updated_role.children.iter() {
        if !assigned_permissions
            .iter()
            .any(|relation| permission.id == relation.id)
        {
            let new_permission_assignment = role_permissions::ActiveModel {
                role_id: Set(updated_role.id),
                permission_id: Set(permission.id),
                ..Default::default()
            };

            new_permission_assignment
                .insert(&transaction)
                .await
                .expect("Unexpected invalid model on insert");
        }
    }

    let mut result = match role.update(&transaction).await {
        Ok(updated) => RoleModel::from(updated),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting role update, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    match transaction.commit().await {
        Ok(_) => (),
        Err(db_err) => return Err(
            ServerFnError::new(
                format!("A error happened when creating the role and assigning permissions, try again later. DbErr: {}", db_err.to_string())
            )
        )
    }

    result.children = updated_role.children.clone();
    Ok(result)
}

/// Delete a Role specified by it's ID.
#[server(DeleteRole, "/api/roles")]
pub async fn delete_role(role_id: i32) -> Result<Option<RoleModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::Role;
    use sea_orm::{EntityTrait, ModelTrait};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let role = match Role::find_by_id(role_id).one(&state.conn).await {
        Ok(role_exists) => match role_exists {
            Some(role) => role,
            None => return Ok(None),
        },
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting the role, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    let deleted_role = RoleModel::from(role.clone());
    match role.delete(&state.conn).await {
        Ok(_) => Ok(Some(deleted_role)),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when deleting the role, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}

// Permission Read

/// Get all permissions on the application.
#[server(ReadPermissions, "/api/permissions")]
pub async fn get_all_permissions() -> Result<Vec<PermissionModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::Permission;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let permissions: Vec<PermissionModel> = Permission::find()
        .all(&state.conn)
        .await
        .unwrap_or_default()
        .iter()
        .map(|model| PermissionModel::from(model.clone()))
        .collect();

    Ok(permissions)
}

/// Read the permission specified by it's ID.
#[server(ReadPermission, "/api/permissions")]
pub async fn get_permission(permission_id: i32) -> Result<Option<PermissionModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::Permission;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let permission = PermissionModel::from(
        match Permission::find_by_id(permission_id).one(&state.conn).await {
            Ok(permission_exists) => match permission_exists {
                Some(permission) => permission,
                None => return Ok(None)
            },
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting the role and assigned permissions, try again later. DbErr: {}", db_err.to_string())
                )
            )
        }
    );

    Ok(Some(permission))
}
