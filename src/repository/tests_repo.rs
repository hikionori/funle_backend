#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::{http::ext::IntoCollection, State};

use crate::{models::tests_model::TestModel, utils::errors::TestsError};
use crate::repository::user_repo::UserRepo;
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
        let mongo_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
        // let mongo_url = "mongodb://root:root@localhost:27017/";
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

    pub async fn get_test_by_name(&self, name: String) -> Result<Option<Test>, Error> {
        let test = self
            .collection
            .find_one(doc! {"name": name}, None)
            .await
            .unwrap();
        Ok(test)
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

    pub async fn get_all_tests(&self) -> Result<Vec<Test>, Error> {
        let tests: Vec<Test> = self
            .collection
            .find(None, None)
            .await
            .ok()
            .expect("Error getting tasks")
            .deserialize_current()
            .into_iter()
            .collect();
        Ok(tests)
    }

    // ! FIXME: Переписать этот метод
    // ! Скорее всего нужно будет переписывать эти функции так как нужно это делать с учетом того что пользователь уже сделал
    pub async fn get_random_tests(
        &self,
        n: i32,
        level: i32,
        user_id: String,
        user_db: &State<UserRepo>,
    ) -> Result<Vec<Test>, TestsError> {
        let mut tests: Vec<Test> = self
            .collection
            .find(doc! {"level": level}, None)
            .await
            .unwrap()
            .deserialize_current()
            .into_iter()
            .collect();

        // remove from tests all tests that user already done
        let user = user_db.get_user_by_id(&user_id).await.unwrap();
        match user {
            Some(user) => {
                for test_id in user.progress.tests.into_iter() {
                    tests.retain(|t| t.id.unwrap() != self.string_to_id(test_id.clone()));
                }
                // shuffle tests
                let mut rng = rand::thread_rng();
                tests.shuffle(&mut rng);
                // return n tests
                tests.truncate(n as usize);
                Ok(tests)
            }
            None => {
                Err(TestsError::WeAreCanNotGetTests)
            }
        }

    }

    fn string_to_id(&self, id: String) -> mongodb::bson::oid::ObjectId {
        mongodb::bson::oid::ObjectId::parse_str(id.as_str()).unwrap()
    }
}
