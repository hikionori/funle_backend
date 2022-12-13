#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::options::UpdateModifications;
use rocket::{http::ext::IntoCollection, State};

use crate::models::response;
use crate::repository::user_repo::UserRepo;
use crate::{models::tests_model::TestModelWithActions, utils::errors::TestsError};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection,
};

use rand::{self, seq::SliceRandom};

pub struct TestsRepo {
    pub collection: Collection<TestModelWithActions>,
}

type Test = TestModelWithActions;

// TODO: add implementation of methods for TestsRepo
// TODO: add tests for methods of TestsRepo
impl TestsRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let client = Client::with_uri_str(&env::var("MONGO_URL").unwrap())
            .await
            .unwrap();
        let db = client.database("tests");
        let collection = db.collection("tests_with_actions");
        Self { collection }
    }

    pub async fn create_test(&self, test: Test) -> Result<InsertOneResult, TestsError> {
        let result = self.collection.insert_one(test, None).await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(TestsError::CreateTest),
        }
    }

    pub async fn get_test_by_id(&self, id: &str) -> Result<Option<Test>, TestsError> {
        let oid = ObjectId::parse_str(id).unwrap();
        let test = self.collection.find_one(doc! {"_id": oid}, None).await;
        match test {
            Ok(ok) => Ok(ok),
            Err(_) => Err(TestsError::GetTest),
        }
    }

    pub async fn get_test_by_ex(&self, ex: String) -> Result<Option<Test>, TestsError> {
        let result = self
            .collection
            .find_one(
                doc! {
                    "example": ex
                },
                None,
            )
            .await;
        match result {
            Ok(ok) => Ok(ok),
            Err(_) => Err(TestsError::GetTest),
        }
    }

    pub async fn delete_test(&self, id: &str) -> Result<(), TestsError> {
        let oid = ObjectId::parse_str(id).unwrap();
        let result = self.collection.find_one_and_delete(doc!{"_id": oid}, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TestsError::DeleteTest)
        }
    }

    pub async fn update_test(&self, id: &str, new_test: Test) -> Result<Option<Test>, TestsError> {
        let oid = ObjectId::parse_str(id).unwrap();
        let new_test = UpdateModifications::Document(
            doc! {"$set": mongodb::bson::to_document(&new_test).unwrap()}
        );
        let result = self.collection.find_one_and_update(doc!{"_id": oid}, new_test, None).await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(TestsError::UpdateTest)
        }
    }

}

#[cfg(test)]
mod tests_with_actions_tests {
    use super::*;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use tokio::*;

    async fn gen_test() -> Test {
        Test {
            id: None,
            example: "2 + 2 * 2".to_string(),
            actions: vec!["2 * 2".to_string(), "2 + 4".to_owned()],
            answer: "6".to_owned(),
            level: 1,
        }
    }

    async fn gen_n_test(n: i32, level: i32) -> Vec<Test> {
        let mut tests = Vec::new();
        for _ in 0..n {
            let mut test = gen_test().await;
            test.level = level;
            tests.push(test)
        }
        tests
    }

    async fn get_test_id(ex: &String) -> ObjectId {
        let client = setup(false).await;
        let oid = client
            .collection
            .find_one(
                doc! {
                    "example": ex.to_owned()
                },
                None,
            )
            .await
            .unwrap()
            .unwrap();
        oid.id.unwrap()
    }

    async fn setup(clean: bool) -> TestsRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let client = TestsRepo::init().await;
        if clean {
            client.collection.drop(None).await.unwrap();
        }
        client
    }

    #[tokio::test]
    async fn create_test() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_test_by_ex() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        match result {
            Ok(_) => {
                let test = client.get_test_by_ex("2 + 2 * 2".to_string()).await;
                match test {
                    Ok(test) => {
                        let test = test.unwrap();
                        assert!(test.actions.len() == 2);
                    }
                    Err(_) => {
                        panic!("Test dont get");
                    }
                }
            }
            Err(_) => {
                panic!("Test dont created");
            }
        }
    }

    #[tokio::test]
    async fn get_test_by_id() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        match result {
            Ok(_) => {
                let test_oid = get_test_id(&"2 + 2 * 2".to_string()).await;
                let test = client.get_test_by_id(test_oid.to_string().as_str()).await;
                match test {
                    Ok(test) => {
                        let test = test.unwrap();
                        assert!(test.actions.len() == 2);
                    }
                    Err(_) => {
                        panic!("Test dont get");
                    }
                }
            }
            Err(_) => {
                panic!("Test dont created");
            }
        }
    }

    #[tokio::test]
    async fn delete_test() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        match result {
            Ok(_) => {
                let test_id = get_test_id(&"2 + 2 * 2".to_string()).await;
                client.delete_test(test_id.clone().to_string().as_str()).await.unwrap();
                let test_after_delete = client.get_test_by_id(test_id.clone().to_string().as_str()).await.unwrap();
                assert!(test_after_delete.is_none());
            },
            Err(_) => {
                panic!("Test dont created")
            }
        }
    }

    #[tokio::test]
    async fn update_test() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        match result {
            Ok(_) => {
                let test_id = get_test_id(&"2 + 2 * 2".to_string()).await;
                let test = client.get_test_by_id(test_id.clone().to_string().as_str()).await.unwrap().unwrap();
                let new_test = Test {
                    id: Some(test.id.unwrap()),
                    example: "2 + 2 / 2".to_string(),
                    actions: vec!["2 / 2".to_string(), "2 + 1".to_string()],
                    answer: "3".to_string(),
                    level: 1
                };
                let result = client.update_test(test_id.to_string().as_str(), new_test).await;
                match result {
                    Ok(_) => {
                        let res = client.get_test_by_id(test_id.clone().to_string().as_str()).await.unwrap().unwrap();
                        assert_eq!(res.id, test.id);
                        assert_eq!(res.example, "2 + 2 / 2".to_string());
                    },
                    Err(_) => panic!("Test dont updated")
                }
            },
            Err(_) => {
                panic!("Test dont created")
            }
        }
    }
}
