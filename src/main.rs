#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("./dist/index.html")
}

#[get("/public/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./dist/public").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![crate::file, crate::index])
}

fn main() {
    rocket().launch();
}
