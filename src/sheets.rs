use crate::{http::GoogleHttp, error::Error};
use serde_json::{Value, json};

// const SHEETS_API_BASE_URL: &str = "https://sheets.googleapis.com/v4";
const SHEETS_API_URL: &str = "https://sheets.googleapis.com/v4/spreadsheets";

pub async fn get_sheets(client: &GoogleHttp) -> Result<Value, Error> {
    let url = "https://www.googleapis.com/drive/v3/files\
?q=mimeType='application/vnd.google-apps.spreadsheet'\
&fields=files(id,name)";
    let resp = client.get(url).await?;
    let json: Value = resp.json().await?;
    Ok(json)
}

pub async fn find_spreadsheet_id_by_name(
    client: &GoogleHttp,
    name: &str,
) -> Result<String, Error> {
    let url = "https://www.googleapis.com/drive/v3/files?q=mimeType='application/vnd.google-apps.spreadsheet'&fields=files(id,name)";
    let resp = client.get(url).await?;
    let json: Value = resp.json().await?;

    if let Some(files) = json["files"].as_array() {
        for file in files {
            if let Some(name_file) = file["name"].as_str() {
                if name == name_file {
                    if let Some(id) = file["id"].as_str() {
                        return Ok(id.to_string());
                    }
                }
            }
        }
    }

    Err(Error::Google(format!("Spreadsheet with name '{}' not found", name)))
}

pub async fn get_spreadsheet(client: &GoogleHttp, spreadsheet_id: &str) -> Result<Value, Error> {
    let url = format!("{}/{}", SHEETS_API_URL, spreadsheet_id);
    let resp = client.get(&url).await?;
    Ok(resp.json().await?)
}

pub async fn get_headers(
    client: &GoogleHttp,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<Vec<String>, Error> {
    let url = format!(
        "{}/{}/values/'{}'!1:1",
        SHEETS_API_URL,
        spreadsheet_id,
        sheet_name
    );

    let resp = client.get(&url).await?;
    let json: Value = resp.json().await?;

    let headers = json["values"]
        .get(0)
        .and_then(|v| v.as_array())
        .unwrap_or(&vec![])
        .iter()
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect();


    Ok(headers)
}


pub async fn append_row(
    client: &GoogleHttp,
    spreadsheet_id: &str,
    sheet_name: &str,
    headers: &[String],
    row: &std::collections::HashMap<String, String>,
) -> Result<(), Error> {

    let values: Vec<String> = headers
        .iter()
        .map(|h| row.get(h).cloned().unwrap_or_default())
        .collect();

    let last_col = column_letter(headers.len());

    let url = format!(
        "{}/{}/values/'{}'!A:{}:append",
        SHEETS_API_URL,
        spreadsheet_id,
        sheet_name,
        last_col
    );
    println!("{:?}", &values);
    client
        .post_json(
            &format!(
                "{}?valueInputOption=RAW&insertDataOption=INSERT_ROWS",
                url
            ),
            &json!({ "values": [values] }),
        )
        .await?;

    Ok(())
}

pub fn column_letter(mut index: usize) -> String {
    let mut col = String::new();
    while index > 0 {
        index -= 1;
        col.insert(0, ((index % 26) as u8 + b'A') as char);
        index /= 26;
    }
    col
}
