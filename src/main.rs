mod args;
mod store;
mod varaltannot;
mod variantlinker;
mod varrefannot;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::varaltannot::varaltanno;
use crate::variantlinker::varlinker;
use crate::varrefannot::varrefanno;
use clap::Parser;
use async_std::task;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-8
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::VARIANTLINKER { vcfile } => {
            let command = task::block_on(varlinker(vcfile)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VARIANTALTANNO { vcffile, altallel } => {
            let command = task::block_on(varaltanno(vcffile, altallel)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VARIANTREFANNO { vcffile, refallele } => {
            let command = task::block_on(varrefanno(vcffile, refallele)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
    }
}
