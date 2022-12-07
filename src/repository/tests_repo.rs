#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateModifications;
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
            collection,
        }
    }

    
    pub async fn create_test(&self, test: Test) -> Result<InsertOneResult, Error> {
        let result = self
        .collection
        .insert_one(test, None)
        .await
        .expect("Error creating task");
        Ok(result)
    }
    
    pub async fn get_test_by_id(&self, id: &String) -> Result<Option<Test>, Error> {
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
        Ok(task)
    }

    pub async fn delete_test(&self, id: &String) -> Result<(), Error> {
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        self.collection
            .delete_one(doc! {"_id": oid}, None)
            .await
            .expect("Error deleting task");
        Ok(())
    }

    pub async fn update_test_by_id(&self, id: &String, new_test: Test) -> Result<Option<TestModel>, Error> {
        let oid = ObjectId::parse_str(id.as_str()).expect("Error parsing id");
        let update = UpdateModifications::Document(doc! {"$set": mongodb::bson::to_document(&new_test).unwrap()});
        let updated = self.collection.find_one_and_update(doc!{"_id": oid}, update, None).await;
        Ok(updated.unwrap())
    }

    pub async fn get_all_tests(&self) -> Result<Vec<Test>, Error> {
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
            None => {
                Err(TestsError::WeAreCanNotGetTests)
            }
        }

    }

    fn string_to_id(&self, id: String) -> mongodb::bson::oid::ObjectId {
        mongodb::bson::oid::ObjectId::parse_str(id.as_str()).unwrap()
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
            answers: vec!["2", "3", "4"].into_iter().map(|x| x.to_string()).collect(),
            correct_answer: "2".to_string(),
            level: 1
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
        let id = client.collection.find_one(doc! {"text_of_question": question_text.to_owned()}, None).await.unwrap().unwrap();
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
        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn update_test() {
        let client = setup(true).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"].into_iter().map(|x| x.to_string()).collect(),
            correct_answer: "2".to_string(),
            level: 1
        };
        client.create_test(test).await.unwrap();
        let test_id = get_test_id(&"1 + 1".to_string()).await;
        println!("{:?}", test_id.to_string());
        let new_test = TestModel {
            id: Some(test_id),
            text_of_question: "2 + 2".to_string(),
            answers: vec!["2", "3", "4"].into_iter().map(|x| x.to_string()).collect(),
            correct_answer: "4".to_string(),
            level: 1
        };
        let updated_id = client.update_test_by_id(&test_id.to_string(), new_test).await.unwrap().unwrap().id.unwrap();
        let result = client.get_test_by_id(&test_id.to_string()).await.unwrap().unwrap();
        assert!(result.id.unwrap() == updated_id)
    }

    #[tokio::test]
    async fn delete_test() {
        let client = setup(true).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"].into_iter().map(|x| x.to_string()).collect(),
            correct_answer: "2".to_string(),
            level: 1
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