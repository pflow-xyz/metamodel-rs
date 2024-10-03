use crate::dsl::{ArcParams, Builder, Dsl};
use crate::zblob::Zblob;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use std::collections::HashMap;

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
    pub fn from_json_value(contents: Value) -> Result<Self, Error> {
        let mut petri_net: PetriNet = serde_json::from_value(contents)?;
        petri_net.populate_arc_attributes();
        Ok(petri_net)
    }

    /// Creates a new `PetriNet` object from the given JSON string.
    pub fn from_json_str(contents: &str) -> Result<Self, Error> {
        let mut petri_net: PetriNet = serde_json::from_str(contents)?;
        petri_net.populate_arc_attributes();
        Ok(petri_net)
    }

    /// Converts the `PetriNet` to a JSON value.
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

    /// Creates a new `PetriNet` object from the given state diagram string.
    ///
    /// # Panics
    ///
    /// Panics if the diagram is invalid
    pub fn from_state_diagram(contents: String) -> Self {
        let contents = contents.replace('\n', "").replace(" ", "");
        let mut net = PetriNet::new();
        net.model_type = "workflow".to_string();
        let mut x = 20;
        let y = 200;
        let grid = 80;

        let lines: Vec<&str> = contents.split(';').map(str::trim).collect();
        assert!(!lines.is_empty(), "Contents cannot be empty");

        for line in lines {
            let action = line.to_string();
            if action.is_empty() {
                continue;
            }
            let parts: Vec<&str> = action.split("-->").map(str::trim).collect();
            if parts.len() != 2 {
                continue;
            }

            let input = parts[0];
            let output = parts[1];

            if !net.places.contains_key(input) {
                x += grid;
                let place_index: i32 = net.places.len().try_into().expect("place index overflow");
                net.add_place(input, place_index, None, None, x, y);
            }

            if !net.transitions.contains_key(&action) {
                x += grid;
                net.add_transition(&action, "default", x, y);
            }

            if !net.places.contains_key(output) {
                x += grid;
                let place_index: i32 = net.places.len().try_into().expect("place index overflow");
                net.add_place(output, place_index, None, None, x, y);
            }

            net.add_arc(ArcParams {
                source: input,
                target: &action,
                weight: Some(1),
                consume: Some(true),
                produce: Some(false),
                inhibit: None,
                read: None,
            });

            net.add_arc(ArcParams {
                source: &action,
                target: output,
                weight: Some(1),
                consume: Some(false),
                produce: Some(true),
                inhibit: None,
                read: None,
            });
        }

        net
    }
    /// Creates a new `PetriNet` object from the given diagram string.
    ///
    /// # Panics
    ///
    /// Panics if the diagram is invalid
    pub fn from_diagram(contents: String) -> Self {
        let contents = contents.replace('\n', "");
        let mut net = PetriNet::new();
        let mut x = 20;
        let y = 200;
        let grid = 80;

        let lines: Vec<&str> = contents.split(';').map(str::trim).collect();
        assert!(!lines.is_empty(), "Contents cannot be empty");

        // Parse the first line to set the model type
        let first_line = lines[0];
        assert!(
            first_line.starts_with("ModelType::"),
            "First line must specify the model type in the format ModelType::[type]"
        );

        net.model_type = match first_line
            .replace("ModelType::", "")
            .to_lowercase()
            .as_str()
        {
            "petrinet" => "petriNet".to_string(),
            "workflow" => "workflow".to_string(),
            "elementary" => "elementary".to_string(),
            _ => panic!("Invalid ModelType: must be one of petrinet, workflow, or elementary"),
        };

        for line in &lines[1..] {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split("-->").map(str::trim).collect();
            if parts.len() != 2 {
                continue;
            }
            let first_param_is_state = parts[0].chars().next().expect("first char").is_uppercase();
            let second_param_is_state = parts[1].chars().next().expect("first char").is_uppercase();

            let state = if first_param_is_state {
                parts[0]
            } else {
                assert!(
                    second_param_is_state,
                    "Second param must be uppercased state"
                );
                parts[1]
            };

            let action = if first_param_is_state {
                parts[1]
            } else {
                assert!(
                    second_param_is_state,
                    "Second param must be uppercase state"
                );
                parts[0]
            };

            if !net.places.contains_key(state) {
                x += grid;
                let place_index: i32 = net.places.len().try_into().expect("place index overflow");
                net.add_place(state, place_index, None, None, x, y);
            }

            if !net.transitions.contains_key(action) {
                x += grid;
                net.add_transition(action, "default", x, y);
            }

            net.add_arc(ArcParams {
                source: if first_param_is_state { state } else { action },
                target: if first_param_is_state { action } else { state },
                weight: Some(1),
                consume: Some(first_param_is_state),
                produce: Some(second_param_is_state),
                inhibit: None, // FIXME: not currently supported
                read: None,    // FIXME not currently supported
            });
        }

        net
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
    pub offset: i32,
    pub x: i32,
    pub y: i32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            role: Option::from("default".to_string()),
            offset: 0,
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
    ///
    /// # Panics
    ///
    /// Panics if the transition index overflows.
    pub fn add_transition(&mut self, label: &str, role: &str, x: i32, y: i32) {
        let offset = i32::try_from(self.transitions.len()).expect("transition index overflow");
        self.transitions.insert(
            label.to_string(),
            Transition {
                role: Option::from(role.to_string()),
                offset,
                x,
                y,
            },
        );
    }

    /// Adds an arc to the petri-net.
    pub fn add_arc(&mut self, params: ArcParams<'_>) {
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
    use super::*;

    pub const DINING_PHILOSOPHERS: &str = r#"
    {
        "modelType": "petriNet",
        "version": "v0",
        "places": {
            "right2": { "offset": 0, "x": 810, "y": 149 },
            "left2": { "offset": 1, "x": 942, "y": 153 },
            "right3": { "offset": 2, "x": 1182, "y": 218 },
            "left3": { "offset": 3, "x": 1260, "y": 339 },
            "right4": { "offset": 4, "x": 1169, "y": 744 },
            "left4": { "offset": 5, "x": 1082, "y": 843 },
            "right5": { "offset": 6, "x": 630, "y": 856 },
            "left5": { "offset": 7, "x": 531, "y": 728 },
            "right1": { "offset": 8, "x": 441, "y": 359 },
            "left1": { "offset": 9, "x": 501, "y": 244 },
            "chopstick1": { "offset": 10, "initial": 1, "x": 811, "y": 426 },
            "chopstick2": { "offset": 11, "initial": 1, "x": 931, "y": 434 },
            "chopstick3": { "offset": 12, "initial": 1, "x": 969, "y": 545 },
            "chopstick4": { "offset": 13, "initial": 1, "x": 863, "y": 614 },
            "chopstick5": { "offset": 14, "initial": 1, "x": 774, "y": 536 }
        },
        "transitions": {
            "eat1": { "offset": 0, "x": 610, "y": 370 },
            "think1": { "offset": 1, "x": 372, "y": 247 },
            "eat2": { "offset": 2, "x": 874, "y": 281 },
            "think2": { "offset": 3, "x": 876, "y": 42 },
            "eat3": { "offset": 4, "x": 1115, "y": 348 },
            "think3": { "offset": 5, "x": 1309, "y": 215 },
            "eat4": { "offset": 6, "x": 1034, "y": 691 },
            "think4": { "offset": 7, "x": 1227, "y": 896 },
            "eat5": { "offset": 8, "x": 673, "y": 688 },
            "think5": { "offset": 9, "x": 483, "y": 887 }
        },
        "arcs": [
            { "source": "chopstick1", "target": "eat1" },
            { "source": "chopstick5", "target": "eat1" },
            { "source": "eat1", "target": "left1" },
            { "source": "eat1", "target": "right1" },
            { "source": "eat2", "target": "right2" },
            { "source": "eat2", "target": "left2" },
            { "source": "chopstick1", "target": "eat2" },
            { "source": "chopstick2", "target": "eat2" },
            { "source": "chopstick2", "target": "eat3" },
            { "source": "chopstick3", "target": "eat3" },
            { "source": "eat3", "target": "right3" },
            { "source": "eat3", "target": "left3" },
            { "source": "chopstick3", "target": "eat4" },
            { "source": "chopstick4", "target": "eat4" },
            { "source": "eat4", "target": "left4" },
            { "source": "eat4", "target": "right4" },
            { "source": "think4", "target": "chopstick4" },
            { "source": "think4", "target": "chopstick3" },
            { "source": "right4", "target": "think4" },
            { "source": "left4", "target": "think4" },
            { "source": "chopstick5", "target": "eat5" },
            { "source": "chopstick4", "target": "eat5" },
            { "source": "eat5", "target": "left5" },
            { "source": "eat5", "target": "right5" },
            { "source": "think5", "target": "chopstick5" },
            { "source": "think5", "target": "chopstick4" },
            { "source": "left5", "target": "think5" },
            { "source": "right5", "target": "think5" },
            { "source": "left1", "target": "think1" },
            { "source": "right1", "target": "think1" },
            { "source": "think2", "target": "chopstick1" },
            { "source": "think2", "target": "chopstick2" },
            { "source": "think1", "target": "chopstick1" },
            { "source": "think1", "target": "chopstick5" },
            { "source": "right3", "target": "think3" },
            { "source": "left3", "target": "think3" },
            { "source": "think3", "target": "chopstick2" },
            { "source": "think3", "target": "chopstick3" },
            { "source": "right2", "target": "think2" },
            { "source": "left2", "target": "think2" }
        ]
    }"#;

    #[test]
    fn test_importing_json() {
        let petri_net =
            PetriNet::from_json_str(DINING_PHILOSOPHERS).expect("Failed to create PetriNet");
        assert_eq!(petri_net.places.len(), 15);
        assert_eq!(petri_net.transitions.len(), 10);
        assert_eq!(petri_net.arcs.len(), 40);
    }

    #[test]
    fn test_exporting_json() {
        let petri_net =
            PetriNet::from_json_str(DINING_PHILOSOPHERS).expect("Failed to create PetriNet");
        let json = petri_net
            .to_json_str()
            .expect("Failed to convert PetriNet to JSON");
        let net = PetriNet::from_json_str(&json).expect("Failed to create PetriNet from JSON");
        assert_eq!(net.places.len(), 15);
    }

    #[test]
    fn test_zblob() {
        let petri_net =
            PetriNet::from_json_str(DINING_PHILOSOPHERS).expect("Failed to create PetriNet");
        let zblob = petri_net.to_zblob();
        let net = zblob.to_net();
        assert_eq!(net.places.len(), 15);
        assert_eq!(
            zblob.ipfs_cid,
            "zb2rhZTUivNkdVe6qCEQ3oFe4xEbhSbVhfRj1kdZhKrTcw2Nk"
        );
    }

    #[test]
    fn test_arrow_diagram() {
        let contents = r"ModelType::PetriNet;
            Water --> boil_water;
            CoffeeBeans --> grind_beans;
            BoiledWater --> brew_coffee;
            GroundCoffee --> brew_coffee;
            Filter --> brew_coffee;
            CoffeeInPot --> pour_coffee;
            Cup --> pour_coffee;
        ";

        let net = PetriNet::from_diagram(contents.to_string());
        let zblob = net.to_zblob();
        println!("https://pflow.dev/?z={}", zblob.base64_zipped);
        let json_out = net.to_json().expect("Failed to convert to JSON");
        let pretty_json =
            serde_json::to_string_pretty(&json_out).expect("failed to convert to pretty json");
        print!("{pretty_json}");
    }

    #[test]
    fn test_state_machine_diagram() {
        let contents = r"
            Crash --> [*];
            Moving --> Crash;
            Moving --> Still;
            Still --> Moving;
            Still --> [*];
            [*] --> Still;
        ";

        let net = PetriNet::from_state_diagram(contents.to_string());
        let zblob = net.to_zblob();
        println!("{:?}", net.places);
        println!("https://pflow.dev/?z={}", zblob.base64_zipped);
    }
}
