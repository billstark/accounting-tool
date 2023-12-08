use axum::{Json, extract::State, Router, routing::{get, post}};
use entity::{transaction_type, transaction};
use sea_orm::{EntityTrait, QueryOrder, Order, QueryFilter, ColumnTrait, ActiveModelTrait};
use serde_json::{Value, json};
use tracing_subscriber::filter;

use crate::{errors::{Error, Result}, AppState, models::{responses::{TransactionTypesResponse, TransactionResponse}, requests::CreateTransactionRequest}};
use entity::transaction_type::Entity as TransactionType;
use entity::transaction::Entity as Transaction;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/api/txn", post(create_txn).get(get_txns)).with_state(state)
}

pub async fn get_txns(state: State<AppState>) -> Result<Json<Value>> {
    tracing::info!("--> Receiving request for getting transactions");
    let conn = &state.conn;
    return match Transaction::find().find_also_related(TransactionType).order_by(transaction::Column::TxnTime, Order::Desc).all(conn).await {
        Ok(data) => {
            let inter_resp = data.iter().map(|t| {
                serde_json::to_value(TransactionResponse::new(t)).unwrap()
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

pub async fn create_txn(
    state: State<AppState>,
    Json(create_txn_request): Json<CreateTransactionRequest>
) -> Result<Json<Value>> {
    tracing::info!("--> Receiving request for creating a new transaction");
    println!("{create_txn_request:?}");
    let conn = &state.conn;
    let txn_type = TransactionType::find()
        .filter(transaction_type::Column::TypeName.eq(&create_txn_request.txn_type))
        .one(conn)
        .await;
    if txn_type.is_err() { return Err(Error::InternalServerError); }
    let txn_type_unwrapped = txn_type.unwrap();
    let txn_type_id = txn_type_unwrapped.as_ref().map(|m| (m as &transaction_type::Model).id);

    let inserted_result = create_txn_request.to_model(txn_type_id).insert(conn).await;
    if inserted_result.is_err() { return Err(Error::InternalServerError); }
    let inserted: transaction::Model = inserted_result.unwrap();
    let txn_resp = serde_json::to_value(TransactionResponse::new(&(inserted, txn_type_unwrapped))).unwrap();
    Ok(Json(txn_resp))
}

// pub async fn delete_txn(state: State<AppState>) -> Result<Json<Value>> {
//     tracing::info!("Receiving request for getting transaction types");
//     let conn = &state.conn;
//     return match TransactionType::find().order_by(transaction_type::Column::Id, Order::Asc).all(conn).await {
//         Ok(data) => {
//             let inter_resp = data.iter().map(|t| {
//                 serde_json::to_value(TransactionTypesResponse::new(t)).unwrap()
//             }).collect::<Vec<Value>>();
            
//             tracing::info!("Successfully retrieved transactions types");
//             Ok(Json(Value::Array(inter_resp)))
//         },
//         Err(e) => {
//             tracing::error!("Failed to get transaction types due to error: [{:?}]", e);
//             Err(Error::InternalServerError)
//         }
//     };

// }
