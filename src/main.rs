mod args;
mod store;
mod variantlinker;
mod varaltannot;
mod varrefannot;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use crate::variantlinker::varlinker;
use crate::varrefannot::varrefanno;
use crate::varaltannot::varaltanno;


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
            let command = varlinker(vcfile).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VARIANTALTANNO { vcffile, altallel } => {
            let command = varaltanno(vcffile, altallel).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VARIANTREFANNO { vcffile, refallele } => {
            let command = varrefanno(vcffile, refallele).unwrap();
            println!("The command has been completed:{:?}", command);
        }
    }
}
