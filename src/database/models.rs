#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use crate::database::schema::*;

use serde::{Serialize, Deserialize};


#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(course_id))]
pub struct Course {
    pub course_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_duration: i32,
    pub course_image: String,
    pub course_leader_channel: String,
    pub course_access_level: i32,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub user_display_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_level: i32,
    pub date_creation: i32,
    pub account_status: i32,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(video_id))]
pub struct Video {
    pub video_id: String,
    pub course_id: i32,
}

