use warp::Filter;

mod models;
mod routes;
mod sdbrepo;

use routes::routes;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST", "UPDATE", "DELETE", "SET", "PUT"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_credentials(true)
        .allow_any_origin(); // Enable credentials
    let routes = routes().with(cors);
    //let reviews: Vec<Review> = db.delete("review").await?;
    //get and print all reviews from db
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{movie::Movie, review::ReviewQuery},
        routes::{get_movie_from_db, get_movies_from_db, post_review_to_db},
        sdbrepo::sdbrepo::SDBrepo,
    };
    use warp::test::request;

    #[tokio::test]
    async fn test_get_movie_from_db() {
        // You need to set up a test database or mock it for this test
        let _db = SDBrepo::new().await.unwrap().db;

        // Create a warp filter for the get_movie_from_db endpoint
        let filter = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("movies"))
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::get())
            .and_then(get_movie_from_db);

        // Perform a test request to the filter
        let response = request()
            .method("GET")
            .path("/api/v1/movies/tt3915174") // Provide a valid movie ID here
            .reply(&filter)
            .await;

        // Assert the response status code and content
        assert_eq!(response.status(), warp::http::StatusCode::OK);
        let _movie: Movie = serde_json::from_slice(response.body()).unwrap();
        // Add more assertions based on your test data
    }

    #[tokio::test]
    async fn test_get_movies_from_db() {
        // You need to set up a test database or mock it for this test
        let _db = SDBrepo::new().await.unwrap().db;

        // Create a warp filter for the get_movies_from_db endpoint
        let filter = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("movies"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(get_movies_from_db);

        // Perform a test request to the filter
        let response = request()
            .method("GET")
            .path("/api/v1/movies")
            .reply(&filter)
            .await;

        // Assert the response status code and content
        assert_eq!(response.status(), warp::http::StatusCode::OK);
        let _movies: Vec<Movie> = serde_json::from_slice(response.body()).unwrap();
        // Add more assertions based on your test data
    }

    #[tokio::test]
    async fn test_post_review_to_db() {
        // You need to set up a test database or mock it for this test
        let _db = SDBrepo::new().await.unwrap().db;

        // Create a warp filter for the post_review_to_db endpoint
        let filter = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("reviews"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(post_review_to_db);

        // Create a sample ReviewQuery object to use in the test
        let review_query = ReviewQuery {
            imdbId: "tt3915174".to_string(), // Provide a valid movie ID here
            reviewBody: "Some review text".to_string(), // Provide valid review data here
        };

        // Perform a test request to the filter
        let response = request()
            .method("POST")
            .path("/api/v1/reviews")
            .json(&review_query)
            .reply(&filter)
            .await;

        // Assert the response status code and content
        assert_eq!(response.status(), warp::http::StatusCode::OK);
        // Add more assertions based on your test data
    }
}
