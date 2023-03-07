use diesel;

diesel::table! {
    courses (courseID) {
        courseID -> Integer,
        courseName -> Varchar,
        courseDescription -> Varchar,
        courseDuration -> Integer,
        courseImage -> Varchar,
        courseLeaderChannel -> Varchar,
        courseAccessLevel -> Integer,
    }
}

diesel::table! {
    users (userID) {
        userID -> Integer,
        userDisplayName -> Varchar,
        userEmail -> Varchar,
        userPassword -> Varchar,
        userLevel -> Integer,
        dateCreation -> Integer,
        accountStatus -> Integer,
    }
}

diesel::table! {
    videos (videoID) {
        videoID -> Varchar,
        courseID -> Integer,
    }
}

diesel::joinable!(videos -> courses (courseID));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    users,
    videos,
);
