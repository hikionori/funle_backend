#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::options::UpdateModifications;
use rocket::futures::TryStreamExt;
use rocket::{http::ext::IntoCollection, State};

use crate::repository::user_repo::UserRepo;
use crate::{models::tests_model::TestModel};
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
    /// It creates a new instance of the TestsRepo struct.
    /// 
    /// Returns:
    /// 
    /// A TestsRepo struct
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        // let mongo_url = "mongodb://root:root@localhost:27017/";
        let client = Client::with_uri_str(mongo_url).await.unwrap();

        let collection: Collection<TestModel> = client.database("mathdb").collection("tests");

        TestsRepo { collection }
    }

    /// > This function creates a new test in the database
    /// 
    /// Arguments:
    /// 
    /// * `test`: Test - This is the test object that we want to insert into the database.
    /// 
    /// Returns:
    /// 
    /// The result of the insert_one function.
    pub async fn create_test(&self, test: Test) -> Result<InsertOneResult, Error> {
        let result = self.collection.insert_one(test, None).await.unwrap();
        Ok(result)
    }

    /// > This function takes a string id, parses it into an ObjectId, and then uses that ObjectId to
    /// find a test in the database
    /// 
    /// Arguments:
    /// 
    /// * `id`: &String - The id of the test we want to get
    /// 
    /// Returns:
    /// 
    /// A Result<Option<Test>, TestsError>
    pub async fn get_test_by_id(&self, id: &String) -> Result<Option<Test>, Error> {
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let test = self.collection.find_one(doc! {"_id": oid}, None).await;
        Ok(test.unwrap())
    }

    /// It deletes a test from the database.
    /// 
    /// Arguments:
    /// 
    /// * `id`: &String - The id of the test to delete
    /// 
    /// Returns:
    /// 
    /// A Result<(), TestsError>
    pub async fn delete_test(&self, id: &String) -> Result<(), Error> {
        let oid = ObjectId::parse_str(id.as_str()).unwrap();
        let result = self.collection.delete_one(doc! {"_id": oid}, None).await;
        Ok(())
    }

    /// It updates a test by id.
    /// 
    /// Arguments:
    /// 
    /// * `id`: &String - The id of the test to update
    /// * `new_test`: Test - The new test that will be used to update the old test.
    /// 
    /// Returns:
    /// 
    /// A Result<Option<TestModel>, TestsError>
    pub async fn update_test_by_id(
        &self,
        id: &String,
        new_test: Test,
    ) -> Result<Option<TestModel>, Error> {
        let oid = ObjectId::parse_str(id.as_str()).expect("Error parsing id");
        let update = UpdateModifications::Document(
            doc! {"$set": mongodb::bson::to_document(&new_test).unwrap()},
        );
        let updated = self
            .collection
            .find_one_and_update(doc! {"_id": oid}, update, None)
            .await;
        Ok(updated.unwrap())
    }

    /// It gets all the tests from the database.
    /// 
    /// Returns:
    /// 
    /// A vector of tests
    pub async fn get_all_tests(&self) -> Result<Vec<Test>, Error> {
        let tests: Vec<Test> = Vec::new();
        let cursor = self.collection.find(None, None).await.unwrap();
        let tests = cursor.try_collect().await.unwrap();
        Ok(tests)
    }

    /// It takes a string and returns a mongodb::bson::oid::ObjectId
    /// 
    /// Arguments:
    /// 
    /// * `id`: The id of the document to be deleted.
    fn string_to_id(&self, id: String) -> mongodb::bson::oid::ObjectId {
        mongodb::bson::oid::ObjectId::parse_str(id.as_str()).unwrap()
    }

    /// It drops the collection
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
            theme: "addition".to_string(),
            question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            answer: "2".to_string(),
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
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
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
        assert_eq!(result.unwrap().question, "1 + 1".to_string());
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
            theme: "addition".to_string(),
            question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            answer: "2".to_string(),
            level: 1,
        };
        client.create_test(test).await.unwrap();
        let test_id = get_test_id(&"1 + 1".to_string()).await;
        println!("{:?}", test_id.to_string());
        let new_test = TestModel {
            id: Some(test_id),
            theme: "addition".to_string(),
            question: "2 + 2".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            answer: "4".to_string(),
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
            theme: "addition".to_string(),
            question: "1 + 1".to_string(),
            answers: vec!["2", "3", "4"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            answer: "2".to_string(),
            level: 1,
        };
        client.create_test(test).await.unwrap();

        let test_id = get_test_id(&"1 + 1".to_string()).await;
        client.delete_test(&test_id.to_string()).await.unwrap();

        let result = client.get_test_by_id(&test_id.to_string()).await.unwrap();
        assert!(result.is_none())
    }
}
