use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use warp::{
    hyper::StatusCode,
    reply::{json, with_status},
    Reply,
};

use crate::{
    db::Db,
    playlist::{Difficulty, Playlist},
};

use mysql_async::{prelude::*, Row};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListingEntry {
    pub date_ranked: DateTime<Utc>,
    pub song_name: String,
    pub level_author_name: String,
    pub category_name: String,
    pub complexity: f64,
}

pub async fn generate_listing_route(from: i64, to: i64, pool: Db) -> warp::reply::Response {
    let mut conn = match pool.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            return with_status(json(&e.to_string()), StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
    };
    let from: DateTime<Utc> = match Utc.timestamp_millis_opt(from) {
        chrono::LocalResult::Single(v) => v,
        _ => {
            return warp::reply::with_status(
                json(&"Invalid beginning date"),
                StatusCode::BAD_REQUEST,
            )
            .into_response()
        }
    };

    let to: DateTime<Utc> = match Utc.timestamp_millis_opt(to) {
        chrono::LocalResult::Single(v) => v,
        _ => {
            return warp::reply::with_status(json(&"Invalid end date"), StatusCode::BAD_REQUEST)
                .into_response()
        }
    };

    let songs = include_str!("listing.sql")
        .with((from.to_string(), to.to_string()))
        .map(&mut conn, |row: Row| {
            serde_json::json! ({
                // "date_ranked": row.get::<async, &str>("date_ranked"),
                "songName": row.get::<String, &str>("song_name"),
                "levelAuthorName": row.get::<String, &str>("level_author_name"),
                "categoryName": row.get::<String, &str>("category_display_name"),
                "difficulty": row.get::<String, &str>("difficulty"),
                "complexity": row.get::<f64, &str>("complexity"),
                "beatSaverKey": row.get::<String, &str>("beat_saver_key"),
            })
        })
        .await;

    drop(conn);

    match songs {
        Ok(songs) => warp::reply::json(&songs).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
