// use std::collections::HashMap;
//
// mod auth;
// mod error;
// mod token_cache;
// mod http;
// mod drive;
// mod sheets;
//
// #[tokio::main]
// async fn main() {
//     let cache  = token_cache::SharedTokenChache::new("path_to_secret".to_string());
//     let token = cache.get_token().await.unwrap();
//     // println!("Token: {}", token);
//
//     // Можно использовать токен для запроса
//     let client = http::GoogleHttp::new(token);
//
//
//     // Получаем ID
//     let sheets_id = sheets::find_spreadsheet_id_by_name(&client, "fsekfsf").await.unwrap();
//
//     // Получаем заголовки
//     let headers = sheets::get_headers(&client, &sheets_id, "Лист1").await.unwrap();
//
//     // Формируем строку для записи
//     let mut row = std::collections::HashMap::new();
//     row.insert("МЕНТАЛОКСИН".to_string(), "10".to_string());
//     row.insert("УЛЬТРАФЛЕКС".to_string(), "3".to_string());
//     row.insert("name".to_string(), "Иван".to_string());
//
//     // Добавляем строку
//     sheets::append_row(&client, &sheets_id, "Лист1", &headers, &row).await.unwrap();
//     // println!("{:#?}", shetTable);
//     // let list = drive::list_files(&client).await.unwrap();
//     // println!("Files list: {:#?}", list);
//     //
//     // let check_file = drive::check_folder(&client, "Shared drive").await.unwrap();
//     // if check_file.is_none() {
//     //     // let folder = drive::create_folder(&client, "Shared drive", None).await.unwrap();
//     //     // println!("Created folder: {:#?}", folder);
//     // } else {
//     //     let folder_id = check_file.unwrap();
//     //     let folder = drive::create_folder(&client, "Check drive api", Some(folder_id)).await.unwrap();
//     //     // println!("Folder exists: {:#?}", folder);
//     //     let folder_id = folder["id"].as_str().unwrap();
//     //     let upload = drive::upload_file(&client, "result.gif", "image/gif", Some(&folder_id)).await.unwrap();
//     //     println!("Uploaded file: {:#?}", upload);
//     // }
// }
