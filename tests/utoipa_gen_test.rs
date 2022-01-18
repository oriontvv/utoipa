#![cfg(feature = "actix_extras")]
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use utoipa::openapi_spec;
use utoipa::{Component, OpenApi};

#[derive(Deserialize, Serialize, Component)]
struct Pet {
    id: u64,
    name: String,
    age: Option<i32>,
}

mod pet_api {
    use super::*;

    /// Get pet by id
    ///
    /// Get pet from database by pet database id  
    #[utoipa::path(
        get,
        path = "/pets/{id}"
        responses = [
            (status = 200, description = "Pet found succesfully", body = Pet),
            (status = 404, description = "Pet was not found")
        ],
        params = [
            ("id" = u64, path, description = "Pet database id to get Per for"),
        ]
    )]
    async fn get_pet_by_id(pet_id: u64) -> Pet {
        Pet {
            id: pet_id,
            age: None,
            name: "lightning".to_string(),
        }
    }
}

#[test]
#[ignore = "this is just a test bed to run macros"]
fn derive_openapi() {
    #[derive(OpenApi, Default)]
    #[openapi(handlers = [pet_api::get_pet_by_id], components = [Pet])]
    struct ApiDoc;

    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
