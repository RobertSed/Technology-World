use actix_web::{self, HttpServer, web::{self, Data}};


pub mod database;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const V1_BASE: &str = "/api/v1";

    let pool = database::get_connection_pool();

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                actix_web::web::scope(V1_BASE)
                    .route("/courses", web::get().to(services::courses::get_all_courses))
                    .route("/videos", web::get().to(services::courses::get_all_videos))
                    .route("/videos/{course_id}", web::get().to(services::courses::get_videos_from_id))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}