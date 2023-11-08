use std::str::FromStr;

use axum::{async_trait, Form, Json, RequestPartsExt};
use axum::extract::{FromRequestParts, Path, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_valid::Validified;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use crate::models::CRUD;

pub mod categories;
pub mod products;

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

#[async_trait]
pub trait RestRoutes {
    type Model: CRUD;

    async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Self::Model>>, StatusCode> {
        match Self::Model::get_all(&db).await {
            Ok(products) => Ok(Json(products)),
            Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
        }
    }

    async fn get_one(
        State(db): State<Client>,
        Id(id): Id,
    ) -> Result<Json<Self::Model>, StatusCode> {
        match Self::Model::get_one(&db, id).await {
            Ok(product) => Ok(Json(product)),
            Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
        }
    }

    async fn update_one(
        State(db): State<Client>,
        Id(id): Id,
        Validified(Form(updated)): Validified<Form<Self::Model>>,
    ) -> StatusCode {
        match Self::Model::update_one(&db, id, updated).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
    async fn add_one(
        State(db): State<Client>,
        Validified(Form(model)): Validified<Form<Self::Model>>,
    ) -> StatusCode {
        match Self::Model::add_one(&db, model).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
    async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
        match Self::Model::delete_one(&db, id).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
}

// pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Category>>, StatusCode> {
//     match Category::get_all(&db).await {
//         Ok(products) => Ok(Json(products)),
//         Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
//     }
// }
//
// pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Category>, StatusCode> {
//     match Category::get_one(&db, id).await {
//         Ok(product) => Ok(Json(product)),
//         Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
//     }
// }
//
// pub async fn update_one(
//     State(db): State<Client>,
//     Id(id): Id,
//     Validified(Form(updated)): Validified<Form<Category>>,
// ) -> StatusCode {
//     match Category::update_one(&db, id, updated).await {
//         Ok(_) => StatusCode::NO_CONTENT,
//         Err(_) => StatusCode::BAD_REQUEST,
//     }
// }
// pub async fn add_one(
//     State(db): State<Client>,
//     Validified(Form(model)): Validified<Form<Category>>,
// ) -> StatusCode {
//     match Category::add_one(&db, model).await {
//         Ok(_) => StatusCode::NO_CONTENT,
//         Err(_) => StatusCode::BAD_REQUEST,
//     }
// }
// pub async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
//     match Category::delete_one(&db, id).await {
//         Ok(_) => StatusCode::NO_CONTENT,
//         Err(_) => StatusCode::BAD_REQUEST,
//     }
// }
