use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use actix_web::{self, web, Responder};
use crate::database;

pub async fn get_all_courses(connection_pool: web::Data<database::MySQLPool>) -> impl Responder{
      let pool = connection_pool.clone();
      use database::schema::courses::dsl::*;

      let connection = &mut pool.get().unwrap();
  
      let courses_rtn = courses.load::<database::models::Course>(connection).unwrap();

      web::Json(courses_rtn)
}

pub async fn get_all_videos(connection_pool: web::Data<database::MySQLPool>) -> impl Responder{
      let pool = connection_pool.clone();
      use database::schema::videos::dsl::*;

      let connection = &mut pool.get().unwrap();
  
      let videos_rtn = videos.load::<database::models::Video>(connection).unwrap();

      web::Json(videos_rtn)
}

pub async fn get_videos_from_id(connection_pool: web::Data<database::MySQLPool>, course_id: web::Path<i32>) -> impl Responder{
      let pool = connection_pool.clone();
      use database::schema::videos::dsl::*;

      let connection = &mut pool.get().unwrap();

      let videos_rtn = videos.filter(courseID.eq(course_id.into_inner())).load::<database::models::Video>(connection).unwrap();

      web::Json(videos_rtn)
}