extern crate hex;

use std::str::FromStr;
use std::fmt;

// LE, Cf. https://github.com/bitcoincashorg/bitcoincash.org/blob/master/etc/protocols.csv
const LOKAD_ID:u32 = 0xd101d400;
const OP_RETURN_CODE:u8 = 0x6a;


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
    input_index: u32,
    output_index: u32,
    hex: bool
}

impl fmt::Display for HatchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.hex {
            write!(f, "{:02x}04{:02x}01{:02x}04{}04{}08{:016x}20{}",
                   OP_RETURN_CODE, // 1 byte
                   LOKAD_ID, // 4 bytes
                   0xd1, // 1 byte
                   hex::encode(self.input_index.to_le_bytes()), // 4 bytes
                   hex::encode(self.output_index.to_le_bytes()),  // 4 bytes
                   self.cost, // 8 bytes
                   self.hashdragon) // 32 bytes (20 in hdex)
        } else {
            write!(f, "OP_RETURN 0x{:02x} 0x{:02x} {} {} {:016x} {}",
                   LOKAD_ID,
                   0xd1,
                   hex::encode(self.input_index.to_le_bytes()),
                   hex::encode(self.output_index.to_le_bytes()),
                   self.cost,
                   self.hashdragon)
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
