use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validify::Validify;

use crate::models::CRUD;
use crate::CATEGORIES_COLLECTION_NAME;

#[derive(Debug, Serialize, Deserialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    name: String,
    description: String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
}

impl CRUD for Category {
    fn collection_name() -> &'static str {
        CATEGORIES_COLLECTION_NAME
    }
}
