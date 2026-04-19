use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{get_service, post},
};
use serde::Deserialize;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

async fn contact(Json(payload): Json<ContactForm>) -> StatusCode {
    // Log inquiry — hook up an email provider (Resend, Postmark) when ready
    println!(
        "New inquiry from {} <{}>\n{}",
        payload.name, payload.email, payload.message
    );
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let static_files = get_service(ServeDir::new("static").append_index_html_on_directories(true));

    let app = Router::new()
        .route("/api/contact", post(contact))
        .fallback_service(static_files)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
