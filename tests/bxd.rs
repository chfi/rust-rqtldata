extern crate rqtl;
use rqtl::Control;

use serde::Deserialize;
use serde_json;

#[cfg(test)]
mod tests {
    #[test]
    fn parses() {
        let bxd_json_str = include_str!("bxd.json");
        let parsed: rqtl::Control = serde_json::from_str(bxd_json_str).unwrap();
        println!("{}", bxd_json_str);
        println!("{:?}", parsed);
        assert_eq!(1, 1);
    }
}
