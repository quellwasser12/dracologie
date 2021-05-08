use crate::create_event;
use create_event::{WanderEvent, RescueEvent};
use futures::executor::block_on;
use bitcoin_hashes::hex::FromHex;
use bitcoincash::consensus::{deserialize, serialize};
use bitcoincash::{Transaction, TxIn, TxOut, Script, Address, OutPoint};
use crate::util::{from_slice_to_four_u8, from_cashaddr_to_legacy};
use bitcoincash::hashes::core::str::FromStr;

const LOKAD_ID:u32 = 0xd101d400;

/// Create the transaction hash for the corresponding event.
/// TODO Cleanup code, and implement other events.
pub fn create(
    event: create_event::Event,
    hashdragon: String,
    destination_address:String,
    change_address:String,
    _cost:u64,
    txn_ref: String,
    _hex:bool,
    coin_txn_ref:String
) {

    // Recipient address - CashAddr needs to be converted to legacy as bitcoincash lib
    //  does not support CashAddr.
    let addr = from_cashaddr_to_legacy(destination_address.as_str());
    let legacy_addr = Address::from_str(addr.as_str()).unwrap();


    // Change address
    let change_addr = from_cashaddr_to_legacy(change_address.as_str());
    let legacy_change_addr = Address::from_str(change_addr.as_str()).unwrap();

    let output:Result<String, String> = match event {
        create_event::Event::Wander => create_wander_txn(hashdragon, txn_ref, coin_txn_ref, legacy_addr, legacy_change_addr),
        create_event::Event::Rescue => create_rescue_txn(hashdragon, txn_ref, coin_txn_ref, legacy_addr, legacy_change_addr),
        _ => Err(format!("Unsupported Event: {:?}", event))
    };
    match output {
        Ok(out) => print!("{}", out),
        Err(e) => eprint!("{}", e)
    }
}

fn create_wander_txn(
    hashdragon: String,
    txn_ref: String,
    coin_txn_ref: String,
    legacy_addr: Address,
    legacy_change_addr: Address
) -> Result<String, String> {
    let original_txn = block_on::<_>(crate::bch_api::get_transaction(txn_ref.as_str()));
    let my_coin = block_on::<_>(crate::bch_api::get_transaction(coin_txn_ref.as_str())).unwrap();

    let output = match original_txn {
        Ok(txn) => {
            let hex: String = txn.hex;
            let tx_bytes = Vec::from_hex(&hex).unwrap();
            let tx: Transaction = deserialize(&tx_bytes).unwrap();
            let mut output_index: u32 = 1;

            let my_coin_tx_bytes = Vec::from_hex(&my_coin.hex).unwrap();
            let my_coin_tx: Transaction = deserialize(&my_coin_tx_bytes).unwrap();

            let hashdragon_script = &tx.output[0].script_pubkey;
            if hashdragon_script.is_op_return() {
                let instr = hashdragon_script.to_bytes();
                assert_eq!(instr[0], 0x6a);
                assert_eq!(u32::from_be_bytes(from_slice_to_four_u8(&instr[2..6])), LOKAD_ID);

                let cmd = instr[5];

                // Breeding is different
                if cmd != 0xd4 {
                    output_index = u32::from_le_bytes(from_slice_to_four_u8(&instr[14..18]));
                }

                let wander_event: String = WanderEvent {
                    hashdragon,
                    input_index: 0,
                    output_index: 1,
                    txn_ref,
                    hex: true
                }.to_string();

                let owner_vin = TxIn {
                    previous_output: OutPoint {
                        txid: tx.txid(),
                        vout: output_index
                    },
                    script_sig: Default::default(),
                    sequence: 0xFFFFFFFF,
                    witness: vec![]
                };

                let coin_vin = TxIn {
                    previous_output: OutPoint {
                        txid: my_coin_tx.txid(),
                        vout: 0
                    },
                    script_sig: Default::default(),
                    sequence: 0xFFFFFFFF,
                    witness: vec![]
                };

                // Create OP_RETURN vout
                let script_bytes = hex::decode(wander_event).unwrap();
                let hashdragon_vout = TxOut {
                    value: 0,
                    script_pubkey: Script::from(
                        script_bytes
                    )
                };
                // Create new owner vout
                let new_owner_vout = TxOut {
                    value: 2000,
                    script_pubkey: legacy_addr.script_pubkey()
                };
                // // Create change vout
                let change_vout = TxOut {
                    value: 4217270, // FIXME hardcoded value
                    script_pubkey: legacy_change_addr.script_pubkey()
                };
                //
                let tx_wander = Transaction {
                    version: 1,
                    lock_time: 0,
                    input: vec![owner_vin, coin_vin],
                    output: vec![hashdragon_vout, new_owner_vout, change_vout]
                };

                Ok(hex::encode(serialize(&tx_wander)))
            } else {
                Err("spec requires hashdragon OP_RETURN to be at vout 0.".to_owned())
            }
        },
        Err(e) => Err(format!("Error retrieving original txn: {}", e))
    };

    return output;
}

