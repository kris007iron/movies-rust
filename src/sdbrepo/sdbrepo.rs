use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct SDBrepo {
    pub db: Surreal<Client>,
}
impl SDBrepo {
    pub async fn new() -> surrealdb::Result<SDBrepo> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;
        Ok(SDBrepo { db })
    }
}
