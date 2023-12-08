use axum::{Json, extract::State, Router, routing::get};
use entity::transaction_type;
use sea_orm::{EntityTrait, QueryOrder, Order};
use serde_json::{Value, json};

use crate::{errors::{Error, Result}, AppState, models::responses::TransactionTypesResponse};
use entity::transaction_type::Entity as TransactionType;
use entity::transaction::Entity as Transaction;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/api/txn_types", get(get_txn_types)).with_state(state)
}

pub async fn get_txn_types(state: State<AppState>) -> Result<Json<Value>> {
    tracing::info!("Receiving request for getting transaction types");
    let conn = &state.conn;
    return match TransactionType::find().order_by(transaction_type::Column::Id, Order::Asc).all(conn).await {
        Ok(data) => {
            let inter_resp = data.iter().map(|t| {
                serde_json::to_value(TransactionTypesResponse::new(t)).unwrap()
            }).collect::<Vec<Value>>();
            
            tracing::info!("Successfully retrieved transactions types");
            Ok(Json(Value::Array(inter_resp)))
        },
        Err(e) => {
            tracing::error!("Failed to get transaction types due to error: [{:?}]", e);
            Err(Error::InternalServerError)
        }
    };

}
