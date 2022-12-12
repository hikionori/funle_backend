#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::cource_model::*;
use mongodb::{
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
        let mongodb_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(&mongodb_url)
            .await
            .expect("Failed to initialize client.");
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

        self.collection
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

    pub async fn get_cource_id(&self, cource_title: &str) -> Option<String> {
        let filter = doc! {"title": cource_title};
        let cource = self.collection.find_one(filter, None).await.unwrap();
        match cource {
            Some(cource) => Some(cource.id.unwrap().to_hex()),
            None => None,
        }
    }

    pub async fn drop_collection(&self) {
        self.collection.drop(None).await.unwrap();
    }

    // CRUD for levels in cource
    pub async fn add_level(&self, cource_id: &str, level: Level, level_number: i32) {
        let cource_id = ObjectId::parse_str(cource_id).unwrap();
        let cource = self.get(cource_id.to_hex().as_str()).await;
        match cource {
            Some(mut cource) => {
                match cource.levels.get_mut(&level_number) {
                    Some(vec) => {
                        vec.push(level);
                    }
                    None => {
                        let mut vec = Vec::new();
                        vec.push(level);
                        cource.levels.insert(level_number, vec);
                    }
                }
                self.update(cource_id.to_hex().as_str(), cource).await;
            },
            None => {
                println!("Cource with id {} not found", cource_id);
            }
        }
    }

    pub async fn delete_level(&self, cource_id: &str, level_number: i32, level_id: &str) {
        let cource_id = ObjectId::parse_str(cource_id).unwrap();
        let cource = self
            .collection
            .find_one(doc! {"_id": cource_id}, None)
            .await
            .unwrap();
        match cource {
            Some(mut cource) => {
                match cource.levels.get_mut(&level_number) {
                    Some(vec) => {
                        let mut index = 0;
                        for level in vec.iter_mut() {
                            if level.id == level_id {
                                break;
                            }
                            index += 1;
                        }
                        vec.remove(index);
                    }
                    None => {
                        println!("Level with number {} not found", level_number);
                    }
                }
                let cource_id = cource.id.unwrap().to_hex();
                self.update(&cource_id, cource).await;
            }
            None => {
                println!("Cource with id {} not found", cource_id);
            }
        }
    }

    pub async fn update_level(
        &self,
        cource_id: &str,
        level_number: i32,
        level_id: &str,
        new_level: Level,
    ) {
        let cource_id = ObjectId::parse_str(cource_id).unwrap();
        let cource = self
            .collection
            .find_one(doc! {"_id": cource_id}, None)
            .await
            .unwrap();
        match cource {
            Some(cource) => {
                let levels = cource.levels.clone();
                // get level from levels with level number as key and level as value
                // if level number already exists, then push level to vector
                match levels.get(&level_number) {
                    Some(vec) => {
                        let mut index = 0;
                        for level in vec.iter() {
                            if level.id == level_id {
                                break;
                            }
                            index += 1;
                        }
                        let mut new_levels = levels.clone();
                        new_levels.get_mut(&level_number).unwrap()[index] = new_level;
                        let mut new_cource = cource.clone();
                        new_cource.levels = new_levels;
                        self.update(cource_id.to_hex().as_str(), new_cource).await;
                    }
                    None => {
                        println!("Level with number {} not found", level_number);
                    }
                }
            }
            None => {
                println!("Cource with id {} not found", cource_id);
            }
        }
    }

    pub async fn get_level(
        &self,
        cource_id: &str,
        level_number: i32,
        level_id: &str,
    ) -> Option<Level> {
        let cource_id = ObjectId::parse_str(cource_id).unwrap();
        let cource = self
            .collection
            .find_one(doc! {"_id": cource_id}, None)
            .await
            .unwrap();
        match cource {
            Some(cource) => {
                let levels = cource.levels;
                // get level from levels with level number as key and level as value
                // if level number already exists, then push level to vector
                match levels.get(&level_number) {
                    Some(vec) => {
                        let level_id = ObjectId::parse_str(level_id).unwrap();
                        let level = vec.iter().position(|x| x.id == level_id.to_hex());
                        match level {
                            Some(level) => Some(vec[level].clone()),
                            None => {
                                println!("Level with id {} not found", level_id);
                                None
                            }
                        }
                    }
                    None => {
                        println!("Level with number {} not found", level_number);
                        None
                    }
                }
            }
            None => {
                println!("Cource with id {} not found", cource_id);
                None
            }
        }
    }
}

