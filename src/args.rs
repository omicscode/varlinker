use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "varlinker",
    version = "1.0",
    about = "specific position annotator for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// annotate the specific coordinate
    VARIANTLINKER {
        /// variant VCF file
        vcfile: String,
    },
    /// extract the annotation of the specific ref allele
    VARIANTREFANNO {
        /// variant VCF file
        vcffile: String,
        /// ref allele
        refallele: String,
    },
    /// extract the annotation of the specific alt allele
    VARIANTALTANNO {
        /// variant VCF file
        vcffile: String,
        /// alt allele
        altallel: String,
    }
}
