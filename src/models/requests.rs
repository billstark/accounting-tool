use std::{any::Any, str::FromStr};
use chrono::{DateTime, FixedOffset};
use entity::{transaction, transaction_type};
use sea_orm::{ActiveValue::Set, prelude::{Decimal, DateTimeWithTimeZone, ChronoDateTimeLocal, ChronoDateTimeWithTimeZone}};
use serde::{Deserialize, de};
use uuid::Uuid;
use std::fmt;

struct DateTimeVisitor;

impl<'de> de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::DateTime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
            Ok(t) => Ok(t),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

fn from_timestamp<'de, D>(d: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(DateTimeVisitor)
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub txn_type: String,
    pub amount: f32,
    pub location: Option<String>,
    #[serde(deserialize_with = "from_timestamp")]
    pub txn_time: DateTime<FixedOffset>,
}

impl CreateTransactionRequest {
    pub fn to_model(self, txn_type_id: Option<i32>) -> transaction::ActiveModel {

        transaction::ActiveModel {
            id: Set(format!("{}", Uuid::new_v4())),
            txn_type: Set(txn_type_id),
            amount: Set(Decimal::from_f32_retain(self.amount).unwrap()),
            location: Set(self.location),
            txn_time: Set(self.txn_time),
            created_at: Default::default(),
            created_by: Set("HBB".to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTransactionTypeRequest {
    pub type_name: String,
    pub display_text: String,
    pub desc: Option<String>,
}

impl CreateTransactionTypeRequest {
    pub fn to_model(self) -> transaction_type::ActiveModel {
        transaction_type::ActiveModel { 
            type_name: Set(self.type_name),
            display_text: Set(self.display_text),
            desc: Set(self.desc),
            ..Default::default()
        }
    }
}
