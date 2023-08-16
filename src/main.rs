#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use serde::Serialize;
use std::path::PathBuf;
use walkdir::WalkDir;


const STATIC_PREFIX: &str = "data";

#[derive(Serialize)]
struct ApiRet {
    files: Vec<String>,
    dirs: Vec<String>,
    parent_dir: String,
    curent_dir: String,
}

impl ApiRet {
    fn new(curdir: &PathBuf) -> ApiRet {
        ApiRet {
            files: vec![],
            dirs: vec![],
            parent_dir: match curdir.parent() {
                Some(path) => path.to_string_lossy().to_string(),
                _ => String::from("root"),
            },
            curent_dir: curdir.to_string_lossy().to_string(),
        }
    }
    fn walk(&mut self, path: &PathBuf) {
        
    }
    fn add(&mut self, entry: &mut walkdir::DirEntry) {
        if entry.path().is_dir() {
            self.dirs.push(String::from(entry.path().to_string_lossy()));
        } else if entry.path().is_file() {
            self.files
                .push(String::from(entry.path().to_string_lossy()));
        }
    }
    fn add_dirs(&mut self, entry: &mut walkdir::DirEntry) {
        if entry.path().is_dir() {
            self.dirs.push(String::from(entry.path().to_string_lossy()));
        }
    }
    fn add_files(&mut self, entry: &mut walkdir::DirEntry) {
        if entry.path().is_file() {
            self.files
                .push(String::from(entry.path().to_string_lossy()));
        }
    }
}

#[get("/<path..>")]
fn index(path: PathBuf) -> String {
    let mut fullpath = PathBuf::from(STATIC_PREFIX);
    fullpath = fullpath.join(path);
    print!("{}", fullpath.to_string_lossy());
    let mut listing = ApiRet::new(&fullpath);
    for entry in WalkDir::new(fullpath) {
        let mut entry = match entry {
            Ok(ent) => ent,
            Err(_) => {
                return String::from("error occured");
            }
        };
        listing.add(&mut entry);
    }
    serde_json::to_string(&listing).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from(STATIC_PREFIX))
        .mount("/api/", routes![index])
        .launch();
}
