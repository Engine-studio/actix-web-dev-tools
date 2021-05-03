use actix_web::{
    web, http, dev, guard, 
    App, HttpResponse, client::Client,
    HttpServer,
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use actix_web::middleware::Logger;
use diesel_migrations::run_pending_migrations;
use serde_json::json;
extern crate env_logger;
use tiny_uploader::routes::{
    load,
    verify,
    delete_files,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => print!("migration success\n"),
        Err(e)=> print!("migration error: {}\n",&e),
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("starting server...");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/uploader")
                    .route("/verify", web::post().to(verify))
                    .route("delete", web::delete().to(delete_files))
                    .route("/load", web::post().to(load))
            )
    })
    .bind("0.0.0.0:8088")?
    .system_exit()
    .run()
    .await
}
