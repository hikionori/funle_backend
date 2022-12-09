use std::env;

extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::info_model::{ContentLevel, InfoModel};
use crate::utils::errors::TestsError;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    options::UpdateModifications,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor,
};
use tokio;

pub struct InfosRepo {
    collection: Collection<InfoModel>,
}

impl InfosRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(mongo_url).await.unwrap();
        let db = client.database("mathdb");

        let collection: Collection<InfoModel> = db.collection("infos");
        InfosRepo { collection }
    }

    pub async fn create_info(&self, info: InfoModel) -> Result<InsertOneResult, TestsError> {
        let result = self.collection.insert_one(info, None).await;
        match result {
            Ok(insert_result) => Ok(insert_result),
            Err(_) => Err(TestsError::WeAreCanNotCreateTest),
        }
    }

    pub async fn get_info(&self, id: &String) -> Result<Option<InfoModel>, TestsError> {
        let result = self
            .collection
            .find_one(
                doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()},
                None,
            )
            .await;
        match result {
            Ok(res) => Ok(res),
            Err(_) => Err(TestsError::WeAreCanNotGetTest),
        }
    }

    pub async fn get_info_by_title(&self, title: &String) -> Result<Option<InfoModel>, TestsError> {
        let test = self.collection.find_one(doc! {"title": title}, None).await;
        match test {
            Ok(test) => Ok(test),
            Err(_) => Err(TestsError::WeAreCanNotGetTest),
        }
    }

    pub async fn get_all_infos(&self) -> Result<Vec<InfoModel>, TestsError> {
        let mut cursor: Cursor<InfoModel> = self.collection.find(None, None).await.unwrap();
        let infos = cursor.try_collect().await;
        match infos {
            Ok(infos) => Ok(infos),
            Err(_) => Err(TestsError::WeAreCanNotGetTests),
        }
    }

    pub async fn update_info(
        &self,
        id: &String,
        info: InfoModel,
    ) -> Result<UpdateResult, TestsError> {
        let update_doc = UpdateModifications::Document(
            doc! {"$set": mongodb::bson::to_document(&info).unwrap()},
        );
        let result = self
            .collection
            .update_one(
                doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()},
                update_doc,
                None,
            )
            .await;
        match result {
            Ok(update_result) => Ok(update_result),
            Err(_) => Err(TestsError::WeAreCanNotUpdateTest),
        }
    }

    pub async fn delete_info(&self, id: &String) -> Result<DeleteResult, TestsError> {
        let result = self
            .collection
            .delete_one(
                doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()},
                None,
            )
            .await;
        match result {
            Ok(delete_result) => Ok(delete_result),
            Err(_) => Err(TestsError::WeAreCanNotDeleteTest),
        }
    }

    async fn drop_collection(&self, you_are_sure: bool) {
        if !(you_are_sure) {
            return;
        }
        self.collection.drop(None).await.unwrap();
    }
}

#[cfg(test)]
mod info_repo_tests {
    use std::collections::HashMap;

    use super::*;
    use mongodb::{
        bson::{doc, oid::ObjectId},
        options::ClientOptions,
        Client,
    };

    async fn setup(clean_db: bool) -> InfosRepo {
        env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");
        let client = InfosRepo::init().await;
        if clean_db {
            client.drop_collection(true).await
        }
        client
    }

    async fn gen_info() -> InfoModel {
        let mut contents: Vec<ContentLevel> = Vec::new();
        // gen HashMap with content levels
        let mut content_levels: HashMap<i32, Vec<ContentLevel>> = HashMap::new();
        for i in 0..3 {
            contents.push(ContentLevel {
                content_type: "text".to_string(),
                data: "test".as_bytes().to_vec(),
            });
            content_levels.insert(i, contents.clone());
        }
        InfoModel {
            id: None,
            title: "test".to_string(),
            content_levels,
        }
    }

    async fn gen_n_infos(n: i32) -> Vec<InfoModel> {
        let mut infos: Vec<InfoModel> = Vec::new();
        for _ in 0..n {
            infos.push(gen_info().await);
        }
        infos
    }

    async fn get_info_id(title: &String) -> String {
        let client = setup(false).await;
        let info = client.get_info_by_title(title).await.unwrap().unwrap();
        info.id.unwrap().to_string()
    }

    #[tokio::test]
    async fn create_info() {
        let client = setup(true).await;
        let info = gen_info().await;
        client.create_info(info.clone()).await.unwrap();
        let info_id = get_info_id(&info.title).await;
        let info_in_db = client.get_info(&info_id).await.unwrap().unwrap();
        assert_eq!(info_in_db.title, info.title)
    }

    #[tokio::test]
    async fn get_info() {
        let client = setup(true).await;
        let info = gen_info().await;
        client.create_info(info.clone()).await.unwrap();

        let result = client
            .get_info(&get_info_id(&info.title).await)
            .await
            .unwrap()
            .unwrap();

        assert!(result.content_levels == info.content_levels)
    }

    #[tokio::test]
    async fn get_info_by_title() {
        let client = setup(true).await;
        let info = gen_info().await;
        client.create_info(info.clone()).await.unwrap();

        let result = client
            .get_info_by_title(&"test".to_string())
            .await
            .unwrap()
            .unwrap();
        assert!(result.content_levels == info.content_levels)
    }

    #[tokio::test]
    async fn get_all_infos() {
        let client = setup(true).await;
        let infos = gen_n_infos(5).await;
        for info in infos.clone() {
            client.create_info(info).await.unwrap();
        }

        let result = client.get_all_infos().await.unwrap();
        assert!(!result.is_empty())
    }

    #[tokio::test]
    async fn update_info() {
        let client = setup(true).await;
        let info = gen_info().await;
        client.create_info(info.clone()).await.unwrap();

        let info_id = get_info_id(&"test".to_string()).await;
        let new_info = InfoModel {
            id: ObjectId::parse_str(info_id.clone().as_str()).ok(),
            title: "test2".to_string(),
            content_levels: info.content_levels,
        };

        let result = client.update_info(&info_id, new_info).await;
        match result {
            Ok(_) => {
                let info_in_db = client.get_info(&info_id).await.unwrap().unwrap();
                assert_eq!(info_in_db.title, "test2".to_string());
            }
            Err(e) => {
                let error_message = e.to_string();
                panic!("{}", error_message);
            }
        }
    }

    #[tokio::test]
    async fn delete_info() {
        let client = setup(true).await;
        let info = gen_info().await;
        client.create_info(info).await.unwrap();

        let info_id = get_info_id(&"test".to_string()).await;
        let result = client.delete_info(&info_id).await.unwrap();
        assert!(result.deleted_count > 0);
    }
}
