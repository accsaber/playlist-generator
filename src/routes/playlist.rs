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

use mysql_async::prelude::*;

pub async fn generate_range_route(from: i64, to: i64, pool: Db) -> warp::reply::Response {
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

    let songs = include_str!("playlist.sql")
        .with((from.to_string(), to.to_string()))
        .map(
            &mut conn,
            |(difficulty, song_hash, song_name): (String, String, String)| crate::playlist::Song {
                song_name,
                hash: song_hash,
                difficulties: vec![Difficulty {
                    characteristic: "Standard".to_string(),
                    name: difficulty,
                }],
            },
        )
        .await;

    drop(conn);

    match songs {
        Ok(songs) => warp::reply::json(&Playlist {
            image: None,
            playlist_title: format!("AccSaber maps ranked between {} and {}", &from, &to)
                .to_string(),
            playlist_author: "AccSaber".to_string(),
            songs,
            sync_url: Some(format!(
                "https://api.accsaber.com/ranked-maps/between/{}/{}",
                from.timestamp_millis(),
                to.timestamp_millis()
            )),
        })
        .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
