use crate::dsl::FlowDsl;
use crate::petri_net::PetriNet;
use crate::vasm::Vasm;

pub struct Model {
    pub vm: Box<dyn Vasm>,
    pub net: PetriNet,
    pub declaration: fn(&mut dyn FlowDsl),
}

impl Model {
    pub fn new(func: fn(&mut dyn FlowDsl)) -> Self {
        let mut net = PetriNet::new();
        let vm = Box::new(net.declare(func).as_vasm());
        Self {
            vm,
            net,
            declaration: func
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model() {
        let model = Model::new(|p| {
            p.model_type("petriNet");
            p.cell("a", Option::from(1), None, 0, 0);
            p.func("f", "default", 1, 1);
        });
        assert_eq!(model.net.model_type, "petriNet");
        let zblob = model.net.to_zblob();
        assert_eq!(zblob.ipfs_cid, "zb2rhoPcxUWE6MeyZuz4sntoko8vVhLgZaXKmSnXgYis2Vy8c");
    }
}