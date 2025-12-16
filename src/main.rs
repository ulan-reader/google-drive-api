mod auth;
mod error;
mod token_cache;
mod http;
mod drive;

#[tokio::main]
async fn main() {
    let cache  = token_cache::SharedTokenChache::new("path_to_secret".to_string());
    let token = cache.get_token().await.unwrap();
    println!("Token: {}", token);

    // Можно использовать токен для запроса
    let client = http::GoogleHttp::new(token);
    
    let list = drive::list_files(&client).await.unwrap();
    println!("Files list: {:#?}", list);
    
    let check_file = drive::check_folder(&client, "Shared drive").await.unwrap();
    if check_file.is_none() {
        // let folder = drive::create_folder(&client, "Shared drive", None).await.unwrap();
        // println!("Created folder: {:#?}", folder);
    } else {
        let folder_id = check_file.unwrap();
        let folder = drive::create_folder(&client, "Check drive api", Some(folder_id)).await.unwrap();
        // println!("Folder exists: {:#?}", folder);
        let folder_id = folder["id"].as_str().unwrap();
        let upload = drive::upload_file(&client, "result.gif", "image/gif", Some(&folder_id)).await.unwrap();
        println!("Uploaded file: {:#?}", upload);
    }
}