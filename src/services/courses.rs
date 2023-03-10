use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use actix_web::{self, web::{self}, Responder};

use crate::{database, services::youtube};

/** This function fetches all the courses from the database.
 
    Returns a JSON response.

*/
pub async fn get_all_courses(connection_pool: web::Data<database::MySQLPool>) -> impl Responder{
      use database::schema::courses::dsl::*;

      // Create a connection to the database and fetch all courses.
      let course = web::block(move || {
            let connection = &mut connection_pool.get().unwrap();

            courses.load::<database::models::Course>(connection).unwrap()
      }).await.unwrap();

      // Return the courses as JSON
      web::Json(course)
}

/** This function fetches all the courses by their ID.
 
    Returns a JSON response.

*/
pub async fn get_course_by_id(connection_pool: web::Data<database::MySQLPool>, course_id: web::Path<i32>) -> impl Responder{
      use database::schema::courses::dsl::*;

      // Create a clone of the data, for memory safety purposes.
      let course_id = course_id.clone();

      // Create a connection to the database and fetch all courses by the requested ID.
      let course = web::block(move || {
            let connection = &mut connection_pool.get().unwrap();

            courses.filter(courseID.eq(course_id)).load::<database::models::Course>(connection).unwrap()
      }).await.unwrap();

      // Return the courses as JSON
      web::Json(course)
}

/** This function fetches all the data related to the course
 
    Returns a JSON response.

*/
pub async fn get_course_data(connection_pool: web::Data<database::MySQLPool>, course_id: web::Path<i32>) -> impl Responder{
      use database::schema::courses::dsl::*;
      use database::schema::videos::dsl::videos;

      // Create a clone of the data, for memory safety purposes.
      let course_id = course_id.clone();

      // Return a tuple of the channel ID and the videos related to the course.
      let course_info = web::block(move || {
            let connection = &mut connection_pool.get().unwrap();
            
            let course = courses.filter(courseID.eq(course_id)).first::<database::models::Course>(connection).unwrap();

            let course_videos = videos.filter(database::schema::videos::dsl::courseID.eq(course_id)).load::<database::models::Video>(connection).unwrap();

            (course.course_leader_channel, course_videos)
      }).await.unwrap();

      // clone the first instance in the tuple
      let channel_id = course_info.0.clone();
      
      // Create a mutable variable to store the total amount of views.
      let mut total_views = 0;

      // loop through each video in the course and add together their views.
      for video in course_info.1 {
           total_views += youtube::get_views_from_id(video.video_id).await;
      }

      // Create a structure of the data.
      let course_data = youtube::CourseData{
            channel_name: youtube::get_channel_name(&channel_id).await,
            channel_thumbnail: youtube::get_channel_thumbnail(&channel_id).await,
            course_total_views: total_views
      };

      // Return the structure as JSON
      web::Json(course_data)
}