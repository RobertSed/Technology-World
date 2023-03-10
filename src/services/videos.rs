use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use actix_web::{self, web::{self}, Responder};

use crate::{database, services::youtube::{self, VideoData}};

/** This function fetches all the videos by their course id.
 
    Returns a JSON response.

*/
pub async fn get_videos_from_id(connection_pool: web::Data<database::MySQLPool>, course_id: web::Path<i32>) -> impl Responder{
    use database::schema::videos::dsl::*;

    // clone the course_id from the path defined by the user.
    let course_id = course_id.clone();

    // Create a connection and return the videos filtered by their course id
    let videos_rtn = web::block(move || {
        let connection = &mut connection_pool.get().unwrap();

        videos.filter(courseID.eq(course_id)).load::<database::models::Video>(connection).unwrap()
    }).await.unwrap();

    // Create a mutable vector to store the video data.
    let mut video_data: Vec<VideoData> = Vec::new();

    // loop each video in the database.
    for vids in videos_rtn {
        // check if the video is valid, if not skip it.
        let vid_data = match youtube::get_video_data(vids.video_id).await{
            Ok(data) => data,
            Err(_) => continue,
        };
        // push the valid data onto the vector.
        video_data.push(vid_data);
    }

    // return the data as a JSON.
    web::Json(video_data)
}