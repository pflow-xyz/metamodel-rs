use libipld::cid::Cid;
use libipld::multihash::{Code, MultihashDigest};
use std::fmt;

/// `Oid` is a struct that represents an object identifier (OID) in the form of a `Cid` object from the `libipld` crate.
/// It provides methods to create a new `Oid` from a byte slice, and to convert the `Oid` to a string or a byte vector.
#[derive(Clone, Copy)]
pub struct Oid {
    cid: Cid,
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cid.to_string_of_base(multibase::Base::Base58Btc)
            .map_or(Err(fmt::Error), |s| write!(f, "{s}"))
    }
}

impl Oid {
    /// Creates a new `Oid` object from the given byte slice.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice that the `Oid` will be created from.
    ///
    /// # Returns
    ///
    /// * A `Result` which is `Ok` if the `Oid` could be created, or `Err` if there was an error.
    ///
    pub fn new(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let hash = MultihashDigest::digest(&Code::Sha2_256, bytes);
        let cid = Cid::new_v1(0x55, hash); // 0x55 is the code for raw codec
        Ok(Self { cid })
    }

    /// Converts the `Oid` to a byte vector.
    ///
    /// # Returns
    ///
    /// * A byte vector that represents the `Oid`.
    ///
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
        println!("oid_string: {oid_string}");
        assert_eq!(
            oid_string,
            "zb2rhhAP4oqMEYFwLJ1UKgQrvBWsDkrvkY9Sn4HBVgfZ5ymNY"
        );
    }
}
