use crate::{
    models::movie::Movie, models::review::Review, models::review::ReviewQuery,
    sdbrepo::sdbrepo::SDBrepo,
};
use uuid::Uuid;
use warp::Filter;

fn generate_unique_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_hyphenated().to_string()
}

pub async fn post_review_to_db(review: ReviewQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let db = SDBrepo::new().await.unwrap().db;
    let id = generate_unique_id();

    let query = format!(
        "SELECT * FROM type::table($table) WHERE imdbId = '{}';",
        review.imdbId
    );
    let mut response: surrealdb::Response =
        db.query(&query).bind(("table", "movie")).await.unwrap();
    let movie_object: Option<Movie> = response.take(0).unwrap();
    //now update the movie with the new review
    let mut reviews = movie_object.clone().unwrap().reviewIds;
    reviews.push(Review {
        id: id.clone(),
        body: review.reviewBody,
    });
    let update_query = format!(
        "UPDATE type::table($table)
        SET reviewIds = $reviewIds
        WHERE imdbId = '{}';",
        review.imdbId
    );
    let _resp = db
        .query(&update_query)
        .bind(("table", "movie"))
        .bind(("reviewIds", reviews))
        .await
        .unwrap();
    //return all reviews for the movie
    let query = format!(
        "SELECT * FROM type::table($table) WHERE imdbId = '{}';",
        review.imdbId
    );
    let mut response: surrealdb::Response =
        db.query(&query).bind(("table", "movie")).await.unwrap();
    let movie_object: Option<Movie> = response.take(0).unwrap();
    let review_ids = movie_object.clone().unwrap().reviewIds;
    Ok(warp::reply::json(&review_ids))
}

pub fn post_reply() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    //handle post_review_to_db at /api/v1/reviews
    warp::path!("api" / "v1" / "reviews")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(post_review_to_db)
}
