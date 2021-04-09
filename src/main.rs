#![feature(exclusive_range_pattern)]
mod keyinfo;
mod describe;
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
        cost: u64,
        #[structopt(name = "hex", long = "hex")]
        hex: bool // Flag to indicate whether to display
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
        Command::CreateEvent { event, hashdragon, cost, hex } => create_event::create(event, hashdragon, cost, hex)
    };
}
