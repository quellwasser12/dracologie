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
    output_index: u32
}

impl fmt::Display for DragonseedEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OP_RETURN 0xd0 {} {} {} {}", self.input_index, self.output_index, self.cost, self.hashdragon)
    }
}

pub fn create(event: Event, hashdragon: String, cost:u64) {

    let output = match event {
        Dragonseed => { DragonseedEvent { cost: cost,
                                          hashdragon: hashdragon,
                                          input_index: 1 as u32,
                                          output_index: 1 as u32 }.to_string() }
    };

    println!("{}", output);
}
