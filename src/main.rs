use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use serde::{Deserialize, Serialize};
use std::env;
use urlencoding;
use walkdir::WalkDir;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};

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
    file_sort: Option<String>,
}

#[get("/api/{tail:.*}")]
async fn api(req: HttpRequest) -> impl Responder {
    let mut ret = ApiRet {
        files: Vec::new(),
        dirs: Vec::new(),
        current_dir: "".to_string(),
        filetypes_present: Vec::new(),
    };
    let data_dir = req
        .app_data::<web::Data<AppConf>>()
        .unwrap()
        .data_dir
        .clone();

    let mut search_path = data_dir.clone();
    let user_path = urlencoding::decode(req.path())
        .unwrap()
        .into_owned()
        .to_string();
    ret.current_dir = user_path
        .to_string()
        .strip_prefix("/api")
        .unwrap()
        .to_string();
    search_path.push_str(match user_path.strip_prefix("/api") {
        Some(str) => str,
        None => return HttpResponse::NotFound().body(String::from("not found")),
    });
    let query = web::Query::<ApiParams>::from_query(req.query_string()).unwrap();

    let dir_walker = match query.dir_depth {
        Some(d) => WalkDir::new(search_path.clone())
            .sort_by(|a, b| {
                a.file_name()
                    .to_ascii_lowercase()
                    .cmp(&b.file_name().to_ascii_lowercase())
            })
            .max_depth(d),
        None => WalkDir::new(search_path.clone()).sort_by(|a, b| {
            a.file_name()
                .to_ascii_lowercase()
                .cmp(&b.file_name().to_ascii_lowercase())
        }),
    };

    for entry in dir_walker {
        match entry {
            Ok(actual_entry) => {
                if actual_entry.file_type().is_dir() {
                    ret.dirs.push(
                        actual_entry
                            .path()
                            .to_string_lossy()
                            .to_string()
                            .strip_prefix(&data_dir)
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    continue;
                }
            }
            Err(error) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error returned from walkdir: {}", error));
            }
        }
    }

    let media_extensions: Vec<String> = match &query.media_extensions {
        Some(s) => s.split(",").map(|x| x.to_string()).collect(),
        None => vec![],
    };
    if media_extensions.len() == 0 {
        // early return if no files need to be searched
        return HttpResponse::Ok().json(ret);
    }

    let file_walker = WalkDir::new(search_path.clone()).sort_by(|a, b| {
        a.file_name()
            .to_ascii_lowercase()
            .cmp(&b.file_name().to_ascii_lowercase())
    });

    for entry in file_walker {
        match entry {
            Ok(actual_entry) => {
                if actual_entry.file_type().is_dir() {
                    continue;
                } else {
                    let ext = match actual_entry.path().extension() {
                        Some(ext) => ext.to_string_lossy().to_string().to_ascii_lowercase(),
                        None => String::from("nonexistent"),
                    };
                    if media_extensions.contains(&ext) {
                        ret.files.push(
                            actual_entry
                                .path()
                                .to_string_lossy()
                                .to_string()
                                .strip_prefix(&data_dir)
                                .unwrap()
                                .to_string(),
                        );
                    }
                }
            }
            Err(error) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error returned from walkdir: {}", error));
            }
        }
    }
    let file_sort: Vec<String> = match &query.file_sort {
        Some(s) => s.split(",").map(|x| x.to_string()).collect(),
        None => vec![],
    };
    // the only existing file sort is random or alphabetical
    match file_sort.len(){
        0 => (),
        1 => {
            ret.files.shuffle(&mut thread_rng())
        },
        _ => (),
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
    print!("serving {} on port: {}", ddir, 8080);
    let _ = io::stdout().flush();
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(fs::Files::new("/data", conf.data_dir.clone()).show_files_listing())
            .service(api)
            .app_data(web::Data::new(conf.clone()))
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
