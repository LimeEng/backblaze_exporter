use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use backblaze_exporter::{
    backblaze,
    prom::{self},
};
use serde::Serialize;
use std::io;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let port = 9100;

    let app = Router::new().route("/metrics", get(metrics));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    tracing::info!("Listening on port {port}");

    axum::serve(listener, app).await.unwrap();
}

async fn metrics() -> Result<String, AppError> {
    let disks = backblaze::collect_metrics()?;
    let encoded = prom::encode(&disks);

    Ok(encoded)
}

enum AppError {
    Io,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::Io => (StatusCode::INTERNAL_SERVER_ERROR, "IO error".to_string()),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<io::Error> for AppError {
    fn from(_err: io::Error) -> Self {
        AppError::Io
    }
}
