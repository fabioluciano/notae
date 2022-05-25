#[macro_use]
extern crate rocket;

use common::database::get_connection;
use rocket::serde::json::{json, Json, Value};
use sea_orm::EntityTrait;
use sea_orm::JsonValue;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct UrlTagResponse {
    #[serde(flatten)]
    url: entity::url::Model,
    tags: Vec<entity::tag::Model>,
}

#[get("/url")]
async fn get_all_urls() -> Json<Vec<JsonValue>> {
    let connection = get_connection().await;
    Json(
        entity::url::Entity::find()
            .into_json()
            .all(&connection)
            .await
            .unwrap(),
    )
}

#[get("/url/<id_url>")]
async fn get_url_by_id(id_url: i32) -> Value {
    let connection = get_connection().await;
    let model = entity::url::Entity::find_by_id(id_url)
        .find_with_related(entity::tag::Entity)
        .all(&connection)
        .await
        .unwrap_or_default()[0]
        .to_owned();

    json!(UrlTagResponse {
        url: model.0,
        tags: model.1,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all_urls, get_url_by_id])
}
