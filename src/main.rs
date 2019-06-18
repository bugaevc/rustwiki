#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::{self, Read, Write};
use std::fs::File;

use rocket::request::Form;
use rocket::response::Redirect;
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
fn view(title: String) -> Result<Template, Redirect> {
    if let Ok(page) = Page::load(title.clone()) {
        let res = Template::render("view", page);
        Ok(res)
    } else {
        Err(Redirect::to(uri!(edit: title)))
    }
}

#[get("/edit/<title>")]
fn edit(title: String) -> Template {
    let page = Page::load(title.clone())
        .unwrap_or(Page::blank(title));
    Template::render("edit", page)
}

#[derive(Debug, FromForm)]
struct SaveForm {
    body: String
}

#[post("/save/<title>", data = "<form>")]
fn save(title: String, form: Form<SaveForm>) -> io::Result<Redirect> {
    let form = form.into_inner();
    let page = Page {
        title: title.clone(),
        body: form.body,
    };
    page.save()?;
    Ok(Redirect::to(uri!(view: title)))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, view, edit, save])
        .attach(Template::fairing())
        .launch();
}
