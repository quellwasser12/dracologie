use serde::Deserialize;
use reqwest::Error;

const BITCOIN_API_URL:&str = "https://rest.bitcoin.com/v2";


#[derive(Deserialize, Debug)]
pub struct ScriptSig {
    asm:String,
    hex:String
}

#[derive(Deserialize, Debug)]
pub struct TxIn {
    txid:String,
    vout:u16,
    #[serde(rename = "scriptSig")]
    script_sig:ScriptSig
}


#[derive(Deserialize, Debug)]
pub struct ScriptPubKey {
    asm:String,
    pub hex:String,
     #[serde(rename = "type")]
    _type:String,
    #[serde(rename = "reqSigs")]
    req_sigs:Option<u16>,
    addresses:Option<Vec<String>>
}


#[derive(Deserialize, Debug)]
pub struct TxOut {
    value:f32,
    n:u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey
}


#[derive(Deserialize, Debug)]
pub struct Transaction {
    txid:String,
    hash:String,
    version:u8,
    size:u32,
    locktime:u32,
    vin:Vec<TxIn>,
    pub vout:Vec<TxOut>,
    blockhash:String,
    confirmations:u32,
    pub time:u32,
    blocktime:u32
}

pub async fn get_transaction(txn_hash:&str) -> Result<Transaction, Error> {
    let request_url = format!("{base_url}/rawtransactions/getRawTransaction/{txid}?verbose=true",
                              base_url = BITCOIN_API_URL,
                              txid = txn_hash);
    let response = reqwest::blocking::get(&request_url)?.json();

    if let Err(e) = response {
        Err(e)
    } else {
        let txn:Transaction = response?;
        Ok(txn)
    }
}
