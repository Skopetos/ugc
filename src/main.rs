use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::{get, post},
};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct MediaItem {
    src:        String,
    category:   String,
    label:      String,
    media_type: String,
}

fn category_label(cat: &str) -> &str {
    match cat {
        "travel"                  => "Travel Lifestyle",
        "food-coffee"             => "Food & Coffee",
        "accommodation"           => "Accommodation",
        "drone"                   => "Drone",
        "aesthetic-destinations"  => "Aesthetic Destinations",
        _                         => cat,
    }
}

async fn recent_list() -> axum::Json<Vec<MediaItem>> {
    let mut items: Vec<MediaItem> = Vec::new();

    if let Ok(files) = std::fs::read_dir("static/assets/recent") {
        for file in files.flatten() {
            let fname = file.file_name().to_string_lossy().to_string();
            let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();
            let media_type = match ext.as_str() {
                "jpg" | "jpeg" | "png" | "webp" => "image",
                "mp4" | "mov" | "webm"          => "video",
                _                               => continue,
            };
            items.push(MediaItem {
                src:        format!("/assets/recent/{}", fname),
                category:   String::new(),
                label:      String::new(),
                media_type: media_type.to_string(),
            });
        }
    }

    axum::Json(items)
}

async fn media_list() -> axum::Json<Vec<MediaItem>> {
    let mut items: Vec<MediaItem> = Vec::new();

    if let Ok(cats) = std::fs::read_dir("static/assets/images") {
        for cat in cats.flatten() {
            let cat_name = cat.file_name().to_string_lossy().to_string();
            if cat_name == "hero" { continue; }
            if let Ok(files) = std::fs::read_dir(cat.path()) {
                for file in files.flatten() {
                    let fname = file.file_name().to_string_lossy().to_string();
                    let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp") {
                        items.push(MediaItem {
                            src:        format!("/assets/images/{}/{}", cat_name, fname),
                            category:   cat_name.clone(),
                            label:      category_label(&cat_name).to_string(),
                            media_type: "image".to_string(),
                        });
                    }
                }
            }
        }
    }

    if let Ok(cats) = std::fs::read_dir("static/assets/videos") {
        for cat in cats.flatten() {
            let folder = cat.file_name().to_string_lossy().to_string();
            let category = if folder == "reels" { "travel".to_string() } else { folder.clone() };
            if let Ok(files) = std::fs::read_dir(cat.path()) {
                for file in files.flatten() {
                    let fname = file.file_name().to_string_lossy().to_string();
                    let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();
                    if matches!(ext.as_str(), "mp4" | "mov" | "webm") {
                        items.push(MediaItem {
                            src:        format!("/assets/videos/{}/{}", folder, fname),
                            category:   category.clone(),
                            label:      category_label(&category).to_string(),
                            media_type: "video".to_string(),
                        });
                    }
                }
            }
        }
    }

    axum::Json(items)
}

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
        .route("/api/recent", get(recent_list))
        .route("/api/media", get(media_list))
        .route("/api/contact", post(contact))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
