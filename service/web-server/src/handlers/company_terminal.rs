use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use lib_core::AppError;
use lib_entity::{
    mongo::{Company, CompanyListResponse},
    AppState,
};
use lib_utils::HttpResult;
use mongodb::{bson::doc, Collection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CompanyListParam {
    pub name: Option<String>,
}

pub async fn company_list(
    Query(param): Query<CompanyListParam>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let collection: Collection<Company> = state.mongo_database.collection("Company");
    let doc = match param.name {
        Some(name) => doc! {
             "name": name
        },
        None => doc! {},
    };
    let cursor = collection.find(doc).await?;
    let company_list: Vec<CompanyListResponse> = cursor
        .try_collect::<Vec<Company>>()
        .await?
        .into_iter()
        .map(CompanyListResponse::from)
        .collect();

    Ok(HttpResult::ok(company_list))
}
