use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CrossInfo {
    pub file: String,
    #[serde(flatten)]
    pub data: HashMap<String, i32>,
}

#[derive(Deserialize, Debug)]
pub struct Control {
    pub description: String,
    pub crosstype: String,
    pub geno: String,
    pub pheno: String,
    pub phenocovar: String,
    pub gmap: String,
    pub pmap: String,
    pub sep: String,
    #[serde(rename(deserialize = "na.strings"))]
    pub na_strings: Vec<String>,
    #[serde(rename(deserialize = "comment.char"))]
    pub comment_char: String,
    pub alleles: Vec<char>,
    pub x_chr: String,
    pub genotypes: HashMap<String, i32>,
    pub geno_transposed: bool,
    pub cross_info: CrossInfo,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn whoa() {}
}
