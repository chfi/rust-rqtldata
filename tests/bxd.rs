extern crate rqtl;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn parses_control() {
        let bxd_json_str = include_str!("bxd.json");
        let parsed: rqtl::Control = serde_json::from_str(bxd_json_str).unwrap();

        assert_eq!(parsed.crosstype, String::from("risib"));

        let genos: BTreeMap<String, u8> = vec![("B".to_string(), 1), ("D".to_string(), 2)]
            .into_iter()
            .collect();
        assert_eq!(parsed.genotypes, genos);

        let cross_info = rqtl::CrossInfo {
            file: String::from("bxd_crossinfo.csv"),
            data: vec![("BxD".to_string(), 0)].into_iter().collect(),
        };
        assert_eq!(parsed.cross_info, cross_info);

        println!("{:?}", parsed);
    }

    #[test]
    fn parses_geno() {
        let bxd_csv_path = "./tests/bxd_geno.csv";

        let geno = rqtl::Geno::parse_transposed_geno(bxd_csv_path).unwrap();

        assert_eq!(198, geno.ids.len());
        assert_eq!(7320, geno.genos.len());
    }

    #[test]
    fn parses_ndarray() {
        let bxd_json_str = include_str!("bxd.json");
        let control: rqtl::Control = serde_json::from_str(bxd_json_str).unwrap();

        let bxd_csv_path = "./tests/bxd_geno.csv";
        let geno = rqtl::Dataset::read_geno_csv(&control, bxd_csv_path).unwrap();

        // println!("{:?}", geno);
        println!("shape: {:?}", geno.data.shape());

        assert_eq!(geno.data.shape(), [198, 7320]);
    }

    #[test]
    fn parses_gmap() {
        let iter = vec![
            ("1a", "1", 1.0),
            ("1b", "1", 1.4),
            ("1c", "1", 1.7),
            ("1d", "1", 1.7),
            ("1e", "1", 1.9),
            ("2a", "2", 1.0),
            ("2b", "2", 1.5),
            ("2c", "2", 2.7),
        ]
        .into_iter();

        let gmap = rqtl::Gmap::parse_by_chr(iter);

        println!("gmap: {:?}", gmap);

        assert_eq!(1, 1);
    }

}
