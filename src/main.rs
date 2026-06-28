use axum::{
    extract::Path,
    routing::get,
    http::StatusCode,
    Json,
    Router,
};
use serde_json::{json, Value};
use std::num::IntErrorKind;

mod core;


async fn is_prime_handler(Path(input): Path<String>) -> Result<Json<Value>, (StatusCode, &'static str)> {
    if input.starts_with('-') {
        return Err((StatusCode::BAD_REQUEST, "Negative numbers are not allowed."));
    }
    
    let number = match input.parse::<u32>() {
        Ok(n) => n,
        Err(e) => match e.kind() {
            IntErrorKind::InvalidDigit => {
                return Err((StatusCode::BAD_REQUEST, "Input contains non-numeric characters."));
            }

            IntErrorKind::PosOverflow => {
                return Err((StatusCode::BAD_REQUEST, "Number is too large for u32."));
            }
            _ => {
                return Err((StatusCode::BAD_REQUEST, "Unknown parsing error."));
            }
        }
    };
    if number > 982_451_653 {
        return Err((StatusCode::BAD_REQUEST, "Your number is too big."));
    };
    
    Ok(Json(json!({"response": core::is_prime(number)})))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{number}", get(is_prime_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");
     
    axum::serve(listener, app)
        .await
        .expect("failed to start server")
}
