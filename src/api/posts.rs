use chrono::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg(feature = "ssr")]
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

use super::users::{get_user, user_have_permission, user_logged_in, UserModel};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct PostMetadataModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub short_desc: Option<String>,
    pub writer_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub content_path: String,
}

#[cfg(feature = "ssr")]
impl From<entities::post_metadata::Model> for PostMetadataModel {
    fn from(value: entities::post_metadata::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            slug: value.slug,
            short_desc: value.short_desc,
            writer_id: value.writer_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            content_path: value.content_path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub short_desc: Option<String>,
    pub writer: UserModel,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub content: String,
}

// Post Create/Read/Update/Delete

#[server(CreateArticle, "/api/articles")]
pub async fn create_article(new_post: PostModel) -> Result<PostMetadataModel, ServerFnError> {
    use crate::AppState;

    use entities::post_metadata;
    use sea_orm::{ActiveModelTrait, Set};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();

    let data_path = format!("data/{}.md", new_post.slug);
    let mut data_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&data_path)
        .await
        .unwrap();

    match data_file.write_all(new_post.content.as_bytes()).await {
        Ok(_) => {
            let article = post_metadata::ActiveModel {
                title: Set(new_post.title),
                slug: Set(new_post.slug),
                short_desc: Set(new_post.short_desc),
                writer_id: Set(new_post.writer.id),
                created_at: Set(Utc::now()
                    .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
                updated_at: Set(Utc::now()
                    .with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
                content_path: Set(data_path),
                ..Default::default()
            };

            let state = state.as_ref().lock().await;
            match article.insert(&state.conn).await {
                Ok(post_meta) => Ok(PostMetadataModel::from(post_meta)),
                Err(db_err) => {
                    return Err(ServerFnError::new(format!(
                        "A error occured when inserting a new file to database. DbErr: {}",
                        db_err.to_string()
                    )))
                }
            }
        }

        Err(file_err) => {
            return Err(ServerFnError::new(format!(
                "A error occured when inserting a new role to database. FileErr: {}",
                file_err.to_string()
            )))
        }
    }
}

#[server(ReadArticles, "/api/articles")]
pub async fn get_all_articles() -> Result<Vec<PostMetadataModel>, ServerFnError> {
    use crate::AppState;

    use entities::post_metadata;
    use entities::prelude::PostMetadata;
    use sea_orm::{EntityTrait, QueryOrder};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let articles: Vec<PostMetadataModel> = PostMetadata::find()
        .order_by_desc(post_metadata::Column::UpdatedAt)
        .all(&state.conn)
        .await
        .unwrap_or_default()
        .iter()
        .map(|model| PostMetadataModel::from(model.clone()))
        .collect();

    Ok(articles)
}

/// Read the Role specified by it ID together with it's permissions.
#[server(ReadArticle, "/api/articles")]
pub async fn get_article(slug: String) -> Result<Option<PostModel>, ServerFnError> {
    use crate::AppState;

    use entities::post_metadata;
    use entities::prelude::PostMetadata;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let article_metadata = PostMetadataModel::from(
        match PostMetadata::find().filter(post_metadata::Column::Slug.eq(slug)).one(&state.conn).await {
            Ok(post_exists) => match post_exists {
                Some(post) => post,
                None => return Ok(None)
            },
            Err(db_err) => return Err(
                ServerFnError::new(
                    format!("A error happened when requesting the post and assigned content, try again later. DbErr: {}", db_err.to_string())
                )
            )
        }
    );

    std::mem::drop(state);

    let writer = get_user(article_metadata.writer_id).await?;
    Ok(Some(PostModel {
        title: article_metadata.title,
        slug: article_metadata.slug,
        short_desc: article_metadata.short_desc,
        writer: writer.unwrap(),
        created_at: article_metadata.created_at,
        updated_at: article_metadata.updated_at,
        content: fs::read_to_string(&article_metadata.content_path)
            .await
            .unwrap(),
        id: article_metadata.id,
    }))
}

/// Update a Role based on it's model.
#[server(UpdateArticle, "/api/articles")]
pub async fn update_article(
    updated_article: PostModel,
) -> Result<PostMetadataModel, ServerFnError> {
    use crate::AppState;

    use entities::post_metadata;
    use entities::prelude::PostMetadata;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let article_metadata = match PostMetadata::find_by_id(updated_article.id).one(&state.conn).await {
        Ok(article) => article.expect("Unexpected 'None' value when updating article"),
        Err(db_err) => return Err(
            ServerFnError::new(
                format!("A error happened when requesting the role and assigned permissions, try again later. DbErr: {}", db_err.to_string())
            )
        )
    };

    let mut article_data = fs::OpenOptions::new()
        .write(true)
        .open(&article_metadata.content_path)
        .await
        .unwrap();

    article_data
        .write_all(updated_article.content.as_bytes())
        .await
        .unwrap();

    let mut article: post_metadata::ActiveModel = article_metadata.into();

    article.title = Set(updated_article.title);
    article.short_desc = Set(updated_article.short_desc);
    article.slug = Set(updated_article.slug);
    article.updated_at =
        Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")));

    match article.update(&state.conn).await {
        Ok(post_meta) => Ok(PostMetadataModel::from(post_meta)),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error occured when inserting a new file to database. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}

#[server(DeleteArticle, "/api/articles")]
pub async fn delete_article(article_id: i32) -> Result<Option<PostMetadataModel>, ServerFnError> {
    if let Some(user) = user_logged_in().await? {
        if user_have_permission(user, "Escrever".to_string()).await? {
            return delete_article_guard(article_id).await;
        } else {
            return Err(ServerFnError::new(
                "User doesn't have the permission to execute this operation.",
            ));
        }
    } else {
        return Err(ServerFnError::new("User is not logged in."));
    }
}

/// Delete a Role specified by it's ID.
#[cfg(feature = "ssr")]
pub async fn delete_article_guard(
    article_id: i32,
) -> Result<Option<PostMetadataModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::PostMetadata;
    use sea_orm::{EntityTrait, ModelTrait};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let article = match PostMetadata::find_by_id(article_id).one(&state.conn).await {
        Ok(article_exists) => match article_exists {
            Some(article) => article,
            None => return Ok(None),
        },
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting the article, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    fs::remove_file(&article.content_path).await.unwrap();

    let deleted_article = PostMetadataModel::from(article.clone());
    match article.delete(&state.conn).await {
        Ok(_) => Ok(Some(deleted_article)),
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when deleting the article, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}
