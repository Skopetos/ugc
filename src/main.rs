use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::post,
};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use std::time::Duration;
use serde::Deserialize;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct ContactForm {
    name:    String,
    email:   String,
    brand:   Option<String>,
    service: Option<String>,
    message: String,
}

async fn contact(Json(payload): Json<ContactForm>) -> StatusCode {
    let smtp_user = std::env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = std::env::var("SMTP_PASS").unwrap_or_default();

    let brand   = payload.brand.unwrap_or_else(|| "—".into());
    let service = payload.service.unwrap_or_else(|| "—".into());

    let body = format!(
        "New inquiry from TryItAway contact form\n\nName: {}\nEmail: {}\nBrand: {}\nService: {}\n\nMessage:\n{}",
        payload.name, payload.email, brand, service, payload.message
    );

    let email = Message::builder()
        .from(smtp_user.parse().unwrap())
        .to(smtp_user.parse().unwrap())
        .reply_to(payload.email.parse().unwrap_or_else(|_| smtp_user.parse().unwrap()))
        .subject(format!("New inquiry from {}", payload.name))
        .body(body)
        .unwrap();

    let creds = Credentials::new(smtp_user, smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .timeout(Some(Duration::from_secs(10)))
        .build();

    match mailer.send(email).await {
        Ok(_)  => StatusCode::OK,
        Err(e) => {
            eprintln!("Mail error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/api/contact", post(contact))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
