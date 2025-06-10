#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct VCF {
    pub chrom: String,
    pub pos: usize,
    pub id: String,
    pub refnuc: String,
    pub altnuc: String,
    pub qual: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenCodeGene {
    pub chrom: String,
    pub typeannotate: String,
    pub start: usize,
    pub stop: usize,
    pub geneid: String,
    pub genename: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenCodeExon {
    pub chrom: String,
    pub typeannotate: String,
    pub start: usize,
    pub stop: usize,
    pub geneid: String,
    pub genename: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenCodeTranscript {
    pub chrom: String,
    pub typeannotate: String,
    pub start: usize,
    pub stop: usize,
    pub geneid: String,
    pub genename: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OUTPUT {
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refnuc: String,
    pub altnuc: String,
    pub typeannotate: String,
    pub geneid: String,
    pub genename: String,
}
