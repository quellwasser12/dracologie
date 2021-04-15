use serde::Deserialize;
use reqwest::Error;
use std::collections::HashMap;

const BITCOIN_API_URL:&str = "https://rest.bitcoin.com/v2";


#[derive(Deserialize, Debug)]
struct ScriptSig {
    asm:String,
    hex:String
}

#[derive(Deserialize, Debug)]
struct TxIn {
    txid:String,
    vout:u16,
    #[serde(rename = "scriptSig")]
    script_sig:ScriptSig
}


#[derive(Deserialize, Debug)]
struct ScriptPubKey {
    asm:String,
    hex:String,
     #[serde(rename = "type")]
    _type:String,
    #[serde(rename = "reqSigs")]
    req_sigs:Option<u16>,
    addresses:Option<Vec<String>>
}


#[derive(Deserialize, Debug)]
struct TxOut {
    value:f32,
    n:u32,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: ScriptPubKey
}


#[derive(Deserialize, Debug)]
struct Transaction {
    txid:String,
    hash:String,
    version:u8,
    size:u32,
    locktime:u32,
    vin:Vec<TxIn>,
    vout:Vec<TxOut>,
    blockhash:String,
    confirmations:u32,
    time:u32,
    blocktime:u32
}

// /rawtransactions/getRawTransaction/:txid?verbose=true


pub async fn get_transaction(txn_hash:&str) -> Result<(), Error> {
    println!("Get Transaction!");
    let request_url = format!("{base_url}/rawtransactions/getRawTransaction/{txid}?verbose=true",
                              base_url = BITCOIN_API_URL,
                              txid = txn_hash);
    let response = reqwest::blocking::get(&request_url)?.json();
        println!("{:?}", response);

    if let Err(e) = response {
        println!("{}", e);
    } else {

        let txn:Transaction = response?;
//    let transaction = response.json().await?;
        println!("{:?}", txn);
    }
    Ok(())
}
