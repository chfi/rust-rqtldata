use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Geno {
    pub ids: Vec<String>,
    pub genos: HashMap<String, Vec<String>>,
}

impl Geno {
    pub fn parse_transposed_geno(path: &str) -> Result<Geno, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let ids: Vec<String> = {
            let headers = rdr.headers()?;
            headers.into_iter().skip(1).map(String::from).collect()
        };

        let mut genos = HashMap::new();

        rdr.records().for_each(|g| {
            let geno = g.unwrap();
            let k = geno.get(0).unwrap().to_string();
            let v = geno.into_iter().skip(1).map(String::from).collect();
            genos.insert(k, v);
        });

        Ok(Geno { ids, genos })
    }

    /*
    fn parse_geno(path: String) -> Result<Geno, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let markers: Vec<String> = {
            let headers = rdr.headers()?;
            headers.into_iter().skip(1).map(String::from).collect()
        };

        Ok(Geno { ids, genos })
    }
    */
}
