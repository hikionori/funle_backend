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
    
}