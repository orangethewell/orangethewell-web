use leptos::*;
use std::sync::Arc;
#[cfg(feature = "ssr")]
use tokio::sync::Mutex;

#[cfg(feature = "ssr")]
use sea_orm::*;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct NotificationModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub recipient_id: i32,
    pub read: bool,
    pub created_at: DateTime<FixedOffset>,
}

#[cfg(feature = "ssr")]
impl From<entities::notification::Model> for NotificationModel {
    fn from(value: entities::notification::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            recipient_id: value.recipient_id,
            created_at: value.created_at,
            read: value.read,
        }
    }
}

#[server(PushNotification, "/api/notifitications")]
pub async fn push_notification(
    new_notification: NotificationModel,
) -> Result<NotificationModel, ServerFnError> {
    use crate::AppState;

    use entities::notification;

    println!("Sending notifications");

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    println!("Preparing notifier");

    let notify = notification::ActiveModel {
        title: Set(new_notification.title),
        description: Set(new_notification.description),
        recipient_id: Set(new_notification.recipient_id),
        created_at: Set(
            Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))
        ),
        ..Default::default()
    };

    let notify = NotificationModel::from({
        match notify.insert(&state.conn).await {
            Ok(notification) => notification,
            Err(db_err) => return Err(ServerFnError::new(format!(
                "A error happened while trying to push a notification, try again later. DbErr: {}",
                db_err.to_string()
            ))),
        }
    });

    drop(state);

    remove_old_readed_notifications().await?;

    Ok(notify)
}

#[server(GetUserNotifications, "/api/notifications")]
pub async fn get_user_notifications(user_id: i32) -> Result<Vec<NotificationModel>, ServerFnError> {
    use crate::AppState;

    use entities::notification;
    use entities::prelude::Notification;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;
    let notifications = Notification::find()
        .filter(notification::Column::RecipientId.eq(user_id))
        .all(&state.conn)
        .await
        .unwrap();

    let transaction = match state.conn.begin().await {
        Ok(transaction) => transaction,
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened while starting a new transaction over database. DbErr: {}",
                db_err.to_string()
            )))
        }
    };
    for notification in notifications.iter() {
        let mut model: notification::ActiveModel = notification.clone().into();
        model.read = Set(true);
        match model.update(&transaction).await {
            Ok(_) => (),
            Err(db_err) => {
                return Err(ServerFnError::new(format!(
                    "A error happened while pushing a new notification over database. DbErr: {}",
                    db_err.to_string()
                )))
            }
        }
    }

    let result = match transaction.commit().await {
        Ok(_) => Ok(notifications.iter().map(|notify| NotificationModel::from(notify.clone())).collect()),
        Err(db_err) => return Err(
            ServerFnError::new(
                format!("A error happened when creating the role and assigning permissions, try again later. DbErr: {}", db_err.to_string())
            )
        )
    };

    drop(state);
    remove_old_readed_notifications().await?;
    result
}

#[server(DeleteOldNotifications, "/api/notifitications")]
pub async fn remove_old_readed_notifications() -> Result<(), ServerFnError> {
    use crate::AppState;

    use entities::prelude::Notification;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let notifications = Notification::find().all(&state.conn).await.unwrap();

    for notification in notifications {
        let date_diff = Utc::now()
            .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))
            - notification.created_at;
        if date_diff.num_days() > 30 && notification.read {
            notification.delete(&state.conn).await?;
        }
    }

    Ok(())
}
