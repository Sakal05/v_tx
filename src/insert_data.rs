use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use models::{ VerifyTx, NewVerifyTx };

use crate::models;
// use crate::schema::posts;
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}

pub fn insert_tx(
    conn: &mut MysqlConnection,
    to_address: Option<String>,
    from_address: Option<String>,
    tx_hash: Option<String>,
    verify_status: Option<i32>
) -> VerifyTx {
    use crate::schema::verify_tx;

    let new_tx = NewVerifyTx { to_address, from_address, tx_hash, verify_status };

    conn.transaction(|conn| {
        diesel::insert_into(verify_tx::table).values(&new_tx).execute(conn)?;

        verify_tx::table.order(verify_tx::id.desc()).select(VerifyTx::as_select()).first(conn)
    }).expect("Error while saving post")
}
