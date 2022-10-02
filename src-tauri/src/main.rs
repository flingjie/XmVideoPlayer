#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod command;
pub mod s;

fn main() {
    use tauri::http::ResponseBuilder;
    // use std::io::prelude::*;

    tauri::Builder::default()
        // register_uri_scheme_protocol 不支持返回数据流？？？
        .register_uri_scheme_protocol("stream", move |_app, request| {
            let mut response = ResponseBuilder::new();
            // get the wanted path
            #[cfg(target_os = "windows")]
            let path = request.uri().strip_prefix("stream://localhost/").unwrap();
            #[cfg(not(target_os = "windows"))]
            let path = request.uri().strip_prefix("stream://").unwrap();
            let path = percent_encoding::percent_decode(path.as_bytes())
                .decode_utf8_lossy()
                .to_string();

            // debug log
            println!("current request.uri(): {:#?}", request.uri());
            println!("current web request url: {:#?}", &path);

            response = response
                .header("Connection", "Keep-Alive")
                .header("Access-Control-Allow-Origin", "*")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", 3)
                .header("Content-Range", "bytes 0-3/3");
            response.mimetype("video/mp4").status(206).body(vec![0])
        })
        .invoke_handler(tauri::generate_handler![
            command::normal::greet,
            command::splashscreen::close_splashscreen,
            command::media::video_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
