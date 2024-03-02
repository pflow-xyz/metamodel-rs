use crate::petri_net::PetriNet;
use crate::vasm::StateMachine;

/// `FlowDsl` is a trait that provides a domain-specific language (DSL) for defining Petri nets.
/// It provides methods for defining the model type, cells, functions, arrows, and guards.
///
/// # Methods
///
/// * `model_type` - Sets the model type of the Petri net.
/// * `cell` - Adds a cell (place) to the Petri net.
/// * `func` - Adds a function (transition) to the Petri net.
/// * `arrow` - Adds an arrow (arc) from a source to a target in the Petri net.
/// * `guard` - Adds a guard (inhibitor arc) from a source to a target in the Petri net.
///
/// # Example
///
/// ```
/// use metamodel::dsl::FlowDsl;
/// use metamodel::vasm::Vasm;
/// fn model_test_code(p: &mut dyn FlowDsl) {
///     p.model_type("petriNet");
///
///     let r = "default";
///     let foo = p.cell("foo", Option::from(1), None, 0, 0);
///     let bar = p.func("bar", r, 0, 0);
///     let baz = p.func("baz", r, 0, 0);
///
///     p.arrow(foo, bar, 1);
///     p.guard(foo, baz, 1);
/// }
///
/// let model = <dyn Vasm>::new(model_test_code);
/// ```
pub trait FlowDsl {
    /// Sets the model type of the Petri net.
    fn model_type(&mut self, model_type: &str);
    /// Adds a cell (place) to the Petri net.
    fn cell<'a>(
        &mut self,
        label: &'a str,
        initial: Option<i32>,
        capacity: Option<i32>,
        x: i32,
        y: i32,
    ) -> &'a str;
    /// Adds a function (transition) to the Petri net.
    fn func<'a>(&mut self, label: &'a str, role: &str, x: i32, y: i32) -> &'a str;
    /// Adds an arrow (arc) from a source to a target in the Petri net.
    fn arrow(&mut self, source: &str, target: &str, weight: i32);
    /// Adds a guard (inhibitor arc) from a source to a target in the Petri net.
    fn guard(&mut self, source: &str, target: &str, weight: i32);
}

/// `Builder` is a struct that implements the `FlowDsl` trait and is used to build a Petri net.
/// It contains a mutable reference to a `PetriNet` object which it modifies as methods are called on it.
///
/// # Methods
///
/// * `new` - Creates a new `Builder` object.
/// * `as_vasm` - Converts the `PetriNet` object into a `StateMachine` object.
///
pub struct Builder<'a> {
    pub net: &'a mut PetriNet,
}

impl<'a> Builder<'a> {
    /// Creates a new `Builder` object with the given `PetriNet` object.
    ///
    /// # Arguments
    ///
    /// * `net` - A mutable reference to a `PetriNet` object that the `Builder` will modify.
    ///
    /// # Returns
    ///
    /// * A new `Builder` object.
    ///
    pub fn new(net: &'a mut PetriNet) -> Self {
        Self { net }
    }

    /// Converts the `PetriNet` object into a `StateMachine` object.
    ///
    /// # Returns
    ///
    /// * A `StateMachine` object that represents the `PetriNet` object.
    ///
    pub fn as_vasm(&mut self) -> StateMachine {
        StateMachine::from_model(self.net)
    }
}

impl<'a> FlowDsl for Builder<'a> {
    fn model_type(&mut self, model_type: &str) {
        self.net.model_type = model_type.to_string();
    }

    fn cell<'b>(
        &mut self,
        label: &'b str,
        initial: Option<i32>,
        capacity: Option<i32>,
        x: i32,
        y: i32,
    ) -> &'b str {
        let offset = self.net.places.len() as i32;
        self.net.add_place(label, offset, initial, capacity, x, y);
        return label;
    }

    fn func<'b>(&mut self, label: &'b str, role: &str, x: i32, y: i32) -> &'b str {
        self.net.add_transition(label, role, x, y);
        return label;
    }

    fn arrow(&mut self, source: &str, target: &str, weight: i32) {
        assert!(weight > 0, "weight must be positive");
        self.net
            .add_arc(source, target, Some(weight), None, None, None, None);
    }

    fn guard(&mut self, source: &str, target: &str, weight: i32) {
        assert!(weight > 0, "weight must be positive");
        self.net
            .add_arc(source, target, None, None, None, Some(true), None);
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::vasm::{Transaction, Vasm};

    use super::*;

    struct TestModel {
        model: Model,
        state: Vec<i32>,
    }

    impl TestModel {

        fn to_link(&self) -> String {
            format!("{}{}", "https://pflow.dev/p/?z=", self.model.net.to_zblob().base64_zipped.replace(" ", "+"))
        }

        fn new() -> Self {
            let model = Model::new(model_test_code);
            let state = model.vm.initial_vector().clone();
            Self {
                model,
                state,
            }
        }

        fn assert_underflow(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.underflow.unwrap(), "expected underflow");
            res
        }

        fn assert_overflow(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.overflow.unwrap(), "expected overflow");
            res
        }

        fn assert_inhibited(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.inhibited.unwrap(), "expected inhibited");
            res
        }

        fn assert_fail(&self, action: &str) -> Transaction {
            let res = self.model.vm.transform(&self.state, action, 1);
            println!("{:?}", res);
            assert!(res.is_err(), "expected fail");
            res
        }

        fn assert_pass(&mut self, action: &str) -> Transaction {
            let res = self.model.vm.transform(&self.state, action, 1);
            println!("{:?}", res);
            assert!(res.is_ok(), "expected pass");
            self.state = res.output.clone();
            res
        }
    }

    fn model_test_code(p: &mut dyn FlowDsl) {
        p.model_type("petriNet");

        let r = "default";
        let foo = p.cell("foo", Option::from(1), Option::from(3), 707, 364);
        let bar = p.func("bar", r, 560, 480);
        let baz = p.func("baz", r, 850, 480);
        let inc = p.func("inc", r, 560, 240);
        let dec = p.func("dec", r, 850, 240);

        p.arrow(inc, foo, 1);
        p.arrow(foo, dec, 1);
        p.guard(bar, foo, 3);
        p.guard(foo, baz, 1);
    }

    #[test]
    fn test_loading_dsl() {
        let m = &mut TestModel::new();
        println!("{}", m.to_link());
        m.assert_pass("bar");
        // FIXME inhibited flags not working
        // m.assert_inhibited("baz");

        m.assert_pass("inc");
        m.assert_pass("inc");
        m.assert_overflow("inc"); // fail due to capacity
        m.assert_pass("dec");
        m.assert_pass("dec");
        m.assert_pass("dec");

        // FIXME inhibited flags not working
        // m.assert_underflow("dec");
    }
}
