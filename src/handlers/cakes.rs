use std::str::FromStr;

use axum::{
    async_trait,
    extract::State,
    extract::{FromRequestParts, Path},
    http::request::Parts,
    http::StatusCode,
    Form, Json, RequestPartsExt,
};
use mongodb::{bson::oid::ObjectId, Client};

use crate::models::cake::Cake;

pub struct Id(ObjectId);

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

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Cake>>, StatusCode> {
    match Cake::get_all(&db).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Cake>, StatusCode> {
    match Cake::get_one(&db, id).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn update_one(
    State(db): State<Client>,
    Id(id): Id,
    Form(updated): Form<Cake>,
) -> StatusCode {
    match Cake::update_one(&db, id, updated).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn add_one(State(db): State<Client>, Form(product): Form<Cake>) -> StatusCode {
    match Cake::add_one(&db, product).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
pub async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
    match Cake::delete_one(&db, id).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
