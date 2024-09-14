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

    pub fn declare(&mut self, func: fn(&mut dyn Dsl)) -> &mut Model {
        self.declaration.push(func);
        self.vm = Box::new(self.net.declare(func).as_vasm());
        self
    }
}
