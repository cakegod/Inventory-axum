use anyhow::{anyhow, Result};
use futures::StreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{DATABASE_NAME, PRODUCT_COLLECTION_NAME};

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

impl Cake {
    fn collection(db: &Client) -> Collection<Self> {
        db.database(DATABASE_NAME)
            .collection::<Self>(PRODUCT_COLLECTION_NAME)
    }

    pub async fn get_all(db: &Client) -> Result<Vec<Self>> {
        let cursor = Self::collection(db).find(None, None).await?;
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
        let result = Self::collection(&db)
            .replace_one(doc! {"_id": id}, updated, None)
            .await?;
        if result.modified_count > 0 {
            Ok(())
        } else {
            Err(anyhow!("The document was not modified"))
        }
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
