#[macro_use(array)]
extern crate ndarray;

use ndarray::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
pub struct CrossInfo {
    pub file: String,
    #[serde(flatten)]
    pub data: HashMap<String, i32>,
}

// Might use this instead of String in the Files struct later
#[derive(Deserialize, Debug)]
pub enum File {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub geno: Option<String>,
    pub founder_geno: Option<String>,
    pub pheno: Option<String>,
    pub covar: Option<String>,
    pub phenocovar: Option<String>,
    pub gmap: Option<String>,
    pub pmap: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum Sex {
    Covar {
        covar: String,
        #[serde(flatten)]
        codes: HashMap<String, String>,
    },
    FromFile {
        file: String,
        #[serde(flatten)]
        codes: HashMap<String, String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Control {
    pub description: String,
    pub crosstype: String,
    #[serde(flatten)]
    pub files: Files,
    pub sep: char,
    #[serde(rename(deserialize = "na.strings"))]
    pub na_strings: Vec<String>,
    #[serde(rename(deserialize = "comment.char"))]
    pub comment_char: String,
    pub alleles: Vec<char>,
    pub x_chr: Option<String>,
    pub genotypes: HashMap<String, u8>,
    pub geno_transposed: Option<bool>,
    pub founder_geno_transposed: Option<bool>,
    pub cross_info: CrossInfo,
    pub sex: Option<Sex>,
}

impl Control {
    pub fn read_geno(&self, geno: &str) -> u8 {
        let g = geno.to_string();
        self.genotypes.get(&g).map(|x| x.clone()).unwrap_or(0)
    }
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

#[derive(Debug)]
pub struct Dataset<A> {
    pub first_entry: String,
    pub ids: Vec<String>,
    pub row_ids: Vec<String>,
    pub data: Array2<A>,
}

impl Dataset<u8> {
    pub fn read_geno_csv(ctrl: &Control, path: &str) -> Result<Dataset<u8>, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let mut first_entry;

        let ids: Vec<String> = {
            let headers = rdr.headers()?;
            first_entry = headers.get(0).unwrap().to_string();

            headers.into_iter().skip(1).map(String::from).collect()
        };

        let mut row_ids: Vec<String> = vec![];

        let mut data_vec: Vec<u8> = vec![];

        rdr.records().for_each(|g| {
            let geno = g.unwrap();
            let k = geno.get(0).unwrap().to_string();

            row_ids.push(k);

            let mut v: Vec<_> = geno
                .into_iter()
                .skip(1)
                .map(|x| ctrl.read_geno(x))
                .collect();

            data_vec.append(&mut v);
        });

        let width = ids.len();
        let height = row_ids.len();
        println!("w: {}", width);
        println!("h: {}", height);

        let data = Array::from_shape_vec((width, height), data_vec)?;

        Ok(Dataset {
            first_entry,
            ids,
            row_ids,
            data,
        })
    }
}
