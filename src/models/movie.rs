use serde::{Deserialize, Serialize};

use super::review::Review;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Movie {
    pub imdbId: String,
    pub title: String,
    pub releaseDate: String,
    pub trailerLink: String,
    pub genres: Vec<String>,
    pub poster: String,
    pub backdrops: Vec<String>,
    pub reviewIds: Vec<Review>,
}
