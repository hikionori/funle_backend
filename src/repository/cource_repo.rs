#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]


use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::cource_model::*;
use mongodb :: {
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
};
use tokio;

pub struct CourceRepo {
    collection: Collection<CourseModel>,
}

impl CourceRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongodb_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
        let client = Client::with_uri_str(&mongodb_url).await.expect("Failed to initialize client.");
        let db = client.database("mathdb");
        let collection = db.collection("cources");
        Self { collection }
    }

    
    pub async fn create(&self, cource: CourseModel) -> InsertOneResult {
        self.collection.insert_one(cource, None).await.unwrap()
    }

    pub async fn get(&self, id: &str) -> Option<CourseModel> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        
        self.collection.find_one(filter, None).await.unwrap()
    }

    pub async fn update(&self, id: &str, cource: CourseModel) -> Option<CourseModel> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        
        self
            .collection
            .find_one_and_replace(filter, cource, None)
            .await
            .unwrap()
    }

    pub async fn delete(&self, id: &str) -> DeleteResult {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        
        self.collection.delete_one(filter, None).await.unwrap()
    }

    pub async fn get_all_ids(&self) -> Option<Vec<String>> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut ids = Vec::new();
        while let Some(result) = cursor.try_next().await.unwrap() {
            let id = result.id.unwrap().to_hex();
            ids.push(id);
        }
        Some(ids)
    }

    pub async fn get_all(&self) -> Option<Vec<CourseModel>> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut courses = Vec::new();
        while let Some(result) = cursor.try_next().await.unwrap() {
            courses.push(result);
        }
        Some(courses)
    }

    pub async fn drop_collection(&self) {
        self.collection.drop(None).await.unwrap();
    }

    // CRUD for levels in cource
    pub async fn add_level(&self, cource_id: &str, level: Level, level_number: i32) {
        let cource_id = ObjectId::parse_str(cource_id).unwrap();
        let cource = self.collection.find_one(doc!{"_id": cource_id}, None).await.unwrap();
        match cource {
            Some(cource) => {
                let mut levels = cource.levels;
                // insert level in levels with level number as key and level as value
                // if level number already exists, then push level to vector
                match levels.get_mut(&level_number) {
                    Some(vec) => vec.push(level),
                    None => {
                        levels.insert(level_number, vec![level]);
                    }
                }
            },
            None => {
                println!("Cource with id {} not found", cource_id);
            }
        }
    }
}

#[cfg(test)]
mod cource_repo_tests {
    use super::*;
    use crate::models::cource_model::*;
    use crate::models::info_model::*;
    use crate::models::tests_model::*;

    use crate::repository::infos_repo::*;
    use crate::repository::tests_repo::*;

    async fn setup_cource_db(clean: bool) -> CourceRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let cource_repo = CourceRepo::init().await;
        if clean {
            cource_repo.drop_collection().await;
        }
        cource_repo
    }

    async fn setup_tests_db(clean: bool) -> TestsRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let tests_repo = TestsRepo::init().await;
        if clean {
            tests_repo.drop_collection().await;
        }
        tests_repo
    }

    async fn setup_infos_db(clean: bool) -> InfosRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let infos_repo = InfosRepo::init().await;
        if clean {
            infos_repo.drop_collection(true).await;
        }
        infos_repo
    }

    async fn create_info_in_db(info: InfoModel) -> String {
        let infos_repo = setup_infos_db(true).await;
        let info = infos_repo.create_info(info).await.unwrap();
        info.inserted_id.to_string()
    }

    async fn create_test_in_db(test: TestModel) -> String {
        let tests_repo = TestsRepo::init().await;
        let test = tests_repo.create_test(test).await.unwrap();
        test.inserted_id.to_string()
    }

    #[tokio::test]
    async fn create_test() {
        unimplemented!()
    }
}
