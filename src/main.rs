use std::{ thread, time, env };

// use network_fee_verify::verify_account_balance;


pub mod query;
pub mod send_raw_tx;
pub mod models;
use web3_rpc::web3::Web3;
pub mod insert_data;
use insert_data::*;
pub mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required arguments are provided
    if args.len() < 3 {
        eprintln!("Usage: cargo run <network> <raw_tx>");
        return Ok(());
    }

    let network = &args[1];
    let raw_tx = &args[2];

    let hash = send_raw_tx::send_raw_tx(network.to_owned(), raw_tx.to_owned()).await;
    /* Get Transaction Receipt */
    let h = match hash {
        Ok(h) => h,
        Err(e) => "0x".to_owned(),
    };

    thread::sleep(time::Duration::from_secs(3));
    // println!("Receipt address: {}", &h);

    let rpc_url = match query::query_rpc(network.to_owned()) {
        Ok(res) => {
            // println!("Successfully obtained RPC URL: {}", res);
            res
        }
        Err(e) => {
            println!("Error finding network: {}", e);
            return Err(anyhow::anyhow!("Network not found"));
        }
    };

    println!("RPC url: {}", &rpc_url);

    let public_node_url = &rpc_url;

    let rpc = Web3::new(public_node_url.to_string());

    if h != "0x" {
        let r = rpc.eth_get_transaction_receipt(h.as_str()).await?;
        thread::sleep(time::Duration::from_secs(3));
        let mut to_address: String = "0x".to_owned();
        let mut from_address: String = "0x".to_owned();
        let mut tx_hash: String = "0x".to_owned();

        let block_hash = match r.result {
            Some(v) => {
                to_address = v.to.clone();
                from_address = v.from.clone();
                tx_hash = v.transaction_hash.clone();
                println!("Success, Block hash: {:#?}", v);
                v.block_hash.clone()
            }
            None => {
                println!("Fail to query block hash");
                "0x".to_string()
            }
        };

        let b_h = block_hash.as_str();
        let eth_block = rpc.eth_get_block_by_hash(b_h, true).await?;

        // println!("Block Num: {:#?}", &eth_block);

        let mut block_confirmation: u128 = 0;

        match eth_block.result {
            Some(block) => {
                // get current block of inside the  network
                thread::sleep(time::Duration::from_secs(1));
                let block_field = rpc.eth_block_number().await?;
                let current_block = match block_field.result {
                    Some(b) => {
                        let block_trim = b.trim_start_matches("0x");
                        let current_b = u128::from_str_radix(block_trim, 16)?;
                        println!("Sucess, current block is: {}", &current_b);
                        current_b
                    }
                    None => {
                        println!("Fail");
                        0
                    }
                };

                let block_num = block.number.trim_start_matches("0x");
                let result = u128::from_str_radix(block_num, 16)?;

                println!("Block num of that transaction: {}", &result);

                println!("Current block number {}", &current_block);
                block_confirmation = current_block - result;

                println!("Block confirmation (curr - block num): {}", block_confirmation);
            }
            None => {
                println!("Faild");
            }
        }
        if block_confirmation >= 2 {
            println!("Successfully confirmed block, total confirmed block: {}", block_confirmation);
            let connection = &mut establish_connection();

            let tx = insert_tx(
                connection,
                Some(to_address),
                Some(from_address),
                Some(tx_hash),
                Some(1)
            );
            println!("\nSaved draft with id {}", tx.id);
        } else {
            let connection = &mut establish_connection();

            let tx = insert_tx(
                connection,
                Some(to_address),
                Some(from_address),
                Some(tx_hash),
                Some(0)
            );
            println!("\nSaved draft with id {}", tx.id);
            println!("Failed, confirmed block must greater than 2");
        }
    } else if h == "0x" {
        println!("Hash not found");
    }

    
    Ok(())
}
