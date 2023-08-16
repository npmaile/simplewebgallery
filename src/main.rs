use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use serde::Serialize;
use std::env;
use std::sync::Arc;
//use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Serialize)]
struct ApiRet {
    files: Vec<String>,
    dirs: Vec<String>,
    parent_dir: String,
    curent_dir: String,
    filetypes_present: Vec<String>,
}

#[get("api")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("weiner")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ddir = std::env::var("DATA_DIR");
    let ddir: String = match ddir {
        Ok(x) => x,
        Err(_) => "data".to_string(),
    };
    let ddir = Arc::new(ddir);
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(fs::Files::new("/data", ddir.to_string()).show_files_listing())
            .service(api)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
