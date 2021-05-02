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
    Rescue,
    Hibernate,
    Breed,
    Trade,
    Fight
}


/// Converts a string into its corresponding enum.
impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seeding" => Ok(Event::Seeding),
            "dragonseed" => Ok(Event::Dragonseed),
            "hatch" => Ok(Event::Hatch),
            "wander" => Ok(Event::Wander),
            "rescue" => Ok(Event::Rescue),
            "hibernate" => Ok(Event::Hibernate),
            _ => Err(format!("Invalid event: {}", s))
        }
    }
}


/// For historical purposes, seeding has already occurred.
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
pub struct Param {
    action: Event,
    cost: Option<u64>,
    hashdragon: Option<String>,
    input_index: Option<u32>,
    output_index: Option<u32>,
    txn_ref: Option<String>
}


//  Example of a return
// 6a04d101d40001d10401000000040100000008000000000000000020d41a8517be90657bd88502def8b6c9ffca37f6c8a2d631d02b535d47965c479c
pub fn parse_hashdragon_script(script:&String, be:bool) -> Param {

    let parts:Vec<u8> = hex::decode(script).unwrap();
    let parsed_script = Script::from(parts);

    println!("{:?}", parsed_script);

    let parsed_script_bytes = parsed_script.into_bytes();

    // OP_RETURN
    assert_eq!(parsed_script_bytes[0], 0x6a);
    // OP_PUSHBYTES_4
    assert_eq!(parsed_script_bytes[1], 0x04);
    // LOKAD ID
    assert_eq!(parsed_script_bytes[2..6], LOKAD_ID.to_be_bytes());
    // OP_PUSHBYTES_1
    assert_eq!(parsed_script_bytes[6], 0x01);
    let cmd = parsed_script_bytes[7];

    assert_eq!(parsed_script_bytes[8], 0x04);
    let mut input_index:[u8;4] = util::from_slice_to_four_u8(&parsed_script_bytes[9..13]);
    if !be { input_index.reverse(); }
    assert_eq!(parsed_script_bytes[13], 0x04);
    let mut output_index:[u8;4] = util::from_slice_to_four_u8(&parsed_script_bytes[14..18]);
    if !be { output_index.reverse(); }

    match cmd {

        0xd1 => {
            assert_eq!(parsed_script_bytes[18], 0x08);
            let mut cost:[u8;8] = util::from_slice_to_eight_u8(&parsed_script_bytes[19..27]);
            if !be { cost.reverse(); }
            assert_eq!(parsed_script_bytes[27], 0x20);
            let hashdragon:[u8;32] = util::from_slice_to_thirtytwo_u8(&parsed_script_bytes[28..60]);

            return Param {
                action: Event::Hatch,
                cost: Some(util::as_u64_be(&cost)),
                hashdragon: Some(hex::encode(hashdragon)),
                input_index: Some(util::as_u32_be(&input_index)),
                output_index: Some(util::as_u32_be(&output_index)),
                txn_ref: None
            };
        },

        0xd2 => {
            if parsed_script_bytes.len() == 18 {
                return Param {
                    action: Event::Wander,
                    cost: None,
                    input_index: Some(util::as_u32_be(&input_index)),
                    output_index: Some(util::as_u32_be(&output_index)),
                    hashdragon: None,
                    txn_ref: None
                }
            } else {
                assert_eq!(parsed_script_bytes[18], 0x20);
                let txn_ref:[u8;32] = util::from_slice_to_thirtytwo_u8(&parsed_script_bytes[19..51]);
                return Param {
                    action: Event::Rescue,
                    cost: None,
                    input_index: Some(util::as_u32_be(&input_index)),
                    output_index: Some(util::as_u32_be(&output_index)),
                    hashdragon: None,
                    txn_ref: Some(hex::encode(txn_ref))
                }
            }
        },
        _ => panic!("Unknown command")
    }

}


/// Verifies the reference transaction used as input for the dragon event.
fn verify_hashdragon_txn(txn: &Transaction) -> Param {
    let vout_vec:&Vec<TxOut> = &txn.vout;

    // First vout must be hashdragon OP_RETURN script
    let vout0:&TxOut = &vout_vec[0];
    let script_pub_key: &bch_api::ScriptPubKey = &vout0.script_pub_key;
    println!("ScriptPubKey: {:?}", script_pub_key);

    return parse_hashdragon_script(&script_pub_key.hex, txn.time > BE_TIME);
}


