#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::user_model::{UserModel, UserProgress, UserRole};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
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

        if (self
            .get_user_by_email(&new_user.email)
            .await
            .unwrap()
            .is_some())
        {
            return Err(<Error as serde::de::Error>::custom(
                "User with this email already exists",
            ));
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
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let user = self
            .collection
            .find_one(doc! {"_id": oid}, None)
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
        let cursor = self.collection.find(None, None).await.unwrap();

        let users = cursor.try_collect().await.unwrap();

        Ok(users)
    }

    pub async fn delete_user_by_id(&self, id: &String) -> Option<UserModel> {
        // let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let email = self.get_user_by_id(id).await.unwrap().unwrap().email;
        self.collection
            .delete_many(doc! {"email": email}, None)
            .await
            .unwrap();
        None
    }

    pub async fn put_user_by_id(
        &self,
        id: &String,
        user: UserModel,
    ) -> Result<Option<UserModel>, Error> {
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let user = self
            .collection
            .find_one_and_replace(doc! {"_id": oid}, user, None)
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

    pub async fn update_all_progress(
        &self,
        user_id: String,
        progress: UserProgress,
    ) -> Result<Option<UserModel>, Error> {
        let mut user = self.get_user_by_id(&user_id).await.unwrap().unwrap();
        user.progress = progress;
        Ok(self.put_user_by_id(&user_id, user).await.unwrap())
    }

    // * Password methods

    pub fn hash_password(&self, password: String) -> String {
        digest(password)
    }

    pub fn hash_password_pub(password: String) -> String {
        digest(password)
    }
}

#[cfg(test)]
mod user_repo_tests {
    use super::*;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use std::{collections::HashMap, env};

    use crate::{
        api::user_api::get_user,
        models::{
            cource_model::{CourseModel, Level},
            info_model::{ContentLevel, InfoModel},
            tests_model::TestModel,
        },
    };

    async fn drop_collection() {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let client = UserRepo::init().await;
        client.collection.drop(None).await.unwrap();
    }

    async fn setup(clean_db: bool, gen_user: Option<i32>) -> UserRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let client = UserRepo::init().await;
        if clean_db {
            drop_collection().await;
        }
        if let Some(n) = gen_user {
            let users = gen_n_users(n).await;
            for user in users {
                client.create_user(user).await.unwrap();
            }
        }
        client
    }

    async fn gen_n_users(n: i32) -> Vec<UserModel> {
        let mut users: Vec<UserModel> = Vec::new();
        for i in 0..n {
            users.push(UserModel {
                id: None,
                username: format!("test{}", i),
                email: format!("test{}", i),
                hashed_password: UserRepo::hash_password_pub(format!("test{}", i)),
                role: UserRole::User,
                progress: UserProgress {
                    courses: vec![],
                    tests: vec![],
                    infos: vec![],
                },
            })
        }
        users
    }

    async fn _get_user_by_email(email: &String) -> UserModel {
        let client = setup(false, None).await;
        client.get_user_by_email(email).await.unwrap().unwrap()
    }

    // *UserRepo CRUD tests
    #[tokio::test]
    async fn create_user() {
        let client = setup(true, None).await;
        let user = UserModel {
            id: None,
            username: "test".to_string(),
            email: "test".to_string(),
            hashed_password: client.hash_password("test".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        let result = client.create_user(user).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_user_by_email() {
        let client = setup(true, None).await;

        let user = UserModel {
            id: None,
            username: "test6".to_string(),
            email: "test6".to_string(),
            hashed_password: client.hash_password("test6".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };

        client.create_user(user).await.unwrap();

        let result = client
            .get_user_by_email(&"test6".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.username, "test6".to_string());
    }

    #[tokio::test]
    async fn get_user_by_id() {
        let client = setup(true, None).await;

        let user = UserModel {
            id: None,
            username: "test9".to_string(),
            email: "test9".to_string(),
            hashed_password: client.hash_password("test9".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        client.create_user(user).await.unwrap();

        let user_for_search = _get_user_by_email(&"test9".to_string()).await;
        let result = client
            .get_user_by_id(&user_for_search.id.unwrap().to_string())
            .await
            .unwrap();
        assert_eq!(result.unwrap().username, user_for_search.username);
    }

    #[tokio::test]
    async fn get_user_by_name() {
        let client = setup(true, None).await;

        let user = UserModel {
            id: None,
            username: "test4".to_string(),
            email: "test4".to_string(),
            hashed_password: client.hash_password("test4".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        client.create_user(user).await.unwrap();

        let user_for_search = _get_user_by_email(&"test4".to_string()).await;
        let result = client.get_user_by_name(&user_for_search.username).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_all_users() {
        // drop_collection().await;
        let client = setup(true, Some(3)).await;
        let result = client.get_all_users().await.unwrap();
        // println!("{:#?}", result);
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn delete_user_by_id() {
        let client = setup(true, None).await;

        let user = UserModel {
            id: None,
            username: "test5".to_string(),
            email: "test5".to_string(),
            hashed_password: client.hash_password("test5".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        client.create_user(user).await.unwrap();

        let user_for_search = _get_user_by_email(&"test5".to_string()).await;
        let result = client
            .delete_user_by_id(&user_for_search.id.unwrap().to_string())
            .await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn put_user_by_id() {
        let client = setup(true, None).await;

        let user = UserModel {
            id: None,
            username: "test7".to_string(),
            email: "test7".to_string(),
            hashed_password: client.hash_password("test7".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        client.create_user(user).await.unwrap();

        let user_for_search = _get_user_by_email(&"test7".to_string()).await;
        let user_for_update = UserModel {
            id: user_for_search.id,
            username: "test78".to_string(),
            email: "test7".to_string(),
            hashed_password: client.hash_password("test7".to_string()),
            role: UserRole::User,
            progress: UserProgress {
                courses: vec![],
                tests: vec![],
                infos: vec![],
            },
        };
        client
            .put_user_by_id(&user_for_search.id.unwrap().to_string(), user_for_update)
            .await
            .unwrap()
            .unwrap();
        let result = _get_user_by_email(&"test7".to_string()).await;
        assert_eq!(result.username, "test78".to_string());
    }

    // *UserRepo Progress methods tests
    // todo: after finished CourceRepo, TestsRepo, InfoRepo write tests for progress methods in UserRepo
}
