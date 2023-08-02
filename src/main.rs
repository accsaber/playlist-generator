mod config;
mod db;
mod playlist;
mod routes;

use db::Db;
use warp::Filter;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = serde_env::from_env::<config::Config>()?;
    let pool = db::connect(&config).await?;

    let playlist_route = warp::path!("generate" / i64 / i64)
        .and(with_db(pool.clone()))
        .then(routes::generate_range_route);
    let listing_route = warp::path!("listing" / i64 / i64)
        .and(with_db(pool.clone()))
        .then(routes::generate_listing_route);

    warp::serve(playlist_route.or(listing_route))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
