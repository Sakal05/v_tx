use diesel::prelude::*;

// use crate::schema::verify_tx;
use super::schema::verify_tx;

#[derive(Queryable, Selectable)]
#[diesel(table_name = verify_tx)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct VerifyTx {
    pub id: i32,
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub tx_hash: Option<String>,
    pub verify_status: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "verify_tx"]
pub struct NewVerifyTx {
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub tx_hash: Option<String>,
    pub verify_status: Option<i32>,
}

