use std::str::FromStr;

use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::{async_trait, extract::State, http::StatusCode, Form, Json, RequestPartsExt};
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
        Ok(Id(ObjectId::from_str(&x).unwrap()))
    }
}

pub struct CustomError(mongodb::error::Error);

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for CustomError
where
    E: Into<mongodb::error::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Cake>>, CustomError> {
    Ok(Json(Cake::get_all(&db).await?))
}

pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Cake>, CustomError> {
    Ok(Json(Cake::get_one(&db, id).await?))
}

pub async fn update_one(State(db): State<Client>, Id(id): Id, Form(updated): Form<Cake>) -> StatusCode {
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
