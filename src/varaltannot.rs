use crate::store::VCF;
use crate::store::{GENCODE, OUTPUT};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-6-9
*/

pub async fn varaltanno(pathfile: &str, variant: &str) -> Result<String, Box<dyn Error>> {
    let _ = Command::new("wegt").
        arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
        .output()
        .expect("command failed");
    let _ = Command::new("gunzip")
        .arg("gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
        .output()
        .expect("command failed");
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let gtfresults: Vec<GENCODE> =
        gtfread("gencode.v48.chr_patch_hapl_scaff.annotation.gtf").unwrap();
    let mut vcstring_file: Vec<VCF> = Vec::new();
    for i in fileread.lines() {
        let linevcf = i.expect("file not present");
        let linevec: Vec<String> = linevcf
            .split("\t")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        vcstring_file.push(VCF {
            chrom: linevec[0].to_string(),
            pos: linevec[1].parse::<usize>().unwrap(),
            id: linevec[2].to_string(),
            refnuc: linevec[3].to_string(),
            altnuc: linevec[4].to_string(),
            qual: linevec[5].to_string(),
        });
    }

    let mut output: Vec<OUTPUT> = Vec::new();
    for i in vcstring_file.iter() {
        for j in gtfresults.iter() {
            if i.pos > j.start && i.pos <= j.stop && j.typeannotate == variant {
                output.push(OUTPUT {
                    chrom: i.chrom.clone(),
                    pos: i.pos.clone().to_string(),
                    id: i.id.clone(),
                    refnuc: i.id.clone(),
                    altnuc: i.altnuc.clone(),
                    typeannotate: j.typeannotate.clone(),
                    genename: j.genename.clone(),
                });
            }
        }
    }

    let mut mutwrite = File::create("annotationfile.txt").expect("file not present");
    writeln!(
        mutwrite,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}",
        "chrom", "pos", "id", "refnuc", "altnuc", "typeannotate", "genename"
    )
    .expect("file not found");
    for i in output.iter() {
        writeln!(
            mutwrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.chrom, i.pos, i.id, i.refnuc, i.altnuc, i.typeannotate, i.genename
        )
        .expect("line not found");
    }
    Ok("The regions have been annotated".to_string())
}

pub fn gtfread(gtffile: &str) -> Result<Vec<GENCODE>, Box<dyn Error>> {
    let fileopen = File::open(gtffile).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut gtf_vector: Vec<GENCODE> = Vec::new();
    for i in fileread.lines() {
        let linegtf = i.expect("line not found");
        let linevec: Vec<String> = linegtf
            .split("\t")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let linecollect: String = linevec[9].split(";").collect::<Vec<_>>()[2]
            .replace(" ", "-")
            .split("-")
            .collect::<Vec<_>>()[1]
            .to_string();
        gtf_vector.push(GENCODE {
            chrom: linevec[0].clone(),
            typeannotate: linevec[2].clone(),
            start: linevec[3].parse::<usize>().unwrap(),
            stop: linevec[4].parse::<usize>().unwrap(),
            genename: linecollect,
        })
    }
    Ok(gtf_vector)
}
