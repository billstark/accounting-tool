use axum::http::{StatusCode, HeaderMap, header, HeaderValue};
use entity::{transaction_type, transaction};
use sea_orm::prelude::Decimal;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TransactionTypesResponse {
    pub type_name: String,
    pub display_text: String,
    pub description: Option<String>,
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

#[derive(Clone, Debug, Serialize)]
pub struct TransactionResponse {
    pub txn_type: Option<String>,
    pub amount: Decimal,
    pub location: Option<String>,
    pub txn_time: String,
}

impl TransactionResponse {
    pub fn new(db_model: &(transaction::Model, Option<transaction_type::Model>)) -> TransactionResponse {
        return TransactionResponse {
            txn_type: db_model.1.as_ref().map(|t| t.type_name.to_owned()),
            amount: db_model.0.amount.to_owned(),
            location: db_model.0.location.to_owned(),
            txn_time: format!("{}", db_model.0.txn_time.format("%Y-%m-%d %H:%M:%S")).to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateTransactionTypeResponse {
    pub id: i32,
}