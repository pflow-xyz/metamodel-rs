use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

use crate::dsl::{ArcParams, Builder, Dsl};
use crate::zblob::Zblob;

/// PetriNet stores petri-net elements used during the construction of a petri-net.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PetriNet {
    pub model_type: String,
    pub version: String,
    pub places: HashMap<String, Place>,
    pub transitions: HashMap<String, Transition>,
    pub arcs: Vec<Arrow>,
}

impl Default for PetriNet {
    fn default() -> Self {
        Self {
            model_type: "petriNet".to_string(),
            version: "v0".to_string(),
            places: HashMap::new(),
            transitions: HashMap::new(),
            arcs: Vec::new(),
        }
    }
}

impl PetriNet {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn declare(&mut self, func: fn(&mut dyn Dsl)) -> Builder<'_> {
        let mut flow_builder = Builder::new(self);
        func(&mut flow_builder);
        flow_builder
    }

    /// Creates a new `PetriNet` object from the given JSON value.
    pub fn from_json(contents: Value) -> Result<Self, Error> {
        let mut petri_net: PetriNet = serde_json::from_value(contents)?;
        petri_net.populate_arc_attributes();
        Ok(petri_net)
    }

    /// Creates a new `PetriNet` object from the given JSON string.
    pub fn from_json_str(contents: String) -> Result<Self, Error> {
        let mut petri_net: PetriNet = serde_json::from_str(&contents)?;
        petri_net.populate_arc_attributes();
        Ok(petri_net)
    }

    pub fn to_json(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }

    /// Converts the `PetriNet` to a canonical JSON string.
    pub fn to_json_str(&self) -> Result<String, cjson::Error> {
        self.to_json().map(|v| cjson::to_string(&v))?
    }

    /// Converts the `PetriNet` to a `Zblob` object.
    pub fn to_zblob(&self) -> Zblob {
        Zblob::from_net(self)
    }
}

/// Place is a struct that represents a place (cell in FLowDsl).
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Place {
    pub offset: i32,
    pub initial: Option<i32>,
    pub capacity: Option<i32>,
    pub x: i32,
    pub y: i32,
}

impl Default for Place {
    fn default() -> Self {
        Self {
            offset: 0,
            initial: Option::from(0),
            capacity: Option::from(0),
            x: 0,
            y: 0,
        }
    }
}

/// Transition is a struct that represents a transition (func in FlowDsl).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transition {
    pub role: Option<String>,
    pub x: i32,
    pub y: i32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            role: Option::from("default".to_string()),
            x: 0,
            y: 0,
        }
    }
}

/// Arrow is a struct that represents an arrow (arc in FlowDsl).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arrow {
    pub source: String,
    pub target: String,
    pub weight: Option<i32>,
    pub consume: Option<bool>,
    pub produce: Option<bool>,
    pub inhibit: Option<bool>,
    pub read: Option<bool>,
}

impl PetriNet {
    /// Populates the arc attributes of the petri-net by inferring the values of consume, produce, inhibit, and read.
    pub fn populate_arc_attributes(&mut self) {
        for arc in &mut self.arcs {
            if arc.consume.is_none() {
                // if arc source is a place consume is true
                arc.consume = Some(self.places.contains_key(&arc.source));
            }
            if arc.produce.is_none() {
                // if arc source is a transition produce is true
                arc.produce = Some(self.transitions.contains_key(&arc.source));
            }
            if arc.read.is_none() {
                // if source is a transition and inhibit is true read is true
                arc.read = Some(
                    self.transitions.contains_key(&arc.source) && arc.inhibit.unwrap_or(false),
                );
            }
        }
    }

    /// Adds a place to the petri-net.
    pub fn add_place(
        &mut self,
        label: &str,
        offset: i32,
        initial: Option<i32>,
        capacity: Option<i32>,
        x: i32,
        y: i32,
    ) {
        self.places.insert(
            label.to_string(),
            Place {
                offset,
                initial,
                capacity,
                x,
                y,
            },
        );
    }

    /// Adds a transition to the petri-net.
    pub fn add_transition(&mut self, label: &str, role: &str, x: i32, y: i32) {
        self.transitions.insert(
            label.to_string(),
            Transition {
                role: Option::from(role.to_string()),
                x,
                y,
            },
        );
    }

    /// Adds an arc to the petri-net.
    pub fn add_arc(
        &mut self,
        params: ArcParams<'_>,
    ) {
        self.arcs.push(Arrow {
            source: params.source.to_string(),
            target: params.target.to_string(),
            weight: params.weight,
            consume: params.consume,
            produce: params.produce,
            inhibit: params.inhibit,
            read: params.read,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::fixtures::DINING_PHILOSOPHERS;

    use super::*;

    #[test]
    fn test_importing_json() {
        let petri_net = PetriNet::from_json_str(DINING_PHILOSOPHERS.to_string())
            .expect("Failed to create PetriNet");
        assert_eq!(petri_net.places.len(), 15);
        assert_eq!(petri_net.transitions.len(), 10);
        assert_eq!(petri_net.arcs.len(), 40);
    }

    #[test]
    fn test_exporting_json() {
        let petri_net = PetriNet::from_json_str(DINING_PHILOSOPHERS.to_string()).expect("Failed to create PetriNet");
        let json = petri_net.to_json_str().expect("Failed to convert PetriNet to JSON");
        let net = PetriNet::from_json_str(json).expect("Failed to create PetriNet from JSON");
        assert_eq!(net.places.len(), 15);
    }

    #[test]
    fn test_zblob() {
        let petri_net = PetriNet::from_json_str(DINING_PHILOSOPHERS.to_string()).expect("Failed to create PetriNet");
        let zblob = petri_net.to_zblob();
        let net = zblob.to_net();
        assert_eq!(net.places.len(), 15);
        assert_eq!(
            zblob.ipfs_cid,
            "zb2rhbJgSpkiifamgPLnyfEDxRKRBjPru2ojyYSBMitPNjXTx"
        );
    }
}
