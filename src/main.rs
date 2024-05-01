use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let api_router = Router::new()
        .route("/hello", get(hello))
        .route("/test", get(test));

    let app = Router::new()
        .route("/", get(home))
        .nest("/actions", api_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Template)]
#[template(path = "pages/hello.html")]
struct HomeTemplate;

async fn home() -> HomeTemplate {
    info!("hitting home");
    HomeTemplate
}

async fn hello() -> &'static str {
    "Hello"
}

#[derive(Template)]
#[template(path = "components/test.html")]
struct TestComponent;

async fn test() -> TestComponent {
    TestComponent
}
