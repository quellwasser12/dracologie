extern crate hex;

use crate::util;

use std::str::FromStr;
use std::fmt;
use bch_api::{Transaction, TxOut};
use bitcoincash::blockdata::script::Script;
use futures::executor::block_on;

use crate::bch_api;

// LE, Cf. https://github.com/bitcoincashorg/bitcoincash.org/blob/master/etc/protocols.csv
const LOKAD_ID:u32 = 0xd101d400;
const OP_RETURN_CODE:u8 = 0x6a;

// After 684000 switch from LE to BE
const BE_TIME:u32 = 1618794000;


/// Different types of events
#[derive(Debug, PartialEq)]
pub enum Event {
    Seeding,
    Dragonseed,
    Hatch,
    Wander,
    Hibernate,
    Breed,
    Trade,
    Fight
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seeding" => Ok(Event::Seeding),
            "dragonseed" => Ok(Event::Dragonseed),
            "hatch" => Ok(Event::Hatch),
            "wander" => Ok(Event::Wander),
            "hibernate" => Ok(Event::Hibernate),
            _ => Err(format!("Invalid event: {}", s))
        }
    }
}


struct DragonseedEvent {
    cost: u64,
    hashdragon: String,
    input_index: u32,
    output_index: u32,
    hex: bool
}

impl fmt::Display for DragonseedEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}08{}08{}10{:016x}20{}",
                   OP_RETURN_CODE,
                   LOKAD_ID,
                   0xd0,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.cost,
                   self.hashdragon)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {:016x} {}",
                   LOKAD_ID,
                   0xd0,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.cost,
                   self.hashdragon)
        }
    }
}

struct HatchEvent {
    cost: u64,
    hashdragon: String,
    txn_ref: String,
    input_index: u32,
    output_index: u32,
    hex: bool // Output script as hex or not
}


#[derive(Debug)]
struct Param {
    cost: Option<u64>,
    hashdragon: Option<String>,
    input_index: Option<u32>,
    output_index: Option<u32>
}


//  Example of a return
// 6a04d101d40001d10401000000040100000008000000000000000020d41a8517be90657bd88502def8b6c9ffca37f6c8a2d631d02b535d47965c479c
fn parse_hashdragon_script(script:&String, be:bool) -> Param {

    let parts:Vec<u8> = hex::decode(script).unwrap();
    let parsed_script = Script::from(parts);

    println!("{:?}", parsed_script);

    let parsed_script_bytes = parsed_script.into_bytes();

    // OP_RETURN
    assert!(parsed_script_bytes[0] == 0x6a);
    // OP_PUSHBYTES_4
    assert!(parsed_script_bytes[1] == 0x04);
    // LOKAD ID
    assert!(parsed_script_bytes[2..6] == LOKAD_ID.to_be_bytes());
    // OP_PUSHBYTES_1
    assert!(parsed_script_bytes[6] == 0x01);
    let cmd = parsed_script_bytes[7];

    match cmd {
        0xd1 => {
            assert!(parsed_script_bytes[8] == 0x04);

            let mut input_index:[u8;4] = util::from_slice_to_four_u8(&parsed_script_bytes[9..13]);
            if !be { input_index.reverse(); }
            assert!(parsed_script_bytes[13] == 0x04);
            let mut output_index:[u8;4] = util::from_slice_to_four_u8(&parsed_script_bytes[14..18]);
            if !be { output_index.reverse(); }
            assert!(parsed_script_bytes[18] == 0x08);
            let mut cost:[u8;8] = util::from_slice_to_eight_u8(&parsed_script_bytes[19..27]);
            if !be { cost.reverse(); }
            assert!(parsed_script_bytes[27] == 0x20);
            let hashdragon:[u8;32] = util::from_slice_to_thirtytwo_u8(&parsed_script_bytes[28..60]);

            return Param {
                cost: Some(util::as_u64_be(&cost)),
                hashdragon: Some(hex::encode(hashdragon)),
                input_index: Some(util::as_u32_be(&input_index)),
                output_index: Some(util::as_u32_be(&output_index))
            };

        },
        _ => panic!("Unknown command")
    }

}


