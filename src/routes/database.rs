use axum::{Json, extract::Query};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct QueryParams {
    query: String,
}

pub async fn query_handler(Query(params): Query<QueryParams>) -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("query".to_string(), params.query);
    response.insert("status".to_string(), "success".to_string());
    Json(response)
}