use axum::http::{StatusCode, HeaderMap, header, HeaderValue};
use entity::transaction_type;
use serde::Serialize;

#[derive(Serialize)]
pub struct TransactionTypesResponse {
    type_name: String,
    display_text: String,
    description: Option<String>,
}

impl TransactionTypesResponse {
    pub fn new(db_model: &transaction_type::Model) -> TransactionTypesResponse {
        return TransactionTypesResponse { 
            type_name: db_model.type_name.to_owned(),
            display_text: db_model.display_text.to_owned(),
            description: db_model.desc.to_owned()
        }
    }
}
