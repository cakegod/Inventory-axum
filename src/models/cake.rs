use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{DATABASE_NAME, PRODUCT_COLLECTION_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cake {
    name: String,
    description: String,
    category: String,
    price: u32,
    in_stock: u32,
    url: String,
    _id: ObjectId,
}

impl Cake {
    async fn collection(db: &Client) -> Collection<Self> {
        db.database(DATABASE_NAME)
            .collection::<Self>(PRODUCT_COLLECTION_NAME)
    }

    pub async fn get_all(db: &Client) -> Result<Vec<Self>, mongodb::error::Error> {
        let cursor = Self::collection(&db).await.find(None, None).await?;
        let products = cursor
            .map(|product| product.unwrap())
            .collect::<Vec<Self>>()
            .await;
        Ok(products)
    }

    pub async fn get_one(db: &Client, id: ObjectId) -> Result<Self, mongodb::error::Error> {
        let cursor = Self::collection(&db)
            .await
            .find_one(doc! {"_id": id}, None)
            .await?;
        let product = cursor.unwrap();
        Ok(product)
    }
}
