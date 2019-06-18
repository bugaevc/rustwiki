#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::{self, Read, Write};
use std::fs::File;

use rocket::response::content::Html;

#[derive(Debug, PartialEq, Eq)]
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
fn view(title: String) -> io::Result<Html<String>> {
    let page = Page::load(title)?;
    let res = format!("<h1>{}</h1><div>{}</div>", page.title, page.body);
    Ok(Html(res))
}

#[get("/edit/<title>")]
fn edit(title: String) -> Html<String> {
    let page = Page::load(title.clone())
        .unwrap_or(Page::blank(title));
    let res = format!("
        <h1>Editing {title}</h1>
        <form action=\"/save/{title}\" method=\"POST\">
            <textarea name=\"body\">{body}</textarea><br>
            <input type=\"submit\" value=\"Save\">
        </form>", title = page.title, body = page.body);
    Html(res)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, view, edit])
        .launch();
}
