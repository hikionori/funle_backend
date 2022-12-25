#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::cource_model::*;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
};

pub struct CourceRepo {
    collection: Collection<CourseModel>,
}

impl CourceRepo {
    /// It connects to the database and returns a collection.
    ///
    /// Returns:
    ///
    /// A new instance of the struct `MongoDB`
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

    /// It creates a new course.
    ///
    /// Arguments:
    ///
    /// * `cource`: CourseModel
    ///
    /// Returns:
    ///
    /// InsertOneResult
    pub async fn create(&self, cource: CourseModel) -> InsertOneResult {
        self.collection.insert_one(cource, None).await.unwrap()
    }

    /// > This function takes an id as a string, converts it to an ObjectId, creates a filter, and then
    /// uses the filter to find a document in the database
    ///
    /// Arguments:
    ///
    /// * `id`: &str - The id of the course we want to get
    ///
    /// Returns:
    ///
    /// Option<CourseModel>
    pub async fn get(&self, id: &str) -> Option<CourseModel> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};

        self.collection.find_one(filter, None).await.unwrap()
    }

    /// It updates a course in the database.
    ///
    /// Arguments:
    ///
    /// * `id`: &str,
    /// * `cource`: The new data to be inserted into the database.
    ///
    /// Returns:
    ///
    /// Option<CourseModel>
    pub async fn update(&self, id: &str, cource: CourseModel) -> Option<CourseModel> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};

        self.collection
            .find_one_and_replace(filter, cource, None)
            .await
            .unwrap()
    }

    /// It deletes a document from the database
    ///
    /// Arguments:
    ///
    /// * `id`: &str - The id of the document to delete.
    ///
    /// Returns:
    ///
    /// DeleteResult
    pub async fn delete(&self, id: &str) -> DeleteResult {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};

        self.collection.delete_one(filter, None).await.unwrap()
    }

    /// > This function returns a vector of all the courses in the database
    ///
    /// Returns:
    ///
    /// A vector of CourseModel
    pub async fn get_all(&self) -> Option<Vec<CourseModel>> {
        let cursor = self.collection.find(None, None).await.unwrap();
        let cources = cursor.try_collect::<Vec<CourseModel>>().await.unwrap();
        Some(cources)
    }

    /// It takes a string as an argument, and returns an option of a string
    ///
    /// Arguments:
    ///
    /// * `cource_title`: The title of the course you want to get the ID of.
    ///
    /// Returns:
    ///
    /// Option<String>
    pub async fn get_cource_id(&self, cource_title: &str) -> Option<String> {
        let filter = doc! {"title": cource_title};
        let cource = self.collection.find_one(filter, None).await.unwrap();
        match cource {
            Some(cource) => Some(cource.id.unwrap().to_hex()),
            None => None,
        }
    }

    /// It drops the collection
    pub async fn drop_collection(&self) {
        self.collection.drop(None).await.unwrap();
    }

    // CRUD for levels in cource
    /// It adds a level to a course.
    ///
    /// Arguments:
    ///
    /// * `cource_id`: The id of the cource to which the level is to be added.
    /// * `level`: Level
    /// * `level_number` or `level`: i32
    pub async fn add_level(&self, cource_id: &str, level: Level, layer_number: i32) {
        // algorithm
        // get cource -> get levels -> add level to levels -> update cources with new levels
        // if layer is not empty add level to vec
        // else create new laeyr
        let cource = self.get(cource_id).await;
        match cource {
            Some(mut cource) => {
                let mut levels = cource.levels;
                if levels.contains_key(&layer_number) {
                    let layer = levels.get_mut(&layer_number).unwrap();
                    layer.push(level);
                } else {
                    let mut layer = Vec::new();
                    layer.push(level);
                    levels.insert(layer_number, layer);
                }
                cource.levels = levels;
                self.update(cource_id, cource).await;
            }
            None => {}
        }
    }

    /// It deletes a level from a cource.
    ///
    /// Arguments:
    ///
    /// * `cource_id`: The id of the course to which the level belongs.
    /// * `level_number`: The number of the level you want to delete.
    /// * `level_id`: The id of the level to be deleted
    pub async fn delete_level(&self, cource_id: &str, level_number: i32, level_id: &str) {
        // algorithm
        // get cource -> get levels -> delete level from levels -> update cources with new levels
        let cource = self.get(cource_id).await;
        match cource {
            Some(mut cource) => {
                let mut levels = cource.levels;
                let layer = levels.get_mut(&level_number).unwrap();
                let level = layer.iter().position(|level| level.id == level_id);
                match level {
                    Some(level) => {
                        layer.remove(level);
                    }
                    None => {}
                }
                cource.levels = levels;
                self.update(cource_id, cource).await;
            }
            None => {}
        }
    }

    /// It takes a cource id, a level number, a level id and a new level as arguments, finds the cource
    /// with the given cource id, finds the level with the given level number and level id, and replaces
    /// the level with the new level
    ///
    /// Arguments:
    ///
    /// * `cource_id`: The id of the cource to update
    /// * `level_number`: i32,
    /// * `level_id`: The id of the level to be updated
    /// * `new_level`: Level - new level to be updated
    pub async fn update_level(
        &self,
        cource_id: &str,
        level_number: i32,
        level_id: &str,
        new_level: Level,
    ) {
        // algorithm
        // get cource -> get levels -> update level in levels -> update cources with new levels
        let cource = self.get(cource_id).await;
        match cource {
            Some(mut cource) => {
                let mut levels = cource.levels;
                let layer = levels.get_mut(&level_number).unwrap();
                let level = layer.iter().position(|level| level.id == level_id);
                match level {
                    Some(level) => {
                        layer.remove(level);
                        layer.insert(level, new_level);
                    }
                    None => {}
                }
                cource.levels = levels;
                self.update(cource_id, cource).await;
            }
            None => {}
        }
    }

    /// > Get level from cource with cource id and level number and level id
    ///
    /// Arguments:
    ///
    /// * `cource_id`: The id of the cource
    /// * `level_number`: The number of the level you want to get.
    /// * `level_id`: The id of the level you want to get
    ///
    /// Returns:
    ///
    /// Option<Level>
    pub async fn get_level(
        &self,
        cource_id: &str,
        level_number: i32,
        level_id: &str,
    ) -> Option<Level> {
        let cource = self.get(cource_id).await;
        match cource {
            Some(cource) => {
                let levels = cource.levels;
                let layer = levels.get(&level_number).unwrap();
                let level = layer.iter().find(|level| level.id == level_id);
                match level {
                    Some(level) => Some(level.clone()),
                    None => None,
                }
            }
            None => None,
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
            cource_vec.push(cource);
        }
        cource_vec
    }

    #[tokio::test]
    async fn create_test() {
        let cource_repo = setup_cource_db(true).await;

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
        if cource.is_some() {
            panic!("Cource not deleted");
        }
    }

    #[tokio::test]
    async fn get_all_test() {
        let cource_repo = setup_cource_db(true).await;
        let cource_vec = gen_n_cource(10).await;
        for cource in cource_vec {
            cource_repo.create(cource.clone()).await;
        }

        let cource_vec = cource_repo.get_all().await;
        match cource_vec {
            Some(cource_vec) => {
                assert_eq!(cource_vec.len(), 10);
            }
            None => {
                panic!("Cource not found");
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

        let cource = cource_repo.get(cource_id.unwrap().as_str()).await;
        match cource {
            Some(cource) => {
                let levels = cource.levels;
                assert_eq!(levels.get(&1).unwrap().len(), 2);
            }
            None => {
                panic!("Cource not found");
            }
        }
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

        cource_repo
            .delete_level(
                cource_id.clone().unwrap().as_str(),
                1,
                test_cell.id.as_str(),
            )
            .await;
        assert_eq!(
            cource_repo
                .get(cource_id.unwrap().as_str())
                .await
                .unwrap()
                .levels
                .len(),
            3
        );
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
            .update_level(
                cource_id.clone().unwrap().as_str(),
                1,
                test_cell.id.as_str(),
                new_test_cell.copy(),
            )
            .await;

        assert_eq!(
            cource_repo
                .get(cource_id.unwrap().as_str())
                .await
                .unwrap()
                .levels
                .get(&1)
                .unwrap()
                .get(0)
                .unwrap()
                .title,
            "New Level title"
        );
    }
}