// Verifies the reference transaction used as input for the dragon event.
fn verify_hashdragon_txn(txn: &Transaction) -> Param {
    let vout_vec:&Vec<TxOut> = &txn.vout;

    // First vout must be hashdragon OP_RETURN script
    let vout0:&TxOut = &vout_vec[0];
    let script_pub_key: &bch_api::ScriptPubKey = &vout0.script_pub_key;
    println!("ScriptPubKey: {:?}", script_pub_key);

    return parse_hashdragon_script(&script_pub_key.hex, txn.time > BE_TIME);
}


// Create the Hatch event script.
fn output_hatch_script(event: &HatchEvent, f: &mut fmt::Formatter, param: &Param) -> fmt::Result {
    if event.hex {
        write!(f, "{:02x}04{:02x}01{:02x}04{}04{}08{:016x}20{}",
               OP_RETURN_CODE, // 1 byte
               LOKAD_ID, // 4 bytes
               0xd1, // 1 byte
               hex::encode(event.input_index.to_be_bytes()), // 4 bytes
               hex::encode(event.output_index.to_be_bytes()),  // 4 bytes
               event.cost, // 8 bytes
               event.hashdragon) // 32 bytes (20 in hdex)
    } else {
        write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {:016x} {}",
               LOKAD_ID,
               0xd1,
               hex::encode(event.input_index.to_be_bytes()),
               hex::encode(event.output_index.to_be_bytes()),
               event.cost,
               event.hashdragon)
    }
}

impl fmt::Display for HatchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let original_txn = block_on::<_>(bch_api::get_transaction(self.txn_ref.as_str()));
        println!("Transaction: {:?}", original_txn);
        match original_txn {
            Err(e) => panic!("Error retrieving original txn: {}", e),
            Ok(txn) => {
                let param = verify_hashdragon_txn(&txn);
                println!("{:?}", param);

                let hd = param.hashdragon.clone();
                match hd {
                    Some(d) => assert!(d == self.hashdragon), // Checks that hashdragon is ref txn is the same one
                    None => panic!("No hashdragon found")     // FIXME Proper error handling rather than panic!
                };
                return output_hatch_script(self, f, &param);
            }
        }
    }
}

struct WanderEvent {
    hashdragon: String,
    input_index: u32,
    output_index: u32,
    txn_ref: String,
    hex: bool
}


impl fmt::Display for WanderEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}04{}04{}20{}",
                   OP_RETURN_CODE,
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.hashdragon)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {}",
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.hashdragon)
        }
    }
}


struct HibernateEvent {
    hashdragon: String,
    input_index: u32,
    output_index: u32,
    hex: bool
}


impl fmt::Display for HibernateEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}04{}04{}20{}",
                   OP_RETURN_CODE,
                   LOKAD_ID,
                   0xd3,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.hashdragon)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {}",
                   LOKAD_ID,
                   0xd3,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.hashdragon)
        }
    }
}



pub fn create(event: Event, hashdragon: String, cost:u64, txn_ref: String, hex:bool) {
    let output = match event {
        Event::Dragonseed => { DragonseedEvent { cost: cost,
                                                 hashdragon: hashdragon,
                                                 input_index: 1 as u32,
                                                 output_index: 1 as u32,
                                                 hex: hex}.to_string() },
        Event::Hatch => { HatchEvent { cost: cost,
                                       hashdragon: hashdragon,
                                       txn_ref: txn_ref,
                                       input_index: 1 as u32,
                                       output_index: 1 as u32,
                                       hex: hex }.to_string() },
        Event::Wander => { WanderEvent { hashdragon: hashdragon,
                                         input_index: 1 as u32,
                                         output_index: 1 as u32,
                                         txn_ref: txn_ref,
                                         hex: hex }.to_string() },
        Event::Hibernate => { HibernateEvent { hashdragon: hashdragon,
                                               input_index: 1 as u32,
                                               output_index: 1 as u32,
                                               hex: hex }.to_string() },
        _ => panic!("Unsupported Event: {:?}", event)
    };

    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hatch() {
        let hatch_event = HatchEvent {
            cost: 0,
            hashdragon: "d4b74244fde6c5bdad53ce7606fa1e7d8657d9a3debbdcb0132f8e9580fa5d76".to_string(),
            input_index: 1,
            output_index: 1,
            hex: true
        };
        assert_eq!(hatch_event.to_string(),
                   "6a04d101d40001d10401000000040100000008000000000000000020d4b74244fde6c5bdad53ce7606fa1e7d8657d9a3debbdcb0132f8e9580fa5d76")
    }
}
