use mongodb::{bson::doc, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cake {
    pub name: String,
    pub description: String,
    pub category: ObjectId,
    pub price: u32,
    pub number_in_stock: u32,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
}
