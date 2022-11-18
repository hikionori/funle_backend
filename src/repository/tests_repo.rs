#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::tests_model::{TestModel};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection
};


pub struct TestsRepo {
    pub collection: Collection<TestModel>
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

    pub async fn get_task_by_id(&self, id: String) -> Result<Option<Test>, Error> {
        let task = self.collection.find_one(doc!{"_id": id}, None).await.ok().expect("Error getting task");
        Ok(task)
    }
}