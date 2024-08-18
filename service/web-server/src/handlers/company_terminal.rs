use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use futures::TryStreamExt;
use lib_core::AppError;
use lib_entity::{mongo::Company, AppState};
use lib_utils::HttpResult;
use mongodb::{bson::doc, Collection};

pub async fn company_list(
    Query(name): Query<Option<String>>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let collection: Collection<Company> = state.mongo_database.collection("Company");
    let doc = match name {
        Some(name) => doc! {
             "name": name
        },
        None => doc! {},
    };
    let cursor = collection.find(doc).await?;
    let company_list: Vec<Company> = cursor.try_collect().await?;
    Ok(HttpResult::ok(company_list))
}
