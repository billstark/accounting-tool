use axum::{Json, extract::State, Router, routing::{get, post}};
use entity::transaction_type;
use sea_orm::{EntityTrait, QueryOrder, Order};
use serde_json::Value;

use crate::{errors::{Error, Result}, models::{requests::CreateTransactionTypeRequest, responses::{CreateTransactionTypeResponse, TransactionTypesResponse}}, AppState};
use entity::transaction_type::Entity as TransactionType;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/api/txn_types", post(create_txn_type).get(get_txn_types)).with_state(state)
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

pub async fn create_txn_type(state: State<AppState>, txn_type_fc: Json<Value>) -> Result<Json<Value>> {
    tracing::info!("Receiving request to create a new transaction type {:?}", txn_type_fc);
    
    let txn_type_model_res = serde_json::from_str::<CreateTransactionTypeRequest>(txn_type_fc.to_string().as_str());
    if txn_type_model_res.is_err() {
        tracing::error!("Invalid request body: {:?}", txn_type_fc);
        return Err(Error::BadRequest);
    }

    let db_model = txn_type_model_res.unwrap().to_model();
    let conn = &state.conn;
    return match transaction_type::Entity::insert(db_model).exec(conn).await {
        Ok(data) => {
            let inserted_id = data.last_insert_id;
            tracing::info!("Successfully inserted transaction with id {}", inserted_id);
            Ok(Json(serde_json::to_value(CreateTransactionTypeResponse { id: inserted_id }).unwrap()))
        },
        Err(e) => {
            tracing::error!("Failed to insert transaction with value {:?}. Error: [{:?}]", txn_type_fc, e);
            Err(Error::InternalServerError)
        }
    };
}