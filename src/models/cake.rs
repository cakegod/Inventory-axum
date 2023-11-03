use anyhow::Result;
use futures::StreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{DATABASE_NAME, PRODUCT_COLLECTION_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cake {
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: u32,
    pub in_stock: u32,
    pub url: String,
    #[serde(skip_deserializing)]
    pub _id: ObjectId,
}

impl Cake {
    fn collection(db: &Client) -> Collection<Self> {
        db.database(DATABASE_NAME)
            .collection::<Self>(PRODUCT_COLLECTION_NAME)
    }

    pub async fn get_all(db: &Client) -> Result<Vec<Self>> {
        let cursor = Self::collection(&db).find(None, None).await?;
        let products = cursor
            .map(|product| product.unwrap())
            .collect::<Vec<Self>>()
            .await;
        Ok(products)
    }

    pub async fn get_one(db: &Client, id: ObjectId) -> Result<Self> {
        let cursor = Self::collection(&db)
            .find_one(doc! {"_id": id}, None)
            .await?;
        let product = cursor.unwrap();
        Ok(product)
    }

    pub async fn update_one(db: &Client, id: ObjectId, updated: Self) -> Result<()> {
        // It's easier to replace the entire item instead of updating specific fields...
        Self::collection(&db)
            .replace_one(doc! {"_id": id}, updated, None)
            .await?;
        Ok(())
    }

    pub async fn add_one(db: &Client, product: Self) -> Result<()> {
        Self::collection(&db).insert_one(product, None).await?;
        Ok(())
    }

    pub async fn delete_one(db: &Client, id: ObjectId) -> Result<()> {
        Self::collection(&db)
            .delete_one(doc! {"_id": id}, None)
            .await?;
        Ok(())
    }
}
