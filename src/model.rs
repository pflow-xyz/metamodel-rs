use crate::dsl::FlowDsl;
use crate::petri_net::PetriNet;
use crate::vasm::StateMachine;

pub struct Model {
    pub net: PetriNet,
    pub declaration: Vec<fn(&mut dyn FlowDsl)>,
    pub vm: Box<StateMachine>,
}

impl Model {
    pub fn new(func: fn(&mut dyn FlowDsl)) -> Self {
        let mut net = PetriNet::new();
        let vm = Box::new(net.declare(func).as_vasm());
        Self {
            net,
            declaration: Vec::from([func]),
            vm,
        }
    }

    pub fn declare(&mut self, func: fn(&mut dyn FlowDsl)) -> &mut Model {
        self.declaration.push(func);
        self.vm = Box::new(self.net.declare(func).as_vasm());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model() {
        let mut model = Model::new(|p| {
            p.model_type("petriNet");
            p.cell("b", Option::from(1), None, 0, 0);
            p.func("f", "default", 1, 1);
        });

        model.declare(|p| {
            p.cell("a", Option::from(1), None, 0, 0);
            p.func("g", "default", 1, 1);
            p.arrow("a", "f", 1);
        });

        assert_eq!(model.net.model_type, "petriNet");
        let zblob = model.net.to_zblob();
        assert_eq!(
            zblob.ipfs_cid,
            "zb2rhXz6Zi73pN9tyWzNGCLUCd9MLvAkupcBKXpCvrV87Rch4"
        );

        let r = model.vm.roles.get("default").unwrap();
        assert!(r, "expected role");
    }
}
