use std::str::FromStr;

use anyhow::anyhow;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, RequestPartsExt};
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::DATABASE_NAME;

pub mod cake;
pub mod category;

pub struct Id(pub ObjectId);

#[async_trait]
impl<S> FromRequestParts<S> for Id
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let Path(x) = parts
            .extract::<Path<String>>()
            .await
            .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

        if let Ok(id) = ObjectId::from_str(&x) {
            Ok(Id(id))
        } else {
            Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
    }
}

pub async fn get_all<T: Serialize + DeserializeOwned>(
    db: &Client,
    collection_name: &str,
) -> anyhow::Result<Vec<T>> {
    let collection = db.database(DATABASE_NAME).collection::<T>(collection_name);
    let cursor = collection.find(None, None).await?;
    let products = cursor
        .map(|product| product.unwrap())
        .collect::<Vec<T>>()
        .await;
    Ok(products)
}

pub async fn get_one<T: Serialize + DeserializeOwned + Unpin + Send + Sync>(
    db: &Client,
    id: ObjectId,
    collection_name: &str,
) -> anyhow::Result<T> {
    let cursor = db
        .database(DATABASE_NAME)
        .collection::<T>(collection_name)
        .find_one(doc! {"_id": id}, None)
        .await?;
    let product = cursor.unwrap();
    Ok(product)
}

pub async fn delete_one<T: Serialize + DeserializeOwned>(
    db: &Client,
    id: ObjectId,
    collection_name: &str,
) -> anyhow::Result<()> {
    db.database(DATABASE_NAME)
        .collection::<T>(collection_name)
        .delete_one(doc! {"_id": id}, None)
        .await?;
    Ok(())
}
pub async fn update_one<T: Serialize + DeserializeOwned>(
    db: &Client,
    id: ObjectId,
    updated: T,
    collection_name: &str,
) -> anyhow::Result<()> {
    // It's easier to replace the entire item instead of updating specific fields...
    let result = db
        .database(DATABASE_NAME)
        .collection::<T>(collection_name)
        .replace_one(doc! {"_id": id}, updated, None)
        .await?;
    if result.modified_count > 0 {
        Ok(())
    } else {
        Err(anyhow!("The document was not modified"))
    }
}

pub async fn add_one<T: Serialize + DeserializeOwned>(
    db: &Client,
    product: T,
    collection_name: &str,
) -> anyhow::Result<()> {
    db.database(DATABASE_NAME)
        .collection::<T>(collection_name)
        .insert_one(product, None)
        .await?;
    Ok(())
}
