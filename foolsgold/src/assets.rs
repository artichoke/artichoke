use rocket::http::{ContentType, Status};
use rocket::{get, Response};
use std::borrow::Cow;
use std::io::Cursor;

// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[derive(RustEmbed)]
#[folder = "foolsgold/static/"]
struct Assets;

#[get("/")]
pub fn index<'a>() -> Result<Response<'a>, Status> {
    let html = Assets::get("index.html").ok_or(Status::NotFound)?;
    let response = Response::build()
        .sized_body(Cursor::new(html))
        .header(ContentType::HTML)
        .finalize();
    Ok(response)
}

#[get("/pyrite.jpg")]
pub fn pyrite() -> Result<Vec<u8>, Status> {
    Assets::get("pyrite.jpg")
        .map(Cow::into_owned)
        .ok_or(Status::NotFound)
}

#[get("/resf.png")]
pub fn resf() -> Result<Vec<u8>, Status> {
    Assets::get("resf.png")
        .map(Cow::into_owned)
        .ok_or(Status::NotFound)
}
