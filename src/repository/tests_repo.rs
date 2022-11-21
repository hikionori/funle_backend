#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::http::ext::IntoCollection;

use crate::models::tests_model::TestModel;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection,
};

use rand::{self, seq::SliceRandom};

pub struct TestsRepo {
    pub collection: Collection<TestModel>,
}

type Test = TestModel;

impl TestsRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        // let mongo_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
        let mongo_url = "mongodb://root:root@localhost:27017/";
        let client = Client::with_uri_str(mongo_url).await.unwrap();

        let collection: Collection<TestModel> = client.database("mathdb").collection("tests");

        TestsRepo {
            collection: collection,
        }
    }

    pub async fn get_test_by_id(&self, id: String) -> Result<Option<Test>, Error> {
        let task = self
            .collection
            .find_one(doc! {"_id": id}, None)
            .await
            .ok()
            .expect("Error getting task");
        Ok(task)
    }

    pub async fn create_test(&self, test: Test) -> Result<InsertOneResult, Error> {
        let result = self
            .collection
            .insert_one(test, None)
            .await
            .ok()
            .expect("Error creating task");
        Ok(result)
    }

    pub async fn delete_test(&self, id: String) -> Result<(), Error> {
        self.collection
            .delete_one(doc! {"_id": id}, None)
            .await
            .ok()
            .expect("Error deleting task");
        Ok(())
    }

    pub async fn update_test_by_id(&self, id: String, new_test: Test) -> Result<(), Error> {
        self.collection
            .find_one_and_replace(doc! {"_id": id}, new_test, None)
            .await
            .ok()
            .expect("Error updating task");
        Ok(())
    }

    pub async fn get_random_tests(&self, n: i32, level: i32) -> Result<Vec<Test>, Error> {
        let mut tests: Vec<Test> = self
            .collection
            .find(doc! {"level": level}, None)
            .await
            .unwrap()
            .deserialize_current()
            .into_iter()
            .collect();

        let mut rng = rand::thread_rng();
        tests.shuffle(&mut rng);
        tests.truncate(n as usize);
        Ok(tests)
    }
}
