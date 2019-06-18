#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::{self, Read, Write};
use std::fs::File;

use rocket_contrib::templates::Template;

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
struct Page {
    title: String,
    body: String,
}

impl Page {
    fn blank(title: String) -> Page {
        Page {
            title,
            body: String::new()
        }
    }

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

#[get("/view/<title>")]
fn view(title: String) -> io::Result<Template> {
    let page = Page::load(title)?;
    let res = Template::render("view", page);
    Ok(res)
}

#[get("/edit/<title>")]
fn edit(title: String) -> Template {
    let page = Page::load(title.clone())
        .unwrap_or(Page::blank(title));
    Template::render("edit", page)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, view, edit])
        .attach(Template::fairing())
        .launch();
}
