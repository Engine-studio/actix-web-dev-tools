use actix_web::{
    web, http, dev, guard, 
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder, 
};
use serde::Deserialize;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::error::ApiError;

use actix_multipart_derive::MultipartForm;
use chrono::Datelike;

#[derive(MultipartForm,Debug,Clone,Default)]
pub struct Form {
    ext: String,
    file: web::BytesMut,
}

use rand::RngCore;
use data_encoding::BASE64URL;
use crate::db::PendingFile;

pub async fn load(
    data: Form,
    conn: web::Data<DbPool>, 
    _req: HttpRequest,
) -> Result<HttpResponse,ApiError> {
    use std::io::Write;
    let mut fkey = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut fkey);
    let fkey: String = BASE64URL.encode(&fkey);
    let date = chrono::Utc::now();
    let path = format!(
        "static/{}/{}/{}/{}.{}",
        date.year(),
        date.month(),
        date.day(),
        fkey,data.ext);
    // TODO: if hash already exists, recreate
    println!("saved to {}",path);

    {
    let path_t = std::path::PathBuf::from(&path);
    let prefix = path_t.parent().unwrap().to_owned();
    web::block(move || std::fs::create_dir_all(prefix)).await.expect("err cr dir");
    }
    let path_t = path.clone();
    let mut file = web::block(|| std::fs::File::create(path_t))
        .await
        .expect("Error creating file");

    web::block(move || file.write_all(&data.file)).await.expect("error writing file");

    use std::time::Duration;
    use futures::future::FutureExt;
    let timer = futures_timer::Delay::new(Duration::from_secs(60));
    let thread_conn = conn.get()?;
    let path_t = path.clone();
    PendingFile::new(path.clone(),&thread_conn).await?;
    println!("after db");
    actix_web::rt::spawn(async move {
        timer.await;
        println!("TICK MAFAKA");
        if PendingFile::was_not_verifyed(&path_t, &thread_conn).await.expect("db err pending") {
            println!("in");
            //web::block(|| std::fs::remove_file(path_t))
            //    .await
            //    .expect("error removing file");
        }
    });
    Ok(HttpResponse::Ok().json(
            json!({
                "code":200,
                "file_path": path,
            })
    ))
}

#[derive(Deserialize,Debug)]
pub struct Files {
    urls: Vec<String>,
}

pub async fn verify(
    form: web::Json<Files>,
    conn: web::Data<DbPool>, 
) -> Result<HttpResponse,ApiError> {
    let conn = conn.get()?;
    let data = form.into_inner();
    PendingFile::verify(data.urls, &conn).await?;
    Ok(HttpResponse::Ok().json(json!({"code":200})))
}

pub async fn delete_files(
    form: web::Json<Files>,
    conn: web::Data<DbPool>, 
) -> Result<HttpResponse,ApiError> {
    let conn = conn.get()?;
    actix_web::rt::spawn(async move {
        for f in form.into_inner().urls {
            web::block(|| std::fs::remove_file(f))
                .await
                .expect("error removing file");
        }
    });
    Ok(HttpResponse::Ok().json(json!({"code":200})))
}
