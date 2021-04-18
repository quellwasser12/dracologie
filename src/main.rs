#![feature(exclusive_range_pattern)]
mod keyinfo;
mod util;
mod describe;
mod bch_api;
mod create_event;


use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {

    #[structopt(name = "keyinfo")]
    KeyInfo {
        key: String
    },
    #[structopt(name = "describe")]
    Describe {
        hash: String
    },
    #[structopt(name = "create-event")]
    CreateEvent {
        event: create_event::Event,
        hashdragon: String,
        #[structopt(name = "hex", long = "hex")]
        hex: bool, // Flag to indicate whether to display
        #[structopt(name = "cost", long, required_if("event", "dragonseed"))]
        cost: Option<u64>,
        #[structopt(name="txn-ref", long)]
        txn_ref: Option<String>
    }
}


fn main() {

    let opt = Opt::from_args();
    match opt.cmd {
        Command::KeyInfo { key } => keyinfo::keyinfo(key),
        Command::Describe { hash } => {
            let result = describe::describe(hash);
            match result {
                Ok(_) => println!(),
                Err(msg) => println!("Message: {}", msg)
            }
            return;
        },
        Command::CreateEvent { event, hashdragon, cost, txn_ref, hex } => {
            let cost_value = match cost {
                Some(c) => c,
                None => 0
            };
            let txn_ref_value = match txn_ref {
                Some(reference) => reference,
                None => "".to_string()
            };
            create_event::create(event, hashdragon, cost_value, txn_ref_value, hex)
        }
    };
}
