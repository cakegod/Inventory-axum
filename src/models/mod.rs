use anyhow::anyhow;
use axum::async_trait;
use futures::StreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::DATABASE_NAME;

pub mod category;
pub mod product;

#[async_trait]
pub trait CRUD: Serialize + DeserializeOwned + Unpin + Send + Sync {
    fn collection_name() -> &'static str;

    async fn get_all(db: &Client) -> anyhow::Result<Vec<Self>> {
        let collection = db
            .database(DATABASE_NAME)
            .collection(Self::collection_name());
        let cursor = collection.find(None, None).await?;
        let products = cursor
            .map(|product| product.unwrap())
            .collect::<Vec<Self>>()
            .await;
        Ok(products)
    }

    async fn get_one(db: &Client, id: ObjectId) -> anyhow::Result<Self> {
        let cursor = db
            .database(DATABASE_NAME)
            .collection(Self::collection_name())
            .find_one(doc! {"_id": id}, None)
            .await?;
        let product = cursor.unwrap();
        Ok(product)
    }

    async fn delete_one(db: &Client, id: ObjectId) -> anyhow::Result<()> {
        db.database(DATABASE_NAME)
            .collection::<Self>(Self::collection_name())
            .delete_one(doc! {"_id": id}, None)
            .await?;
        Ok(())
    }
    async fn update_one(db: &Client, id: ObjectId, updated: Self) -> anyhow::Result<()> {
        // It's easier to replace the entire item instead of updating specific fields...
        let result = db
            .database(DATABASE_NAME)
            .collection(Self::collection_name())
            .replace_one(doc! {"_id": id}, updated, None)
            .await?;
        if result.modified_count > 0 {
            Ok(())
        } else {
            Err(anyhow!("The document was not modified"))
        }
    }

    async fn add_one(db: &Client, product: Self) -> anyhow::Result<()> {
        db.database(DATABASE_NAME)
            .collection(Self::collection_name())
            .insert_one(product, None)
            .await?;
        Ok(())
    }
}
