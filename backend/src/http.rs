use std::net::SocketAddr;

use anyhow::Context;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json, Router};
use axum_extra::extract::WithRejection;
use openssl::symm::{encrypt, Cipher};
use serde_json::json;
use tower::ServiceBuilder;

use crate::{
    error::{Error, ExtractorError, ResponseResult},
    APIContext,
};

async fn version() -> impl IntoResponse {
    Json(json!({ "version": env!("CARGO_PKG_VERSION") }))
}

#[derive(serde::Deserialize)]
struct RegisterInterestPayload {
    name: String,
    email: String,
}

async fn register(
    context: Extension<APIContext>,
    WithRejection(Json(payload), _): WithRejection<Json<RegisterInterestPayload>, ExtractorError>,
) -> ResponseResult<impl IntoResponse> {
    let cipher = Cipher::aes_128_cbc();
    let key = context.encryption_key.as_bytes();
    let mut iv: [u8; 16] = [0; 16];
    openssl::rand::rand_bytes(&mut iv).unwrap();

    let name = hex::encode(encrypt(cipher, key, Some(&iv), payload.name.as_bytes()).unwrap());
    let email = hex::encode(encrypt(cipher, key, Some(&iv), payload.email.as_bytes()).unwrap());
    let db_iv = hex::encode(iv);

    sqlx::query!(
        "INSERT INTO users (name, email, iv) VALUES ($1, $2, $3)",
        name,
        email,
        db_iv
    )
    .execute(&context.db)
    .await?;

    return Ok(StatusCode::NO_CONTENT);
}

fn api_router() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/", axum::routing::get(version))
            .route("/registerinterest", axum::routing::post(register))
            .fallback(|| async {
                Error {
                    status: 404,
                    message: "Not Found".to_string(),
                }
                .into_response()
            }),
    )
}

pub async fn serve(context: APIContext) -> anyhow::Result<()> {
    let application = api_router().layer(ServiceBuilder::new().layer(Extension(context)));

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 5000)))
        .serve(application.into_make_service())
        .await
        .context("Failed to start server")
}
