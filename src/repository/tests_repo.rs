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

use crate::repository::user_repo::UserRepo;
use crate::{models::tests_model::TestModel, utils::errors::TestsError};
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

        TestsRepo { collection }
    }

    pub async fn create_test(&self, test: Test) -> Result<InsertOneResult, TestsError> {
        let result = self.collection.insert_one(test, None).await;
        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(TestsError::CreateTest),
        }
    }

    pub async fn get_test_by_id(&self, id: &String) -> Result<Option<Test>, TestsError> {
        let oid = ObjectId::parse_str(id.as_str());
        let oid = match oid {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        let task = self
            .collection
            .find_one(doc! {"_id": oid}, None)
            .await
            .expect("Error getting task");
        match task {
            Some(task) => Ok(Some(task)),
            None => Err(TestsError::GetTest),
        }
    }

    pub async fn delete_test(&self, id: &String) -> Result<(), TestsError> {
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let result = self.collection.delete_one(doc! {"_id": oid}, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TestsError::DeleteTest),
        }
    }

    pub async fn update_test_by_id(
        &self,
        id: &String,
        new_test: Test,
    ) -> Result<Option<TestModel>, TestsError> {
        let oid = ObjectId::parse_str(id.as_str()).expect("Error parsing id");
        let update = UpdateModifications::Document(
            doc! {"$set": mongodb::bson::to_document(&new_test).unwrap()},
        );
        let updated = self
            .collection
            .find_one_and_update(doc! {"_id": oid}, update, None)
            .await;
        match updated {
            Ok(test) => Ok(test),
            Err(_) => Err(TestsError::UpdateTest)
        }
    }

    pub async fn get_all_tests(&self) -> Result<Vec<Test>, TestsError> {
        let tests: Vec<Test> = self
            .collection
            .find(None, None)
            .await
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
            None => Err(TestsError::GetTests),
        }
    }

    fn string_to_id(&self, id: String) -> mongodb::bson::oid::ObjectId {
        mongodb::bson::oid::ObjectId::parse_str(id.as_str()).unwrap()
    }

    pub async fn drop_collection(&self) {
        self.collection.drop(None).await.unwrap();
    }
}

#[cfg(test)]
mod test_repo_tests {
    use super::*;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use tokio::*;

    async fn gen_test() -> TestModel {
        TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            correct_answer: "2".to_string(),
            level: 1,
        }
    }

    async fn gen_n_test(n: i32, level: i32) -> Vec<TestModel> {
        let mut tests = Vec::new();
        for _ in 0..n {
            let mut test = gen_test().await;
            test.level = level;
            tests.push(test);
        }
        tests
    }

    async fn setup(clean_db: bool) -> TestsRepo {
        env::set_var("MONGODB_URL", "mongodb://root:root@localhost:27017/");
        let client = TestsRepo::init().await;
        if clean_db {
            client.collection.drop(None).await.unwrap();
        }
        client
    }

    async fn get_test_id(question_text: &String) -> ObjectId {
        let client = setup(false).await;
        let id = client
            .collection
            .find_one(doc! {"text_of_question": question_text.to_owned()}, None)
            .await
            .unwrap()
            .unwrap();
        id.id.unwrap()
    }

    #[tokio::test]
    async fn create_test() {
        let client = setup(true).await;
        let test = gen_test().await;
        let result = client.create_test(test).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn get_test_by_id() {
        let client = setup(true).await;
        let test = gen_test().await;
        client.create_test(test).await.unwrap();
        let test_id = get_test_id(&"1 + 1".to_string()).await;
        let result = client.get_test_by_id(&test_id.to_string()).await.unwrap();
        assert_eq!(result.unwrap().text_of_question, "1 + 1".to_string());
    }

    #[tokio::test]
    async fn get_all_tests() {
        let client = setup(true).await;
        let tests = gen_n_test(10, 1).await;
        for test in tests {
            client.create_test(test).await.unwrap();
        }
        let result = client.get_all_tests().await.unwrap();
        assert!(!result.is_empty());
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn update_test() {
        let client = setup(true).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            correct_answer: "2".to_string(),
            level: 1,
        };
        client.create_test(test).await.unwrap();
        let test_id = get_test_id(&"1 + 1".to_string()).await;
        println!("{:?}", test_id.to_string());
        let new_test = TestModel {
            id: Some(test_id),
            text_of_question: "2 + 2".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            correct_answer: "4".to_string(),
            level: 1,
        };
        let updated_id = client
            .update_test_by_id(&test_id.to_string(), new_test)
            .await
            .unwrap()
            .unwrap()
            .id
            .unwrap();
        let result = client
            .get_test_by_id(&test_id.to_string())
            .await
            .unwrap()
            .unwrap();
        assert!(result.id.unwrap() == updated_id)
    }

    #[tokio::test]
    async fn delete_test() {
        let client = setup(true).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            correct_answer: "2".to_string(),
            level: 1,
        };
        client.create_test(test).await.unwrap();

        let test_id = get_test_id(&"1 + 1".to_string()).await;
        client.delete_test(&test_id.to_string()).await.unwrap();

        let result = client.get_test_by_id(&test_id.to_string()).await.unwrap();
        assert!(result.is_none())
    }

    #[tokio::test]
    async fn get_random_tests() {
        unimplemented!()
    }
}
