use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    pub id: String,
    pub body: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReviewQuery {
    pub reviewBody: String,
    pub imdbId: String,
}
