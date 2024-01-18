use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod todo;

use todo::TodoTemplate;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    todos: Vec<TodoTemplate>,
}

async fn index(
    axum::extract::State(pool): axum::extract::State<db::DBPool>,
) -> Result<IndexTemplate, (axum::http::StatusCode, String)> {
    sqlx::query_as!(TodoTemplate, "SELECT * FROM todo")
        .fetch_all(&pool)
        .await
        .map(|todos| IndexTemplate { todos })
        .map_err(db::map_db_err)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::connect_to_database().await;

    sqlx::migrate!().run(&pool).await.unwrap();

    // build our application
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .nest_service(
            "/favicon.ico",
            tower_http::services::ServeFile::new("assets/favicon.ico"),
        )
        .nest("/api", api::api_routes())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(pool);

    #[cfg(debug_assertions)]
    let app = {
        use notify::Watcher;
        let livereload = tower_livereload::LiveReloadLayer::new().request_predicate(
            |req: &axum::http::Request<axum::body::Body>| !req.headers().contains_key("hx-request"),
        );
        let reloader = livereload.reloader();
        let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
        watcher
            .watch(
                std::path::Path::new("assets"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        watcher
            .watch(
                std::path::Path::new("templates"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        tracing::info!("Reloading!");
        app.layer(livereload)
    };

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
