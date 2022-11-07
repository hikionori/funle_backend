#[warn(unused_imports)]
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::{User, UserRole};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection
};
use bcrypt;

// TODO: Переписать репозиторий пользователя с учётом новой модели

pub struct UserRepo {
    pub collection: Collection<User>,
}

impl UserRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        // let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let mongo_url = "mongodb://root:root@localhost:27017/"; //? test database TODO: change to env variable
        let client = Client::with_uri_str(mongo_url).await.unwrap();
        let db = client.database("mathdb");

        let collection: Collection<User> = db.collection("users");
        UserRepo { collection }
    }

    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        let new_user = User {
            id: None,
            username: user.username,
            email: user.email,
            hashed_password: Self::hash_password(&self, user.hashed_password),
            role: UserRole::User,
        };
        let user = self
            .collection
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error inserting user");
        Ok(user)
    }

    pub async fn get_user_by_name(&self, name: String) -> Result<Option<User>, Error> {
        let user = self
            .collection
            .find_one(doc! {"name": name}, None)
            .await
            .ok()
            .expect("Error finding user");
        Ok(user)
    }

    pub async fn delete_user_by_id(&self, id: String) -> Result<Option<User>, Error> {
        let user = self
            .collection
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user)
    }

    pub async fn put_user_by_id(&self, id: String, user: User) -> Result<Option<User>, Error> {
        let user = self
            .collection
            .find_one_and_replace(doc! {"_id": id}, user, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(user)
    }

    fn hash_password(&self, password: String) -> String {
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        hashed_password
    }
}