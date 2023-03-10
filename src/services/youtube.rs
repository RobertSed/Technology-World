use ytextract::{self, video, channel};
use serde;

use rustube::{self, VideoFetcher, Id};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CourseData{
    pub channel_name: String,
    pub channel_thumbnail: String,
    pub course_total_views: u64
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VideoData{
    pub channel_thumbnail: String,
    pub channel_name: String,
    pub video_title: String,
    pub video_description: String,
    pub video_views: u64,
    pub video_id: String,
    pub video_thumbnail: String
}

/** This function get the views of the video
 
    Returns the views as a u64 datatype.

*/
pub async fn get_views_from_id(video_id: String) -> u64{
    // Convert the video id to a rustube::Id datatype.
    let video_id = Id::from_string(video_id).unwrap();

    // Fetch the video data
    let descrambler = VideoFetcher::from_id(video_id.into_owned()).unwrap().fetch().await.unwrap();

    // Return the views
    descrambler.video_details().view_count
}

/** This function get the thumbnail of the YouTube channel.
 
    Returns the thumbnail URL as a String.

*/
pub async fn get_channel_thumbnail(channel_id: &String) -> String{
    // Get the channel id then parse from a string to channel::Id datatype.
    let channel_id: channel::Id = channel_id.clone().parse().unwrap();
    
    // Create a new client to access YouTube.
    let client = ytextract::Client::new();

    // Get the channel data
    let channel = client.channel(channel_id).await.unwrap();

    // Get the thumbnail of the channel
    let thumbnail = channel.avatar().last().unwrap();

    // Return the thumbnail as a String
    thumbnail.url.to_string()
}

/** This function gets the YouTube channel name
 
    Returns the name as a String.

*/
pub async fn get_channel_name(channel_id: &String) -> String{
    // Get the channel id then parse from a string to channel::Id datatype.
    let channel_id: channel::Id = channel_id.clone().parse().unwrap();

    // Create a new client to access YouTube.
    let client = ytextract::Client::new();

    // Get the channel data
    let channel = client.channel(channel_id).await.unwrap();

    // Return the name of the channel
    channel.name().to_string()
}

/** This function gets the video data used for displaying on the website
 
    Returns a VideoData struct.

*/
pub async fn get_video_data(video_id: String) -> Result<VideoData, ytextract::Error>{
    // Create a new client to access YouTube.
    let client = ytextract::Client::new();

    // Get the video id then parse from a string to video::Id datatype.
    let video_id: video::Id = video_id.parse().unwrap();

    // Get the video data + error handling
    let video = match client.video(video_id).await{
        Ok(vids) => vids,
        Err(e) => return Err(e),
    };

    // Get the channel id
    let channel_id = video.channel().id().to_string();

    // Return the video data
    Ok(VideoData{
        channel_thumbnail: get_channel_thumbnail(&channel_id).await,
        channel_name: get_channel_name(&channel_id).await,
        video_title: video.title().to_string(),
        video_description: video.description().to_string(),
        video_views: *&video.views(),
        video_id: video.id().to_string(),
        video_thumbnail: video.thumbnails().last().unwrap().url.to_string()
    })
}