/// Create the Hatch event script.
fn output_hatch_script(event: &HatchEvent, f: &mut fmt::Formatter, param: &Param) -> fmt::Result {

    if event.hex {
        write!(f, "{:02x}04{:02x}01{:02x}04{}04{}08{:016x}20{}",
               OP_RETURN_CODE, // 1 byte
               LOKAD_ID, // 4 bytes
               0xd1, // 1 byte
               hex::encode(event.input_index.to_be_bytes()), // 4 bytes
               hex::encode(event.output_index.to_be_bytes()),  // 4 bytes
               event.cost, // 8 bytes
               event.hashdragon) // 32 bytes (20 in hex)
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
                if let Some(d) = hd {
                    assert_eq!(d, self.hashdragon)
                } else {
                    panic!("No hashdragon found")
                };
                return output_hatch_script(self, f, &param);
            }
        }
    }
}

pub struct WanderEvent {
    pub hashdragon: String,
    pub input_index: u32,
    pub output_index: u32,
    pub txn_ref: String,
    pub hex: bool
}


impl fmt::Display for WanderEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}04{}04{}",
                   OP_RETURN_CODE,
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()))
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {}",
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()))
        }
    }
}


struct RescueEvent {
    hashdragon: String,
    input_index: u32,
    output_index: u32,
    txn_ref: String,
    hex: bool
}


impl fmt::Display for RescueEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}04{}04{}20{}",
                   OP_RETURN_CODE,
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()),
                   self.txn_ref)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {}",
                   LOKAD_ID,
                   0xd2,
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()),
                   self.txn_ref)
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
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()),
                   self.hashdragon)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {}",
                   LOKAD_ID,
                   0xd3,
                   hex::encode(self.input_index.to_be_bytes()),
                   hex::encode(self.output_index.to_be_bytes()),
                   self.hashdragon)
        }
    }
}


/// Create an event based on the options passed on the command line.
pub fn create(event: Event, hashdragon: String, cost:u64, txn_ref: String, hex:bool) {

    // TODO Get indices from txn_ref.
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
        Event::Rescue => { RescueEvent { hashdragon: hashdragon,
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
            hashdragon: "d41a8517be90657bd88502def8b6c9ffca37f6c8a2d631d02b535d47965c479c".to_string(),
            txn_ref: "fcf5a12c7a85271e8726f0bb53d683335fe84d9c683ec5d0f038114951a2863d".to_string(),
            input_index: 1,
            output_index: 1,
            hex: true
        };
        assert_eq!(hatch_event.to_string(),
                   "6a04d101d40001d10400000001040000000108000000000000000020d41a8517be90657bd88502def8b6c9ffca37f6c8a2d631d02b535d47965c479c")
    }


    #[test]
    fn test_parse_hashdragon_script_wander() {
        let parsed_script_val = parse_hashdragon_script(&"6a04d101d40001d204000000010400000001".to_string(), true);
        assert_eq!(parsed_script_val.action, Event::Wander);
        assert_eq!(parsed_script_val.cost, None);
        assert_eq!(parsed_script_val.input_index.unwrap(), 1);
        assert_eq!(parsed_script_val.output_index.unwrap(), 1);
    }

    #[test]
    fn test_parse_hashdragon_script_rescue() {
        let parsed_script_val = parse_hashdragon_script(&"6a04d101d40001d20400000001040000000120f3ced6df469e44ddc774af1df41c7a5e0381217b9e9462ca6f05781ed6dc3ba0".to_string(), true);
        assert_eq!(parsed_script_val.action, Event::Rescue);
        assert_eq!(parsed_script_val.cost, None);
        assert_eq!(parsed_script_val.input_index.unwrap(), 1);
        assert_eq!(parsed_script_val.output_index.unwrap(), 1);
        assert_eq!(parsed_script_val.txn_ref.unwrap(), "f3ced6df469e44ddc774af1df41c7a5e0381217b9e9462ca6f05781ed6dc3ba0");
    }
}
