extern crate rqtl;

#[cfg(test)]
mod tests {
    #[test]
    fn parses_control() {
        let bxd_json_str = include_str!("bxd.json");
        let parsed: rqtl::Control = serde_json::from_str(bxd_json_str).unwrap();
        println!("{}", bxd_json_str);
        println!("{:?}", parsed);
        assert_eq!(1, 1);
    }

    #[test]
    fn parses_geno() {
        let bxd_csv_path = "./tests/bxd_geno.csv";

        let geno = rqtl::Geno::parse_transposed_geno(bxd_csv_path).unwrap();

        assert_eq!(198, geno.ids.len());
        assert_eq!(7320, geno.genos.len());
    }

}
