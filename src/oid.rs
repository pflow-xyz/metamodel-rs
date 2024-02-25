use libipld::cid::Cid;
use libipld::multihash::{Code, MultihashDigest};

pub struct Oid {
    cid: Cid,
}

impl Oid {
    pub fn new(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let hash = MultihashDigest::digest(&Code::Sha2_256, bytes);
        let cid = Cid::new_v1(0x55, hash); // NOTE: always produces string starting with "zb2"
        Ok(Self { cid })
    }

    pub fn to_string(&self) -> String {
        return self.cid.to_string_of_base(multibase::Base::Base58Btc).unwrap();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.cid.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::fixtures::DINING_PHILOSOPHERS;

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestStruct {
        field1: u32,
        field2: String,
    }

    #[test]
    fn test_cid_generation() {
        let oid = Oid::new(DINING_PHILOSOPHERS.as_bytes()).expect("Failed to create Oid");
        let oid_string = oid.to_string();
        println!("oid_string: {:?}", oid_string);
        assert_eq!(oid_string, "zb2rhhAP4oqMEYFwLJ1UKgQrvBWsDkrvkY9Sn4HBVgfZ5ymNY");
    }
}
