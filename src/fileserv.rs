use crate::{app::App, AppState};
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use http::header;
use leptos::*;
use std::sync::Arc;
use tokio::{fs, sync::Mutex};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let mut res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.headers_mut().insert(
            "Cross-Origin-Embedder-Policy",
            "require-corp".parse().unwrap(),
        );
        res.headers_mut()
            .insert("Cross-Origin-Opener-Policy", "same-origin".parse().unwrap());

        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
        handler(req).await.into_response()
    }
}

async fn get_image_with_extern_state(
    image_id: i32,
    state_ref: Arc<Mutex<AppState>>,
) -> Result<Option<Vec<u8>>, ServerFnError> {
    use entities::prelude::ImageMetadata;
    use sea_orm::EntityTrait;

    let state = state_ref.as_ref().lock().await;

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

pub async fn get_image_by_id_handler(
    Path(id): Path<i32>,
    state: Arc<Mutex<AppState>>,
) -> impl IntoResponse {
    let image = match get_image_with_extern_state(id, state).await {
        Ok(Some(data)) => data,
        _ => vec![],
    };

    ([(header::CONTENT_TYPE, "image/png")], image)
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
