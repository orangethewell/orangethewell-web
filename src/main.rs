#![feature(duration_constructors)]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{extract::Path, routing::get, Router};

    use tower_sessions::cookie::time::Duration;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

    use dotenv::dotenv;
    use std::env;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use sea_orm::Database;

    use migration::{MigrationTrait, Migrator, MigratorTrait};

    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use orangethewell_web::app::*;
    use orangethewell_web::fileserv::{file_and_error_handler, get_image_by_id_handler};
    use orangethewell_web::AppState;

    dotenv().ok();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Setup redis connections
    let pool = RedisPool::new(
        RedisConfig::from_url(&env::var("REDIS_URL").unwrap()).unwrap(),
        None,
        None,
        None,
        6,
    )?;
    let redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::new(pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(3)));

    // Load database connection and secret key for encrypting
    let conn = Database::connect(env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let secret_key = env::var("SECRET_KEY").unwrap();
    let state = Arc::new(Mutex::new(AppState { conn, secret_key }));
    let state_2 = state.clone();

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state.clone()),
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .route(
            "/gallery/:id",
            get(move |id: Path<i32>| get_image_by_id_handler(id, state_2.clone())),
        )
        //.layer(axum::Extension(Arc::new(state.clone())))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    redis_conn.await??;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
