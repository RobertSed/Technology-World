use std::env;

use actix_web::{self, HttpServer, web::{self, Data}};

pub mod database;
pub mod services;


// This is the main function that starts the web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a constant string for the base of the API,
    // if a new API is created, a new base can be made without interfering with the old API.
    const V1_BASE: &str = "/api/v1";

    // Get the IP and Port from the environment variables.
    let ip_port = env::var("IP_PORT").expect("No IP Port Configured.");

    // Get the connection pool from the database module.
    let pool = database::get_connection_pool();

    // Start the web server.
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                actix_web::web::scope(V1_BASE)
                    .route("/courses", web::get().to(services::courses::get_all_courses))
                    .route("/courses/{course_id}", web::get().to(services::courses::get_course_by_id))
                    .route("/courses/{course_id}/data", web::get().to(services::courses::get_course_data))
                    .route("/videos/{course_id}", web::get().to(services::videos::get_videos_from_id))
            )
    })
    .bind(ip_port)
    .unwrap()
    .run()
    .await
}