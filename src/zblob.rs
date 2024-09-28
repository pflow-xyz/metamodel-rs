use serde::Serialize;

use crate::compression::{compress_brotli_encode, decompress_brotli_decode};
use crate::oid::Oid;
use crate::petri_net::PetriNet;

/// `Zblob` is a struct used to pack and unpack a zipped base64 encoded PetriNet into a sharable blob.
#[derive(Debug, Clone, Serialize)]
pub struct Zblob {
    /// The id of the zblob.
    pub id: i64,
    /// The IPFS CID of the zblob.
    pub ipfs_cid: String,
    /// The base64 zipped content of the zblob.
    pub base64_zipped: String,
    /// The title of the zblob.
    pub title: String,
    /// The description of the zblob.
    pub description: String,
    /// The keywords associated with the zblob.
    pub keywords: String,
    /// The referrer of the zblob.
    pub referrer: String,
    /// The creation time of the zblob.
    pub created_at: String,
}

const EMPTY_NET: &str = "UEsDBAoAAAAAAER3WVjjbbhPbAAAAGwAAAAKAAAAbW9kZWwuanNvbnsKICAibW9kZWxUeXBlIjogInBldHJpTmV0IiwKICAidmVyc2lvbiI6ICJ2MCIsCiAgInBsYWNlcyI6IHsKICB9LAogICJ0cmFuc2l0aW9ucyI6IHsKICB9LAogICJhcmNzIjogWwogIF0KfVBLAQIUAAoAAAAAAER3WVjjbbhPbAAAAGwAAAAKAAAAAAAAAAAAAAAAAAAAAABtb2RlbC5qc29uUEsFBgAAAAABAAEAOAAAAJQAAAAAAA==";

impl Default for Zblob {
    fn default() -> Self {
        Self {
            id: 0,
            ipfs_cid: Oid::new(EMPTY_NET.as_bytes()).expect("oid fault").to_string(),
            base64_zipped: EMPTY_NET.to_string(),
            title: "default".to_string(),
            description: String::new(),
            keywords: "new".to_string(),
            referrer: String::new(),
            created_at: String::new(),
        }
    }
}

// TODO: replace w/ error enum
const INVALID_ZIP: &str = "invalid zip";
const FAILED_TO_CONVERT: &str = "failed to convert to json";
const FAILED_TO_DECOMPRESS: &str = "failed to decompress";

impl Zblob {
    /// Creates a new `Zblob` from a base64 encoded string.
    ///
    /// # Arguments
    ///
    /// * `encoded_zip` - A base64 encoded string.
    ///
    /// # Panics
    ///
    /// This function will panic if the given string is not a valid base64 encoded string.
    pub fn from_string(encoded_zip: Option<&str>) -> Self {
        let mut zblob = Zblob::default();
        if encoded_zip.is_some() {
            zblob.base64_zipped = encoded_zip.expect(INVALID_ZIP).to_string();
            zblob.ipfs_cid = Oid::new(encoded_zip.expect(INVALID_ZIP).as_bytes())
                .expect(INVALID_ZIP)
                .to_string();
            zblob.keywords = String::new();
        }
        zblob
    }

    /// Creates a new `Zblob` from a `PetriNet`.
    ///
    /// # Panics
    ///
    /// This function will panic if the given `PetriNet` cannot be converted to a JSON string.
    pub fn from_net(net: &PetriNet) -> Self {
        let net_json = net.to_json_str().expect(FAILED_TO_CONVERT);
        let data = compress_brotli_encode(&net_json).expect(FAILED_TO_CONVERT);
        Self::from_string(Some(&data))
    }

    /// Converts the `Zblob` into a `PetriNet`.
    ///
    /// # Panics
    ///
    /// This function will panic if the base64 zipped content of the `Zblob` cannot be decompressed or if the decompressed data is not a valid JSON string.
    pub fn to_net(&self) -> PetriNet {
        let decoded = decompress_brotli_decode(&self.base64_zipped).expect(FAILED_TO_DECOMPRESS);
        serde_json::from_str(&decoded).expect(FAILED_TO_DECOMPRESS)
    }
}
