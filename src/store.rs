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

pub struct GENCODE {
    pub chrom: String,
    pub typeannotate: String,
    pub start: usize,
    pub stop: usize,
    pub genename: String,
}

pub struct OUTPUT {
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refnuc: String,
    pub altnuc: String,
    pub typeannotate: String,
    pub genename: String,
}
