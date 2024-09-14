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
/// use pflow_metamodel::dsl::Dsl;
/// use pflow_metamodel::vasm::Vasm;
/// fn model_test_code(p: &mut dyn Dsl) {
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
pub trait Dsl {
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

impl Dsl for Builder<'_> {
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
        let offset: i32 = self.net.places.len().try_into().expect("too many places");
        self.net.add_place(label, offset, initial, capacity, x, y);
        label
    }

    fn func<'b>(&mut self, label: &'b str, role: &str, x: i32, y: i32) -> &'b str {
        self.net.add_transition(label, role, x, y);
        label
    }

    fn arrow(&mut self, source: &str, target: &str, weight: i32) {
        assert!(weight > 0, "weight must be positive");
        self.net
            .add_arc(ArcParams {
                source,
                target,
                weight: Some(weight),
                consume: None,
                produce: None,
                inhibit: None,
                read: None,
            });
    }

    fn guard(&mut self, source: &str, target: &str, weight: i32) {
        assert!(weight > 0, "weight must be positive");
        self.net.add_arc(ArcParams {
            source,
            target,
            weight: Some(weight),
            consume: Some(true),
            produce: None,
            inhibit: Some(true),
            read: None,
        });
    }
}

pub struct ArcParams<'a> {
    pub source: &'a str,
    pub target: &'a str,
    pub weight: Option<i32>,
    pub consume: Option<bool>,
    pub produce: Option<bool>,
    pub inhibit: Option<bool>,
    pub read: Option<bool>,
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
            format!(
                "{}{}",
                "http://example.com/?z=",
                self.model.net.to_zblob().base64_zipped.replace(' ', "+")
            )
        }

        fn new() -> Self {
            let model = Model::new(model_test_code);
            let state = model.vm.initial_vector();
            Self { model, state }
        }

        fn assert_underflow(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.underflow, "expected underflow");
            res
        }

        fn assert_overflow(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.overflow, "expected overflow");
            res
        }

        fn assert_inhibited(&self, action: &str) -> Transaction {
            let res = self.assert_fail(action);
            assert!(res.inhibited, "expected inhibited");
            res
        }

        fn assert_fail(&self, action: &str) -> Transaction {
            let res = self.model.vm.transform(&self.state, action, 1);
            assert!(res.is_err(), "expected fail");
            res
        }

        fn assert_pass(&mut self, action: &str) -> Transaction {
            let res = self.model.vm.transform(&self.state, action, 1);
            assert!(res.is_ok(), "expected pass");
            self.state.clone_from(&res.output);
            res
        }
    }

    fn model_test_code(p: &mut dyn Dsl) {
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
        assert_eq!(m.state, vec![1]);

        println!("link: {}", m.to_link()); // compare w/ GUI

        m.assert_inhibited("bar");
        m.assert_inhibited("baz");
        m.assert_pass("inc");
        m.assert_pass("inc");
        m.assert_pass("bar"); // enabled
        m.assert_overflow("inc"); // fail due to capacity
        m.assert_pass("dec");
        m.assert_pass("dec");
        m.assert_pass("dec");
        m.assert_underflow("dec");
        m.assert_pass("baz"); // enabled
    }
}
