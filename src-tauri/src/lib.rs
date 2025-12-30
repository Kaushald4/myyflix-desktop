pub mod app;
pub mod models;
pub mod routes;
pub mod services;
pub mod stream;
pub mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[tauri::command]
async fn get_stream_link(
    id: &str,
    content_type: &str,
    web_url: &str,
    season: Option<u32>,
    episode: Option<u32>,
) -> Result<Option<String>, String> {
    services::extract_service::extract_stream_link(id, content_type, season, episode, web_url)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_stream_link])
        .setup(|_| {
            app::server::start();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running app");
}
