use crate::dsl::Dsl;
use crate::petri_net::PetriNet;
use crate::vasm::StateMachine;

#[derive(Debug)]
pub struct Model {
    pub net: PetriNet,
    pub vm: Box<StateMachine>,
}

impl Model {
    pub fn new(func: fn(&mut dyn Dsl)) -> Self {
        let mut net = PetriNet::new();
        let vm = Box::new(net.declare(func).as_vasm());
        Self { net, vm }
    }

    /// Use pflow DSL to declare a function that defines the model
    ///
    /// This is the same logic as the `new` function, but it allows
    /// for chaining
    pub fn declare(&mut self, func: fn(&mut dyn Dsl)) -> &mut Model {
        self.vm = Box::new(self.net.declare(func).as_vasm());
        self
    }

    ///  Parse a diagram into a PetriNet
    ///
    /// # Panics
    ///
    /// Panics if the diagram is not valid
    pub fn from_diagram(contents: String) -> Self {
        let mut net = if contents.contains("ModelType::") {
            PetriNet::from_diagram(contents)
        } else {
            PetriNet::from_state_diagram(contents)
        };
        // println!("https://pflow.dev/?z={}", net.to_zblob().base64_zipped);
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self { net, vm }
    }

    /// Parse a JSON value into a PetriNet
    ///
    /// # Panics
    ///
    /// Panics if the JSON value cannot be parsed
    pub fn from_json_value(value: serde_json::Value) -> Self {
        let mut net = PetriNet::from_json_value(value).expect("Failed to parse JSON");
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self { net, vm }
    }

    /// Parse a JSON string into a PetriNet
    ///
    /// # Panics
    ///
    /// Panics if the JSON string cannot be parsed
    pub fn from_json_str(value: &str) -> Self {
        let mut net = PetriNet::from_json_str(value).expect("Failed to parse JSON");
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self { net, vm }
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        let mut net = self.net.clone();
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self { net, vm }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model() {
        let m = Model::from_diagram(
            "Crash-->[*];Moving-->Crash;Moving-->Still;Still-->Moving;Still-->[*];[*]-->Still;"
                .to_string(),
        );
        assert_eq!(m.net.places.len(), 4);
        for place in m.net.places.iter() {
            println!("{place:?}");
        }
        for transition in m.net.transitions.iter() {
            println!("{transition:?}");
        }
    }
}
