use axum::{
    routing::{get},
    response::Redirect,
    Router,
};
use std::env;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(get(move || async {
            Redirect::permanent(&env::var("REDIR_URL")
                .unwrap_or("https://www.mint.se".to_string()))
        }));

    let port = env::var("APP_PORT").unwrap_or("5000".to_string());
    let iface = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(iface).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

