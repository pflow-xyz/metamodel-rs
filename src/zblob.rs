use serde::Serialize;

use crate::oid::Oid;
use crate::compression::{encode_zip, unzip_encoded};
use crate::petri_net::PetriNet;

#[derive(Debug, Clone, Serialize)]
pub struct Zblob {
    pub id: i64,
    pub ipfs_cid: String,
    pub base64_zipped: String,
    pub title: String,
    pub description: String,
    pub keywords: String,
    pub referrer: String,
    pub created_at: String,
}

const EMPTY_NET: &str = "UEsDBAoAAAAAAER3WVjjbbhPbAAAAGwAAAAKAAAAbW9kZWwuanNvbnsKICAibW9kZWxUeXBlIjogInBldHJpTmV0IiwKICAidmVyc2lvbiI6ICJ2MCIsCiAgInBsYWNlcyI6IHsKICB9LAogICJ0cmFuc2l0aW9ucyI6IHsKICB9LAogICJhcmNzIjogWwogIF0KfVBLAQIUAAoAAAAAAER3WVjjbbhPbAAAAGwAAAAKAAAAAAAAAAAAAAAAAAAAAABtb2RlbC5qc29uUEsFBgAAAAABAAEAOAAAAJQAAAAAAA==";

impl Default for Zblob {
    fn default() -> Self {
        Self {
            id: 0,
            ipfs_cid: Oid::new(EMPTY_NET.as_bytes()).unwrap().to_string(),
            base64_zipped: EMPTY_NET.to_string(),
            title: "default".to_string(),
            description: "".to_string(),
            keywords: "new".to_string(),
            referrer: "".to_string(),
            created_at: "".to_string(),
        }
    }
}

impl Zblob {
    pub fn from_string(encoded_zip: Option<&str>) -> Self {
        let mut zblob = Zblob::default();
        if encoded_zip.is_some() {
            zblob.base64_zipped = encoded_zip.unwrap().to_string();
            zblob.ipfs_cid = Oid::new(encoded_zip.unwrap().as_bytes()).unwrap().to_string();
        }
        zblob
    }
    pub fn from_net(net: &PetriNet) -> Self {
        let data = encode_zip(&serde_json::to_string(net).unwrap(), "model.json");
        return Self::from_string(Some(&data));
    }

    pub fn to_net(&self) -> PetriNet {
        let decoded = unzip_encoded(&self.base64_zipped, "model.json").unwrap();
        return serde_json::from_str(&decoded).unwrap();
    }
}