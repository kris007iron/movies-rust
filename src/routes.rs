pub mod movie_routes;
pub mod review_routes;

pub use movie_routes::*;
pub use review_routes::*;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_movie().or(get_movies()).or(post_reply())
}
