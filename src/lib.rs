extern crate ndarray;

use ndarray::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
pub struct CrossInfo {
    pub file: String,
    #[serde(flatten)]
    pub data: BTreeMap<String, i32>,
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
        codes: BTreeMap<String, String>,
    },
    FromFile {
        file: String,
        #[serde(flatten)]
        codes: BTreeMap<String, String>,
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
    pub genotypes: BTreeMap<String, u8>,
    pub geno_transposed: Option<bool>,
    pub founder_geno_transposed: Option<bool>,
    pub cross_info: CrossInfo,
    pub sex: Option<Sex>,
}

impl Control {
    pub fn parse_geno(&self, geno: &str) -> u8 {
        self.genotypes.get(geno).copied().unwrap_or(0)
    }

    pub fn parse_f32(&self, data: &str) -> f32 {
        self.na_strings
            .iter()
            .find(|&s| s == data)
            .map_or_else(|| data.parse::<f32>().unwrap_or(0.0), |_| 0.0)
    }

    pub fn parse_cross_info(&self, x: &str) -> Option<i32> {
        self.cross_info.data.get(x).copied()
    }
}

#[derive(Debug)]
pub struct Geno {
    pub ids: Vec<String>,
    pub genos: BTreeMap<String, Vec<String>>,
}

impl Geno {
    pub fn parse_transposed_geno(path: &str) -> Result<Geno, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let ids: Vec<String> = {
            let headers = rdr.headers()?;
            headers.into_iter().skip(1).map(String::from).collect()
        };

        let mut genos = BTreeMap::new();

        rdr.records().for_each(|g| {
            let geno = g.unwrap();
            let k = geno.get(0).unwrap().to_string();
            let v = geno.into_iter().skip(1).map(String::from).collect();
            genos.insert(k, v);
        });

        Ok(Geno { ids, genos })
    }
}

#[derive(Debug)]
pub struct Chromosome {
    pub markers: Vec<(String, f32)>,
}

impl Chromosome {
    pub fn new() -> Chromosome {
        Chromosome { markers: vec![] }
    }
}

#[derive(Debug)]
pub struct Marker {
    pub name: String,
    pub pos: f32,
}

/// Used to represent both gmap and pmap data
#[derive(Debug)]
pub struct MapData {
    pub chromosomes: Vec<(String, Array1<Marker>)>,
}

fn get_chr_vec<'a>(v: &'a mut Vec<(String, Vec<Marker>)>, chr: &str) -> &'a mut Vec<Marker> {
    if let None = v.iter().find(|(c, _)| c == chr) {
        v.push((chr.to_string(), vec![]));
    }
    let (_, m) = v.iter_mut().find(|(c, _)| c == chr).unwrap();
    m
}

#[derive(Deserialize)]
struct MapRow<'a> {
    marker: &'a str,
    chr: &'a str,
    pos: f32,
}

impl MapData {
    pub fn new() -> MapData {
        MapData {
            chromosomes: vec![],
        }
    }

    pub fn get_chr(&self, chr: &str) -> Option<&[Marker]> {
        self.chromosomes
            .iter()
            .find(|&(c, _)| c == chr)
            .and_then(|(_, m)| m.as_slice())
    }

    /// Parse a dataset provided as an iterator over the lines of the
    /// dataset, parsed into a tuple, into one Array1<Marker> per
    /// chromosome, each stored as an element in a Vec
    pub fn read_csv(path: &str) -> Result<MapData, Box<dyn Error>> {
        let mut chromosomes_vec = vec![];

        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let hdrs = rdr.headers().ok().cloned();

        rdr.records().for_each(|r| {
            let row = r.unwrap();
            let maprow: MapRow = row.deserialize(hdrs.as_ref()).unwrap();

            let chr_vec = get_chr_vec(&mut chromosomes_vec, maprow.chr);
            chr_vec.push(Marker {
                name: maprow.marker.to_string(),
                pos: maprow.pos,
            })
        });

        let chromosomes = chromosomes_vec
            .into_iter()
            .map(|(c, m)| (c, Array::from_iter(m.into_iter())))
            .collect();

        Ok(MapData { chromosomes })
    }
}

#[derive(Debug, PartialEq)]
pub struct Dataset<A> {
    pub first_entry: String,
    pub ids: Vec<String>,
    pub row_ids: Vec<String>,
    pub data: Array2<A>,
}

impl<T> Dataset<T> {
    pub fn read_csv<F>(parser: F, path: &str) -> Result<Dataset<T>, Box<dyn Error>>
    where
        F: Fn(&str) -> Option<T>,
    {
        let mut rdr = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_path(path)?;

        let first_entry;

        let ids: Vec<String> = {
            let headers = rdr.headers()?;
            first_entry = headers.get(0).unwrap().to_string();

            headers.into_iter().skip(1).map(String::from).collect()
        };

        let mut row_ids: Vec<String> = vec![];
        let mut data_vec: Vec<T> = vec![];

        rdr.records().for_each(|g| {
            let geno = g.unwrap();
            let k = geno.get(0).unwrap().to_string();

            row_ids.push(k);

            let mut v: Vec<_> = geno
                .into_iter()
                .skip(1)
                .map(|x| parser(x).expect("Error parsing dataset"))
                .collect();

            data_vec.append(&mut v);
        });

        let width = ids.len();
        let height = row_ids.len();
        let data = Array::from_shape_vec((width, height), data_vec)?;

        Ok(Dataset {
            first_entry,
            ids,
            row_ids,
            data,
        })
    }

    pub fn transpose(self) -> Self {
        Dataset {
            data: self.data.reversed_axes(),
            ..self
        }
    }
}

impl Dataset<u8> {
    pub fn read_geno_csv(ctrl: &Control, path: &str) -> Result<Dataset<u8>, Box<dyn Error>> {
        Dataset::read_csv(|g| Some(ctrl.parse_geno(g)), path)
    }
}
