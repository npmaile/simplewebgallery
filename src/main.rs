use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Serialize)]
struct ApiRet {
    files: Vec<String>,
    dirs: Vec<String>,
    parent_dir: String,
    curent_dir: String,
    filetypes_present: Vec<String>,
}

#[derive(Clone)]
struct AppConf {
    data_dir: String,
}

#[get("/api/{tail:.*}")]
async fn api(req: HttpRequest) -> impl Responder {
    let mut search_path = req
        .app_data::<web::Data<AppConf>>()
        .unwrap()
        .data_dir
        .clone();
    let user_path = req.path();
    search_path.push_str(user_path.strip_prefix("/api").unwrap());
    for entry in WalkDir::new(search_path.clone()){
        let entry = entry.unwrap();
        println!("{}",entry.path().display())

    };
    
    HttpResponse::Ok().body(format!("searching path: {}", search_path))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ddir = std::env::var("DATA_DIR");
    let ddir: String = match ddir {
        Ok(x) => x,
        Err(_) => "data".to_string(),
    };
    let conf = AppConf {
        data_dir: ddir.clone(),
    };
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(fs::Files::new("/data", conf.data_dir.clone()))
            .service(api)
            .app_data(web::Data::new(conf.clone()))
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
