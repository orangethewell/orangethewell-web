use leptos::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::{GetUrl, MultipartData, MultipartFormData};
use std::sync::Arc;

#[cfg(feature = "ssr")]
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct ImageMetadataModel {
    pub id: i32,
    pub image_path: String,
}

#[cfg(feature = "ssr")]
impl From<entities::image_metadata::Model> for ImageMetadataModel {
    fn from(value: entities::image_metadata::Model) -> Self {
        Self {
            id: value.id,
            image_path: value.image_path,
        }
    }
}

// Image Create/Read/Delete

#[server(UploadImage, "/api/gallery", input = MultipartFormData)]
pub async fn upload_image_to_gallery(
    data: MultipartData,
) -> Result<ImageMetadataModel, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    use crate::AppState;

    use entities::image_metadata;
    use sea_orm::{ActiveModelTrait, Set};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    // this will just measure the total number of bytes uploaded
    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = format!("data/uploads/{}", field.file_name().unwrap_or("data_file"));
        if name == "file_to_upload" {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&file_name)
                .await
                .unwrap();

            let image_meta = image_metadata::ActiveModel {
                image_path: Set(file_name.clone()),
                ..Default::default()
            };

            let image = image_meta.insert(&state.conn).await.unwrap();

            while let Ok(Some(chunk)) = field.chunk().await {
                file.write(&chunk).await.unwrap();
            }

            return Ok(ImageMetadataModel::from(image));
        }
    }

    Err(ServerFnError::new(
        "Something went wrong on uploading this image.",
    ))
}

#[server(GetImageLen, "/api/gallery")]
pub async fn get_image_len() -> Result<usize, ServerFnError> {
    use crate::AppState;

    use entities::image_metadata::Model;
    use entities::prelude::ImageMetadata;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    match ImageMetadata::find().all(&state.conn).await {
        Ok(images) => Ok(images.len()),
        _ => Ok(0),
    }
}

#[deprecated = "Use URL `/gallery/[image_id]` instead."]
/// Get image from server using `image_id`. This is deprecated in favor of using the `/gallery/{image_id}`
/// URL path over a server function.
#[server(GetImage, "/api/gallery", input = GetUrl)]
pub async fn get_image(image_id: i32) -> Result<Option<Vec<u8>>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::ImageMetadata;
    use sea_orm::EntityTrait;

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    match ImageMetadata::find_by_id(image_id).one(&state.conn).await {
        Ok(image_exists) => match image_exists {
            Some(image) => {
                let data = fs::read(&image.image_path).await.unwrap();
                Ok(Some(data))
            }
            None => return Ok(None),
        },

        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting the image, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}

#[server(DeleteImage, "/api/gallery")]
pub async fn delete_image_from_gallery(
    image_id: i32,
) -> Result<Option<ImageMetadataModel>, ServerFnError> {
    use crate::AppState;

    use entities::prelude::ImageMetadata;
    use sea_orm::{EntityTrait, ModelTrait};

    let state: Arc<Mutex<AppState>> = use_context().unwrap();
    let state = state.as_ref().lock().await;

    let image = match ImageMetadata::find_by_id(image_id).one(&state.conn).await {
        Ok(image_exists) => match image_exists {
            Some(image) => image,
            None => return Ok(None),
        },

        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when requesting the image, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    };

    let deleted_image = ImageMetadataModel::from(image.clone());

    match image.delete(&state.conn).await {
        Ok(_) => match fs::remove_file(&deleted_image.image_path).await {
            Ok(_) => Ok(Some(deleted_image)),
            Err(file_err) => {
                return Err(ServerFnError::new(format!(
                    "A error occured when removing image from filesystem. FileErr: {}",
                    file_err.to_string()
                )))
            }
        },
        Err(db_err) => {
            return Err(ServerFnError::new(format!(
                "A error happened when removing the image, try again later. DbErr: {}",
                db_err.to_string()
            )))
        }
    }
}
