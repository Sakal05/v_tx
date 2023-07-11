pub fn query_rpc(network_name: String) -> anyhow::Result<String> {
    let rpc_url: Option<String> = match network_name.to_lowercase().as_str() {
        "polygon" => Some("https://polygon-testnet-rpc.allthatnode.com:8545".to_owned()),
        "avalanche" => Some("https://ava-testnet.public.blastapi.io/ext/bc/C/rpc".to_owned()),
        "goerli" => Some("https://ethereum-goerli-rpc.allthatnode.com".to_owned()),
        _ => None,
    };
    //https://api-testnet.snowtrace.io/
    //https://api-testnet.polygonscan.com/
    match rpc_url {
        Some(url) => Ok(url),
        None => Err(anyhow::anyhow!("Invalid network name")),
    }
}