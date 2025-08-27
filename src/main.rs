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
use async_std::task;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-8
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("varLinker");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::VariantLINKER { vcfile } => {
            let command = task::block_on(varlinker(vcfile)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VariantTALTANNO { vcffile, altallel } => {
            let command = task::block_on(varaltanno(vcffile, altallel)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::VariantTREFANNO { vcffile, refallele } => {
            let command = task::block_on(varrefanno(vcffile, refallele)).unwrap();
            println!("The command has been completed:{:?}", command);
        }
    }
}
