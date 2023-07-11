use crate::query::query_rpc;
use web3_rpc::web3::Web3;

pub async fn send_raw_tx(network: String, hash: String) -> anyhow::Result<String> {
    let rpc_url = match query_rpc(network) {
        Ok(res) => {
            // println!("Successfully obtained RPC URL: {}", res);
            res
        }
        Err(e) => {
            println!("Error finding network: {}", e);
            return Err(anyhow::anyhow!("Invalid network name"))
        }
    };

    let rpc = Web3::new(rpc_url.to_string());

    /* Send Raw Transaction */
    let raw_tx_hash = format!("0x{}", hash);

    let send_raw = rpc.eth_send_raw_transaction(raw_tx_hash.as_str()).await?;

    let tx_hash = match send_raw.result {
        Some(res) => {
            println!("Result: {:?}", res);
            res
        }
        None => {
            println!("Send Failed");
            "0x".to_string()
        }
    };

    println!("Tx hash: {}", &tx_hash);

    Ok(tx_hash)
}
