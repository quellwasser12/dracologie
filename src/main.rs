#![feature(exclusive_range_pattern)]
mod keyinfo;
mod util;
mod describe;
mod bch_api;
mod create_event;
mod create_txn;

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
#[structopt(name = "dracologie", about = "Useful commands for hashdragons")]
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

        /// Flag to indicate whether to display script as hex.
        #[structopt(name = "hex", long = "hex")]
        hex: bool,
        #[structopt(name = "cost", long, required_if("event", "dragonseed"))]
        cost: Option<u64>,

        /// Hash of the last hashdragon transaction
        #[structopt(name="txn-ref", long)]
        txn_ref: String
    },
    #[structopt(name = "create-txn")]
    CreateTxn {
        event: create_event::Event,
        hashdragon: String,
        #[structopt(name="dest-address", long)]
        destination_address:String,
        #[structopt(name="change-address", long)]
        change_address:String,

        /// Flag to indicate whether to display script as hex.
        #[structopt(name = "hex", long = "hex")]
        hex: bool,
        #[structopt(name = "cost", long, required_if("event", "dragonseed"))]
        cost: Option<u64>,

        /// Hash of the last hashdragon transaction
        #[structopt(name="txn-ref", long)]
        txn_ref: String,
        #[structopt(name="coin-txn-ref", long)]
        coin_txn_ref: String,
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
            create_event::create(event, hashdragon, cost_value, txn_ref, hex)
        },
        Command::CreateTxn { event, hashdragon, destination_address, change_address, cost, txn_ref, hex, coin_txn_ref } => {
            let cost_value = match cost {
                Some(c) => c,
                None => 0
            };
            create_txn::create(event, hashdragon, destination_address, change_address, cost_value, txn_ref, hex, coin_txn_ref)
        }
    };
}
