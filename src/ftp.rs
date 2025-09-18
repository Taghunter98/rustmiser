use std::fs::File;
use std::io::Write;
use std::path::Path;

use axum::{Json, extract::Multipart, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

/// Handler stores files on the server from multipart form data.
///
/// ## Errors
///
/// - ['Json<ErrorResponse>'](Json) - JSON error messages returned
///
pub async fn upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<String>), (StatusCode, Json<ErrorResponse>)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Failed to read field: {}", e),
            }),
        )
    })? {
        let name = field
            .file_name()
            .ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Missing file name".to_string(),
                    }),
                )
            })?
            .to_string();

        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to read bytes: {}", e),
                }),
            )
        })?;

        println!("Received file `{}`", name);

        let path = Path::new("../uploads").join(&name);
        let mut file = File::create(&path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Unable to create file: {}", e),
                }),
            )
        })?;

        file.write_all(&data).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Unable to write data: {}", e),
                }),
            )
        })?;
    }

    Ok((StatusCode::OK, Json("File stored successfully".to_string())))
}
