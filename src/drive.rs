use crate::{http::GoogleHttp, error::Error};
use reqwest::multipart;
use serde_json::Value;

// const GOOGLE_API_BASE_URL: &str = "https://www.googleapis.com";
const DRIVE_API_BASE_URL: &str = "https://www.googleapis.com/drive/v3";
const DRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";

pub async fn list_files(client: &GoogleHttp) -> Result<Value, Error> {
    let url = format!("{}/files", DRIVE_API_BASE_URL);
    let resp = client.get(&url).await?;
    let json: Value = resp.json().await?;
    Ok(json)
}

pub async fn check_folder(client: &GoogleHttp, folder_name: &str) -> Result<Option<String>, Error> {
    let url = format!(
        "{}/files?q=name='{}' and mimeType='application/vnd.google-apps.folder' and trashed=false",
        DRIVE_API_BASE_URL, folder_name
    );

    let resp = client.get(&url).await?;
    let json: Value = resp.json().await?;
    
    if let Some(files) = json["files"].as_array() {
        if !files.is_empty() {
            if let Some(folder_id) = files[0]["id"].as_str() {
                return Ok(Some(folder_id.to_string()));
            }
        }
    }
    Ok(None)
}

pub async fn create_folder(
    client: &GoogleHttp,
    folder_name: &str,
    parent_folder_id: Option<String>,
) -> Result<Value, Error> {
    let mut metadata = serde_json::json!({
        "name": folder_name,
        "mimeType": "application/vnd.google-apps.folder"
    });

    if let Some(parent_id) = parent_folder_id {
        metadata["parents"] = serde_json::json!([parent_id]);
    }

    let url = format!("{}/files", DRIVE_API_BASE_URL);

    let resp = client.post_json(&url, &metadata).await?;
    let json: Value = resp.json().await?;
    Ok(json)
}


pub async fn upload_file(
    client: &GoogleHttp,
    file_path: &str,
    mime_type: &str,
    parent_folder_id: Option<&str>,
) -> Result<Value, Error> {
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(Error::InvalidFileName)?;

    let mut metadata = serde_json::json!({
        "name": file_name,
    });

    if let Some(parent_id) = parent_folder_id {
        metadata["parents"] = serde_json::json!([parent_id]);
    }

    let metadata_part = multipart::Part::text(metadata.to_string())
        .mime_str("application/json; charset=UTF-8")?;

    let file_bytes = tokio::fs::read(file_path).await?;
    let file_part = multipart::Part::bytes(file_bytes)
        .file_name(file_name.to_string())
        .mime_str(mime_type)?;
    
    let form = multipart::Form::new()
        .part("metadata", metadata_part)
        .part("file", file_part);

    let url = format!(
        "{}?uploadType=multipart&fields=id,name,mimeType",
        DRIVE_UPLOAD_URL
    );

    let resp = client.post_multipart(&url, form).await?;

    let json: Value = resp.json().await?;
    
    Ok(json)
}