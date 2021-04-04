mod keyinfo;
mod describe;

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
        }
    };
}
