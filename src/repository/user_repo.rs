#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::{UserModel, UserRole, UserProgress};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection
};
use sha256::digest;

use tokio;

pub struct UserRepo {
    pub collection: Collection<UserModel>,
}

impl UserRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(mongo_url).await.unwrap();
        let db = client.database("mathdb");

        let collection: Collection<UserModel> = db.collection("users");
        UserRepo { collection }
    }

    // * User methods

    pub async fn create_user(&self, user: UserModel) -> Result<InsertOneResult, Error> {
        let new_user = UserModel {
            id: None,
            username: user.username,
            email: user.email,
            hashed_password: Self::hash_password(self, user.hashed_password),
            role: UserRole::User,
            progress: user.progress,
        };

        if (self.get_user_by_email(&new_user.email).await.unwrap().is_some()) {
            return Err(<Error as serde::de::Error>::custom("User with this email already exists"));
        }

        let user = self
            .collection
            .insert_one(new_user, None)
            .await
            .expect("Error inserting user");
        Ok(user)
    }

    pub async fn get_user_by_name(&self, name: &String) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one(doc! {"name": name}, None)
            .await
            .expect("Error finding user");
        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &String) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one(doc! {"_id": id}, None)
            .await
            .expect("Error finding user");
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one(doc! {"email": email}, None)
            .await
            .expect("Error finding user");
        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserModel>, Error> {
        let users = self
            .collection
            .find(None, None)
            .await
            .unwrap()
            .deserialize_current()
            .into_iter()
            .collect();
        Ok(users)
    } 

    pub async fn delete_user_by_id(&self, id: &String) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .expect("Error deleting user");
        Ok(user)
    }

    pub async fn put_user_by_id(&self, id: &String, user: UserModel) -> Result<Option<UserModel>, Error> {
        let user = self
            .collection
            .find_one_and_replace(doc! {"_id": id}, user, None)
            .await
            .expect("Error updating user");
        Ok(user)
    }

    // * Progress methods
    
    pub async fn add_cource_to_user(&self, user_id: String, cource_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.courses.push(cource_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn add_test_to_user(&self, user_id: String, test_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.tests.push(test_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn add_info_to_user(&self, user_id: String, info_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.infos.push(info_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn remove_cource_from_user(&self, user_id: String, cource_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.courses.retain(|x| x != &cource_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn remove_test_from_user(&self, user_id: String, test_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.tests.retain(|x| x != &test_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn remove_info_from_user(&self, user_id: String, info_id: String) {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress.infos.retain(|x| x != &info_id);
        self.put_user_by_id(&user_id, user).await.unwrap();
    }

    pub async fn update_all_progress(&self, user_id: String, progress: UserProgress) -> Result<Option<UserModel>, Error> {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress = progress;
        Ok(self.put_user_by_id(&user_id, user).await.unwrap())
    }

    // * Password methods

    pub fn hash_password(&self, password: String) -> String {
        digest(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use std::env;

    async fn setup(clean_db: bool) -> UserRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let client = UserRepo::init().await;
        if clean_db {
            client.collection.drop(None).await.unwrap();
        }
        client
    }
    
    #[tokio::test]
    async fn test_create_user() {
        let client = setup(true).await;
        let user = UserModel {
            id: None,
            username: "test".to_string(),
            email: "test".to_string(),
            hashed_password: client.hash_password("test".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![]
            }
        };
        let result = client.create_user(user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        todo!()
    }
}