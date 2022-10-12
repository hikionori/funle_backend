#[warn(unused_imports)]
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::User;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

pub struct UserRepo {
    pub collection: Collection<User>,
}

impl UserRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        // let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let mongo_url = "mongodb://root:root@localhost:27017/"; // test database TODO: change to env variable
        let client = Client::with_uri_str(mongo_url).await.unwrap();
        let db = client.database("mathdb");

        let collection: Collection<User> = db.collection("users");
        UserRepo { collection }
    }

    pub async fn create(&self, user: User) -> Result<InsertOneResult, Error> {
        let new_user = User {
            id: None,
            name: user.name,
            email: user.email,
            password: user.password,
        };
        let user = self.collection.insert_one(new_user, None).await.ok().expect("Error inserting user");
        Ok(user)
    }
}
