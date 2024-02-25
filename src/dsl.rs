use crate::petri_net::PetriNet;
use crate::vasm::StateMachine;

pub trait FlowDsl {
    fn model_type(&mut self, model_type: &str);
    fn cell<'a>(&mut self, label: &'a str, initial: Option<i32>, capacity: Option<i32>, x: i32, y: i32) -> &'a str;
    fn func<'a>(&mut self, label: &'a str, role: &str, x: i32, y: i32) -> &'a str;
    fn arrow(&mut self, source: &str, target: &str, weight: i32);
    fn guard(&mut self, source: &str, target: &str, weight: i32);
}
pub struct Builder<'a> {
    pub net: &'a mut PetriNet,
}

impl<'a> Builder<'a> {
    pub fn new(net: &'a mut PetriNet) -> Self {
        Self {
            net
        }
    }

    pub fn as_vasm(&mut self) -> StateMachine {
        StateMachine::from_model(self.net)
    }
}

impl<'a> FlowDsl for Builder<'a> {
    fn model_type(&mut self, model_type: &str) {
        self.net.model_type = model_type.to_string();
    }

    fn cell<'b>(&mut self, label: &'b str, initial: Option<i32>, capacity: Option<i32>, x: i32, y: i32) -> &'b str {
        let offset = self.net.places.len() as i32;
        self.net.add_place(label, offset, initial, capacity, x, y);
        return label;
    }

    fn func<'b>(&mut self, label: &'b str, role: &str, x: i32, y: i32) -> &'b str {
        self.net.add_transition(label, role, x, y);
        return label;
    }

    fn arrow(&mut self, source: &str, target: &str, weight: i32) {
        //TxNode::new(&self.net, label.to_string(), role.to_string(), pos)
    }

    fn guard(&mut self, source: &str, target: &str, weight: i32) {
        //TxNode::new(&self.net, label.to_string(), role.to_string(), pos)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::vasm::declare;

    use super::*;

    fn model_test_code(p: &mut dyn FlowDsl) {
        p.model_type("petriNet");

        let r = "default";
        let foo = p.cell("foo", Option::from(1), None, 0, 0);
        let bar = p.func("bar", r, 0, 0);
        let baz = p.func("baz", r, 0, 0);

        p.arrow(foo, bar, 1);
        p.guard(foo, baz, 1);
    }

    #[test]
    fn test_loading_dsl() {
        let model = declare(model_test_code);
        let vm = model.deref();
        let state = vm.initial_vector();
        let res = vm.transform(state, "bar", 1);

        assert!(res.is_ok());
        // FIXME actually make state changes
        //assert_ne!(res.output, state);
        println!("{:?}", res);
    }
}
