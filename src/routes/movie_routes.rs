use crate::{models::movie::Movie, sdbrepo::sdbrepo::SDBrepo};
use warp::Filter;

pub async fn get_movie_from_db(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let db = SDBrepo::new().await.unwrap().db;
    let query = format!("SELECT * FROM type::table($table) WHERE imdbId = '{}';", id);
    let mut response: surrealdb::Response =
        db.query(&query).bind(("table", "movie")).await.unwrap();

    let movie_object: Option<Movie> = response.take(0).unwrap();
    Ok(warp::reply::json(&movie_object.unwrap()))
}

pub async fn get_movies_from_db() -> Result<impl warp::Reply, warp::Rejection> {
    let db = SDBrepo::new().await.unwrap().db;
    let query = format!("SELECT * FROM type::table($table);");
    let mut response: surrealdb::Response =
        db.query(&query).bind(("table", "movie")).await.unwrap();

    let movie_object: Vec<Movie> = response.take(0).unwrap();
    Ok(warp::reply::json(&movie_object))
}

// A route to handle GET requests for a specific post
pub fn get_movie() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "movies" / String)
        .and(warp::get())
        .and_then(get_movie_from_db)
}
pub fn get_movies() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "movies")
        .and(warp::get())
        .and_then(get_movies_from_db)
}
