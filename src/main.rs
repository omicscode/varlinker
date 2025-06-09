mod args;
mod store;
mod variantlinker;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;

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
        Commands::CUIGENERATE {
            medgenhpo,
            medgen_omim,
            medgenmapping,
            medgenpubmed,
        } => {
            let command = cuiparallel(medgenhpo, medgen_omim, medgenmapping, medgenpubmed).unwrap();
            println!("The command has been completed:{:?}", command);
        }
    }
}