fn create_rescue_txn(
    hashdragon: String,
    txn_ref: String,
    coin_txn_ref: String,
    legacy_addr: Address,
    legacy_change_addr: Address
) -> Result<String, String> {
    let original_txn = block_on::<_>(crate::bch_api::get_transaction(txn_ref.as_str()));
    let my_coin = block_on::<_>(crate::bch_api::get_transaction(coin_txn_ref.as_str())).unwrap();

    let output = match original_txn {
        Ok(txn) => {
            let hex: String = txn.hex;
            let tx_bytes = Vec::from_hex(&hex).unwrap();
            let tx: Transaction = deserialize(&tx_bytes).unwrap();

            let my_coin_tx_bytes = Vec::from_hex(&my_coin.hex).unwrap();
            let my_coin_tx: Transaction = deserialize(&my_coin_tx_bytes).unwrap();

            let hashdragon_script = &tx.output[0].script_pubkey;
            if hashdragon_script.is_op_return() {
                let instr = hashdragon_script.to_bytes();
                assert_eq!(instr[0], 0x6a);
                assert_eq!(u32::from_be_bytes(from_slice_to_four_u8(&instr[2..6])), LOKAD_ID);

                let rescue_event: String = RescueEvent {
                    hashdragon,
                    input_index: 0,
                    output_index: 1,
                    txn_ref,
                    hex: true
                }.to_string();

                let coin_vin = TxIn {
                    previous_output: OutPoint {
                        txid: my_coin_tx.txid(),
                        vout: 0
                    },
                    script_sig: Default::default(),
                    sequence: 0xFFFFFFFF,
                    witness: vec![]
                };

                // Create OP_RETURN vout
                let script_bytes = hex::decode(rescue_event).unwrap();
                let hashdragon_vout = TxOut {
                    value: 0,
                    script_pubkey: Script::from(
                        script_bytes
                    )
                };
                // Create new owner vout
                let new_owner_vout = TxOut {
                    value: 2000,
                    script_pubkey: legacy_addr.script_pubkey()
                };

                // Create change vout
                let change_vout = TxOut {
                    value: 1997600, // FIXME hardcoded value: should val in (vin 0 - 2000 - fee)
                    script_pubkey: legacy_change_addr.script_pubkey()
                };
                //
                let tx_rescue = Transaction {
                    version: 1,
                    lock_time: 0,
                    input: vec![coin_vin],
                    output: vec![hashdragon_vout, new_owner_vout, change_vout]
                };

                Ok(hex::encode(serialize(&tx_rescue)))
            } else {
                Err("spec requires hashdragon OP_RETURN to be at vout 0.".to_owned())
            }
        },
        Err(e) => Err(format!("Error retrieving original txn: {}", e))
    };

    return output;
}
