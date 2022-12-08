use std::env;

extern crate dotenv;
use dotenv::dotenv;
use rocket::{fairing::Info, futures::TryStreamExt};

use crate::models::info_model::{InfoModel, ContentLevel};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor, options::UpdateModifications
};
use tokio;

pub struct InfosRepo {
    collection: Collection<InfoModel>
}

impl InfosRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(mongo_url).await.expect("Failed to initialize client.");
        let db = client.database("mathdb");

        let collection: Collection<InfoModel> = db.collection("infos");
        InfosRepo { collection }
    }

    pub async fn create_info(&self, info: InfoModel) -> Result<InsertOneResult, Error> {
        let result = self.collection.insert_one(info, None).await;
        Ok(result.unwrap())
    }

    pub async fn get_info(&self, id: &String) -> Result<Option<InfoModel>, Error> {
        let result = self.collection.find_one(doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()}, None).await;
        Ok(result.unwrap())
    }

    pub async fn get_info_by_title(&self, title: &String) -> Result<Option<InfoModel>, Error> {
        let test = self.collection.find_one(doc!{"title": title}, None).await.unwrap();
        Ok(test)
    }

    pub async fn get_all_infos(&self) -> Result<Vec<InfoModel>, Error> {
        let mut cursor: Cursor<InfoModel> = self.collection.find(None, None).await.unwrap();
        let infos = cursor.try_collect().await.unwrap();
        Ok(infos)
    }

    pub async  fn update_info(&self, id: &String, info: InfoModel) -> Result<UpdateResult, Error> {
        let update_doc = UpdateModifications::Document(doc!{"$set": mongodb::bson::to_document(&info).unwrap()});
        let result = self.collection.update_one(doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()}, update_doc, None).await;
        Ok(result.unwrap())
    }

    pub async fn delete_info(&self, id: &String) -> Result<DeleteResult, Error> {
        let result = self.collection.delete_one(doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()}, None).await;
        Ok(result.unwrap())
    }

}

#[cfg(test)]
mod info_repo_tests {
    use std::collections::HashMap;

    use super::*;
    use mongodb::{bson::{oid::ObjectId, doc}, options::ClientOptions, Client};
                

    async fn setup(clean_db: bool) -> InfosRepo {
        env::set_var("MONGODB_URL", "mongodb://root:root@localhost:27017/");
        let client = InfosRepo::init().await;
        if clean_db {
            client.collection.drop(None).await.unwrap();
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
            content_levels
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
        todo!()
    }

    #[tokio::test]
    async fn get_info() {
        todo!()
    }

    #[tokio::test]
    async fn get_info_by_title() {
        todo!()
    }

    #[tokio::test]
    async fn get_all_infos() {
        todo!()
    }

    #[tokio::test]
    async fn update_info() {
        todo!()
    }

    #[tokio::test]
    async fn delete_info() {
        todo!()
    }
}