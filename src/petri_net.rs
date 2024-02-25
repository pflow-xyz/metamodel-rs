use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::dsl::{FlowDsl, Builder};

pub fn read_petri_net_from_string(contents: String) -> Result<PetriNet, Error> {
    let mut petri_net: PetriNet = serde_json::from_str(&*contents)?;
    petri_net.populate_arc_attributes();
    Ok(petri_net)
}

pub fn read_petri_net_from_file(file_path: &str) -> Result<PetriNet, Error> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    read_petri_net_from_string(contents)
}

#[derive(Serialize, Deserialize, Debug)]
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

    pub(crate) fn declare(&mut self, func: fn(&mut dyn FlowDsl)) -> Builder {
        let mut flow_builder = Builder::new(self);
        func(&mut flow_builder);
        return flow_builder;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
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
                arc.read = Some(self.transitions.contains_key(&arc.source) && arc.inhibit.unwrap_or(false));
            }
        }
    }

    pub fn add_place(&mut self, label: &str, offset: i32, initial: Option<i32>, capacity: Option<i32>, x: i32, y: i32) {
        self.places.insert(label.to_string(), Place { offset, initial, capacity, x, y });
        return;
    }

    pub fn add_transition(&mut self, label: &str, role: &str, x: i32, y: i32) {
        self.transitions.insert(label.to_string(), Transition { role: Option::from(role.to_string()), x, y });
        return;
    }

    pub fn add_arc(&mut self, source: &str, target: &str, weight: Option<i32>, consume: Option<bool>, produce: Option<bool>, inhibit: Option<bool>, read: Option<bool>) {
        self.arcs.push(Arrow {
            source: source.to_string(),
            target: target.to_string(),
            weight,
            consume,
            produce,
            inhibit,
            read
        });
        return;
    }
}

#[cfg(test)]
mod tests {
    use crate::fixtures::DINING_PHILOSOPHERS;

    use super::*;

    #[test]
    fn test_importing_json() {
        let petri_net = read_petri_net_from_string(DINING_PHILOSOPHERS.to_string()).unwrap();
        assert_eq!(petri_net.places.len(), 15);
        assert_eq!(petri_net.transitions.len(), 10);
        assert_eq!(petri_net.arcs.len(), 40);
    }
}