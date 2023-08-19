use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use serde::{Deserialize, Serialize};
use std::env;
use walkdir::WalkDir;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Serialize)]
struct ApiRet {
    files: Vec<String>,
    dirs: Vec<String>,
    current_dir: String,
    filetypes_present: Vec<String>,
}

#[derive(Clone)]
struct AppConf {
    data_dir: String,
}

#[derive(Deserialize)]
struct ApiParams {
    dir_depth: Option<usize>,
    media_extensions: Option<String>,
}

#[get("/api/{tail:.*}")]
async fn api(req: HttpRequest) -> impl Responder {
    let mut ret = ApiRet {
        files: Vec::new(),
        dirs: Vec::new(),
        current_dir: "".to_string(),
        filetypes_present: Vec::new(),
    };
    let mut search_path = req
        .app_data::<web::Data<AppConf>>()
        .unwrap()
        .data_dir
        .clone();
    let user_path = req.path();
    search_path.push_str(match user_path.strip_prefix("/api") {
        Some(str) => str,
        None => return HttpResponse::NotFound().body(String::from("not found")),
    });
    ret.current_dir = search_path.to_string();
    let query = web::Query::<ApiParams>::from_query(req.query_string()).unwrap();

    let dir_walker = match query.dir_depth {
        Some(d) => WalkDir::new(search_path.clone()).max_depth(d),
        None => WalkDir::new(search_path.clone()),
    };

    let media_extensions: Vec<String> = match &query.media_extensions{
        Some(s) => s.split(",").map(|x| x.to_string()).collect(),
        None => vec![] 
    };

    for entry in dir_walker {
        match entry {
            Ok(actual_entry) => {
                if actual_entry.file_type().is_dir() {
                    ret.dirs
                        .push(actual_entry.path().to_string_lossy().to_string());
                } else {
                    let ext = match actual_entry.path().extension() {
                        Some(ext) => ext.to_string_lossy().to_string(),
                        None => String::from("nonexistent"),
                    };
                    if media_extensions.contains(&ext) {
                        ret.files
                            .push(actual_entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
            Err(error) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error returned from walkdir: {}", error));
            }
        }
    }

    HttpResponse::Ok().json(ret)
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
