#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct VCF {
   pub  chrom: String,
   pub  pos: usize,
   pub  id: String,
   pub refnuc: String,
   pub altnuc: String,
   pub qual: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct GENCODE {
    pub
}
