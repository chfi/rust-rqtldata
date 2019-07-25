extern crate rqtl;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn parses_control() {
        let bxd_json_str = include_str!("bxd.json");
        let parsed: rqtl::Control = serde_json::from_str(bxd_json_str).unwrap();

        assert_eq!(parsed.crosstype, String::from("risib"));

        let genos: HashMap<String, i32> = vec![("B".to_string(), 1), ("D".to_string(), 2)]
            .into_iter()
            .collect();
        assert_eq!(parsed.genotypes, genos);

        let cross_info = rqtl::CrossInfo {
            file: String::from("bxd_crossinfo.csv"),
            data: vec![("BxD".to_string(), 0)].into_iter().collect(),
        };
        assert_eq!(parsed.cross_info, cross_info);
    }

    #[test]
    fn parses_geno() {
        let bxd_csv_path = "./tests/bxd_geno.csv";

        let geno = rqtl::Geno::parse_transposed_geno(bxd_csv_path).unwrap();

        assert_eq!(198, geno.ids.len());
        assert_eq!(7320, geno.genos.len());
    }

}