#[cfg(test)]
mod cource_repo_tests {
    use std::collections::HashMap;

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

    async fn str2oid(id: &str) -> ObjectId {
        ObjectId::parse_str(id).unwrap()
    }

    async fn gen_n_cource(n: i32) -> Vec<CourseModel> {
        let mut cource_vec = Vec::new();
        for i in 0..n {
            let cource = CourseModel {
                id: None,
                title: format!("Cource title {}", i),
                description: format!("Cource description {}", i),
                levels: HashMap::new(),
            };
        }
        cource_vec
    }

    #[tokio::test]
    async fn create_test() {
        let cource_repo = setup_cource_db(true).await;
        // let info = InfoModel {
        //     id: None,
        //     title: "Info title".to_string(),
        //     content_levels: HashMap::new(),
        // };
        // create_info_in_db(info).await;
        // let test = TestModel {
        //     id: None,
        //     text_of_question: "1 + 1".to_string(),
        //     correct_answer: "2".to_string(),
        //     answers: vec!["3".to_string(), "4".to_string()],
        //     level: 1,
        // };
        // create_test_in_db(test).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;
        let cource = cource_repo.get(cource_id.unwrap().as_str()).await;

        match cource {
            Some(cource) => {
                assert_eq!(cource.title, "Cource title");
                assert_eq!(cource.description, "Cource description");
            }
            None => {
                panic!("Cource not found");
            }
        }
    }

    #[tokio::test]
    async fn get_test() {
        let cource_repo = setup_cource_db(true).await;
        // let info = InfoModel {
        //     id: None,
        //     title: "Info title".to_string(),
        //     content_levels: HashMap::new(),
        // };
        // create_info_in_db(info).await;
        // let test = TestModel {
        //     id: None,
        //     text_of_question: "1 + 1".to_string(),
        //     correct_answer: "2".to_string(),
        //     answers: vec!["3".to_string(), "4".to_string()],
        //     level: 1,
        // };
        // create_test_in_db(test).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;
        let cource = cource_repo.get(cource_id.unwrap().as_str()).await;

        match cource {
            Some(cource) => {
                assert_eq!(cource.title, "Cource title");
                assert_eq!(cource.description, "Cource description");
            }
            None => {
                panic!("Cource not found");
            }
        }
    }

    #[tokio::test]
    async fn update_test() {
        let cource_repo = setup_cource_db(true).await;
        // let info = InfoModel {
        //     id: None,
        //     title: "Info title".to_string(),
        //     content_levels: HashMap::new(),
        // };
        // create_info_in_db(info).await;
        // let test = TestModel {
        //     id: None,
        //     text_of_question: "1 + 1".to_string(),
        //     correct_answer: "2".to_string(),
        //     answers: vec!["3".to_string(), "4".to_string()],
        //     level: 1,
        // };
        // create_test_in_db(test).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;

        let new_cource = CourseModel {
            id: Some(str2oid(&cource_id.clone().unwrap()).await),
            title: "New cource title".to_string(),
            description: "New cource description".to_string(),
            levels: HashMap::new(),
        };

        cource_repo
            .update(cource_id.clone().unwrap().as_str(), new_cource.clone())
            .await;
        let cource = cource_repo.get(cource_id.unwrap().as_str()).await;
        match cource {
            Some(cource) => {
                assert_eq!(cource.title, new_cource.title);
                assert_eq!(cource.id, new_cource.id);
            }
            None => {
                panic!("Cource not updated")
            }
        }
    }

