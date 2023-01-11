use std::env;

extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::TryStreamExt;

use crate::models::info_model::{InfoModel};

use mongodb::{
    bson::{doc, oid::ObjectId, extjson::de::Error},
    options::UpdateModifications,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor
};
use base64::{
    Engine as _,
    engine::{general_purpose}
};

pub struct InfosRepo {
    collection: Collection<InfoModel>,
}
impl InfosRepo {
    /// It creates a new MongoDB client, connects to the database, and creates a new collection
    /// 
    /// Returns:
    /// 
    /// A new instance of the InfosRepo struct.
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(mongo_url).await.unwrap();
        let db = client.database("mathdb");

        let collection: Collection<InfoModel> = db.collection("infos");
        InfosRepo { collection }
    }

    /// It creates a new info.
    /// 
    /// Arguments:
    /// 
    /// * `info`: InfoModel - The info to be inserted into the database.
    /// 
    /// Returns:
    /// 
    /// The result of the insert_one function.
    pub async fn create_info(&self, info: InfoModel) -> Result<InsertOneResult, Error> {
        let result = self.collection.insert_one(info, None).await;
        Ok(result.unwrap())
    }

    /// It gets the info from the database.
    /// 
    /// Arguments:
    /// 
    /// * `id`: &String
    /// 
    /// Returns:
    /// 
    /// A Result<Option<InfoModel>, InfosError>
    pub async fn get_info(&self, id: &String) -> Result<Option<InfoModel>, Error> {
        let result = self
            .collection
            .find_one(
                doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()},
                None,
            )
            .await;
        Ok(result.unwrap())
    }

    /// It gets the information from the database by title.
    /// 
    /// Arguments:
    /// 
    /// * `title`: &String
    /// 
    /// Returns:
    /// 
    /// A Result<Option<InfoModel>, InfosError>
    pub async fn get_info_by_title(&self, title: &String) -> Result<Option<InfoModel>, Error> {
        let test = self.collection.find_one(doc! {"title": title}, None).await;
        Ok(test.unwrap())
    }

    /// It gets all the infos from the database.
    /// 
    /// Returns:
    /// 
    /// A vector of InfoModel
    pub async fn get_all_infos(&self) -> Result<Vec<InfoModel>, Error> {
        let cursor: Cursor<InfoModel> = self.collection.find(None, None).await.unwrap();
        let infos = cursor.try_collect().await;
        Ok(infos.unwrap())
    }

    /// It takes an id and an InfoModel, and updates the document with the given id with the given
    /// InfoModel
    /// 
    /// Arguments:
    /// 
    /// * `id`: The id of the info to update
    /// * `info`: InfoModel - The InfoModel struct that we created earlier.
    /// 
    /// Returns:
    /// 
    /// A Result<UpdateResult, InfosError>
    pub async fn update_info(
        &self,
        id: &String,
        info: InfoModel,
    ) -> Result<UpdateResult, Error> {
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
        Ok(result.unwrap())
    }

    /// It deletes a document from the database
    /// 
    /// Arguments:
    /// 
    /// * `id`: &String - The id of the info to delete
    /// 
    /// Returns:
    /// 
    /// A DeleteResult
    pub async fn delete_info(&self, id: &String) -> Result<DeleteResult, Error> {
        let result = self
            .collection
            .delete_one(
                doc! {"_id": ObjectId::parse_str(id.as_str()).unwrap()},
                None,
            )
            .await;
        Ok(result.unwrap())
    }

    pub async fn encode_info_data(&self, data: Vec<u8>) -> String {
        general_purpose::STANDARD.encode(data) 
    }

    pub async fn decode_info_data(&self, data: String) -> Vec<u8> {
        general_purpose::STANDARD.decode(data).unwrap()
    }

    /// "If you are sure, drop the collection."
    /// 
    /// The function takes a boolean parameter, `you_are_sure`. If the parameter is `true`, the function
    /// drops the collection. If the parameter is `false`, the function does nothing
    /// 
    /// Arguments:
    /// 
    /// * `you_are_sure`: This is a boolean value that is used to make sure that the user is sure that
    /// they want to drop the collection.
    pub async fn drop_collection(&self, you_are_sure: bool) {
        if !(you_are_sure) {
            return;
        }
        self.collection.drop(None).await.unwrap();
    }
}

#[cfg(test)]
mod info_repo_tests {
    use std::collections::HashMap;

    use crate::models::info_model::ContentLevel;

    use super::*;
    use mongodb::{
        bson::{oid::ObjectId},
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
                data: "test".to_owned(),
            });
            content_levels.insert(i, contents.clone());
        }
        InfoModel {
            id: None,
            theme: "test".to_string(),
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
            theme: "test2".to_string(),
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
