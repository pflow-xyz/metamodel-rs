use crate::dsl::Dsl;
use crate::petri_net::PetriNet;
use crate::vasm::StateMachine;

pub struct Model {
    pub net: PetriNet,
    pub declaration: Vec<fn(&mut dyn Dsl)>,
    pub vm: Box<StateMachine>,
}

impl Model {
    pub fn new(func: fn(&mut dyn Dsl)) -> Self {
        let mut net = PetriNet::new();
        let vm = Box::new(net.declare(func).as_vasm());
        Self {
            net,
            declaration: Vec::from([func]),
            vm,
        }
    }

    /// Use pflow DSL to declare a function that defines the model
    ///
    /// This is the same logic as the `new` function, but it allows
    /// for chaining
    pub fn declare(&mut self, func: fn(&mut dyn Dsl)) -> &mut Model {
        self.declaration.push(func);
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
        println!("https://pflow.dev/?z={}", net.to_zblob().base64_zipped);
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self {
            net,
            declaration: Vec::new(),
            vm,
        }
    }

    /// Parse a JSON value into a PetriNet
    ///
    /// # Panics
    ///
    /// Panics if the JSON value cannot be parsed
    pub fn from_json_value(value: serde_json::Value) -> Self {
        let mut net = PetriNet::from_json_value(value).expect("Failed to parse JSON");
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self {
            net,
            declaration: Vec::new(),
            vm,
        }
    }

    /// Parse a JSON string into a PetriNet
    ///
    /// # Panics
    ///
    /// Panics if the JSON string cannot be parsed
    pub fn from_json_str(value: &str) -> Self {
        let mut net = PetriNet::from_json_str(value).expect("Failed to parse JSON");
        let vm = Box::new(net.declare(|_| {}).as_vasm());
        Self {
            net,
            declaration: Vec::new(),
            vm,
        }
    }
}