    #[tokio::test]
    async fn delete_test() {
        let cource_repo = setup_cource_db(true).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };

        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;
        cource_repo
            .delete(cource_id.clone().unwrap().as_str())
            .await;
        let cource = cource_repo.get(cource_id.unwrap().as_str()).await;
        if let Some(_) = cource {
            panic!("Cource not deleted");
        }
    }

    #[tokio::test]
    async fn get_all_ids_test() {
        let cource_repo = setup_cource_db(true).await;
        let cources = gen_n_cource(5).await;
        for cource in cources {
            cource_repo.create(cource.clone()).await;
        }
        let ids = cource_repo.get_all_ids().await;
        match ids {
            Some(ids) => {
                assert!(!ids.is_empty());
            }
            None => {
                panic!("Cources not found");
            }
        }
    }

    #[tokio::test]
    async fn get_all_test() {
        let cource_repo = setup_cource_db(true).await;
        let cources = gen_n_cource(5).await;
        for cource in cources {
            cource_repo.create(cource.clone()).await;
        }
        let cources = cource_repo.get_all().await;
        match cources {
            Some(cources) => {
                assert!(!cources.is_empty());
            }
            None => {
                panic!("Cources not found");
            }
        }
    }

    #[tokio::test]
    async fn add_level_test() {
        let cource_repo = setup_cource_db(true).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(cource.title.as_str()).await;

        let info = InfoModel {
            id: None,
            title: "Info title".to_string(),
            content_levels: HashMap::new(),
        };
        let info_id = create_info_in_db(info).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            correct_answer: "2".to_string(),
            answers: vec!["3".to_string(), "4".to_string()],
            level: 1,
        };
        let test_id = create_test_in_db(test).await;

        let test_cell = Level {
            id: test_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };
        let info_cell = Level {
            id: info_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };

        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 2)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 3)
            .await;

        let cource = cource_repo.get(cource_id.unwrap().as_str()).await.unwrap();
        let levels = cource.levels;
        assert_eq!(levels.get(&1).unwrap().len(), 2);
    }

    #[tokio::test]
    async fn delete_level_test() {
        let cource_repo = setup_cource_db(true).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;

        let info = InfoModel {
            id: None,
            title: "Info title".to_string(),
            content_levels: HashMap::new(),
        };
        let info_id = create_info_in_db(info).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            correct_answer: "2".to_string(),
            answers: vec!["3".to_string(), "4".to_string()],
            level: 1,
        };
        let test_id = create_test_in_db(test).await;

        let test_cell = Level {
            id: test_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };
        let info_cell = Level {
            id: info_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };

        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 2)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 3)
            .await;

        cource_repo.delete_level(cource_id.clone().unwrap().as_str(), 1, test_cell.id.as_str()).await;
        assert_eq!(cource_repo.get(cource_id.unwrap().as_str()).await.unwrap().levels.len(), 3);
    }

    #[tokio::test]
    async fn update_level_test() {
        let cource_repo = setup_cource_db(true).await;
        let cource = CourseModel {
            id: None,
            title: "Cource title".to_string(),
            description: "Cource description".to_string(),
            levels: HashMap::new(),
        };
        cource_repo.create(cource.clone()).await;

        let cource_id = cource_repo.get_cource_id(&cource.title).await;

        let info = InfoModel {
            id: None,
            title: "Info title".to_string(),
            content_levels: HashMap::new(),
        };
        let info_id = create_info_in_db(info).await;
        let test = TestModel {
            id: None,
            text_of_question: "1 + 1".to_string(),
            correct_answer: "2".to_string(),
            answers: vec!["3".to_string(), "4".to_string()],
            level: 1,
        };
        let test_id = create_test_in_db(test).await;

        let test_cell = Level {
            id: test_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };
        let info_cell = Level {
            id: info_id.clone(),
            title: "Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };

        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 1)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), test_cell.copy(), 2)
            .await;
        cource_repo
            .add_level(cource_id.clone().unwrap().as_str(), info_cell.copy(), 3)
            .await;

        let new_test_cell = Level {
            id: test_id.clone(),
            title: "New Level title".to_string(),
            mini_image: "Bytes".to_string().as_bytes().to_vec(),
        };
        cource_repo
            .update_level(cource_id.clone().unwrap().as_str(), 1, test_cell.id.as_str(), new_test_cell.copy())
            .await;
        
        assert_eq!(cource_repo.get(cource_id.unwrap().as_str()).await.unwrap().levels.get(&1).unwrap().get(0).unwrap().title, "New Level title");
    }
}
