#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::{self, Read, Write};
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
struct Page {
    title: String,
    body: String,
}

impl Page {
    fn load(title: String) -> io::Result<Page> {
        let file_name = format!("{}.txt", title);
        let mut file = File::open(file_name)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;
        Ok(Page { title, body })
    }

    fn save(&self) -> io::Result<()> {
        let file_name = format!("{}.txt", self.title);
        let mut file = File::create(file_name)?;
        write!(file, "{}", self.body)
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
