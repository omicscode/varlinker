use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::process::Command;
use crate::store::VCF;


/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-6-9
*/

pub fn varaltanno(pathfile : &str, variant: &str) -> Result<String, Box<dyn Error>>{

    let _ = Command::new("wegt").
        arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
        .output()
        .expect("command failed");
    let _ = Command::new("gunzip").arg("gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
        .output()
        .expect("command failed");
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let gtfopen = File::open("gencode.v48.chr_patch_hapl_scaff.annotation.gtf")
        .expect("file not present");
    let gtfread = BufReader::new(gtfopen);
    let exonvector:Vec<String> = Vec::new();
    let gene: Vec<String> = Vec::new();
    let mut vcstring_file: Vec<VCF> = Vec::new();
    for i in fileread.lines(){
        let linevcf = i.expect("file not present");
        let linevec:Vec<String> = linevcf.split("\t").
            map(|x|x.to_string())
            .collect::<Vec<_>>();
        vcstring_file.push(VCF{
                  chrom: linevec[0].to_string(),
                  pos: linevec[1].parse::<usize>().unwrap(),
                  id: linevec[2].to_string(),
                  refnuc: linevec[3].to_string(),
                  altnuc: linevec[4].to_string(),
                  qual: linevec[5].to_string(),
        });
    }




    Ok("The regions have been annotated".to_string())
}
