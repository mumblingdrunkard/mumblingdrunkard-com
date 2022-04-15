use std::{ffi::OsStr, path::PathBuf, time::Duration};

use rocket::{
    fs::{relative, FileServer, NamedFile},
    response::content::Json,
    tokio::time::sleep,
};

#[macro_use]
extern crate rocket;

#[get("/api/<path..>")]
async fn api(path: PathBuf) -> Json<String> {
    let mut components = path.as_path().iter();
    match components
        .next()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("")
    {
        "delay" => api_delay(&mut components).await,
        "" => Json(r#"{ "status": "error", "message": "no route supplied" }"#.to_string()),
        _ => Json(r#"{ "status": "error", "message": "unmapped route" }"#.to_string()),
    }
}

async fn api_delay<'a>(path: &mut std::path::Iter<'a>) -> Json<String> {
    if let Some(v) = path.next() {
        if let Ok(seconds) = v.to_str().unwrap_or("").parse::<u64>() {
            sleep(Duration::from_secs(seconds)).await;
            Json(r#"{ "status": "success", "message": "Test"}"#.to_string())
        } else {
            Json(r#"{ "status": "error", "message": "delay must be an integer" }"#.to_string())
        }
    } else {
        Json(r#"{ "status": "error", "message": "no delay supplied" }"#.to_string())
    }
}

#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open("app/dist/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api, index])
        .mount("/static", FileServer::from(relative!("app/dist")).rank(1))
        .mount("/doc", FileServer::from(relative!("doc")).rank(1))
}
