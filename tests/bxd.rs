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

        println!("shape: {:?}", geno.data.shape());

        assert_eq!(geno.data.shape(), [198, 7320]);
    }

    #[test]
    fn parses_gmap() {
        let bxd_gmap_csv = "./tests/bxd_gmap.csv";
        let gmap = rqtl::Gmap::parse_csv(bxd_gmap_csv).expect("Could not parse gmap csv");

        assert_eq!(gmap.chromosomes.len(), 20);
        let (chr1, ms) = gmap.chromosomes.first().unwrap();
        assert_eq!(chr1, "1");
        assert_eq!(ms.len(), 636);
    }

}
