use mongodb::{bson::doc, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};
use validify::Validify;

use crate::models::CRUD;
use crate::PRODUCT_COLLECTION_NAME;

#[derive(Debug, Serialize, Deserialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub description: String,
    pub category: ObjectId,
    pub price: u32,
    pub number_in_stock: u32,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
}

impl CRUD for Product {
    fn collection_name() -> &'static str {
        PRODUCT_COLLECTION_NAME
    }
}

