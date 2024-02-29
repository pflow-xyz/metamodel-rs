use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dsl::FlowDsl;
use crate::petri_net::PetriNet;

/// RoleMap is a type alias for a HashMap that maps a string to a boolean.
pub type RoleMap = HashMap<String, bool>;

/// Vector is a type alias for a vector of 32-bit integers.
/// It is used to represent the state of a state machine and the delta of each transition or inhibitor.
pub type Vector = Vec<i32>;

/// Guard is a struct that represents a guard in a state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guard {
    delta: Vector,
    read: bool,
}

/// GuardMap is a type alias for a HashMap that maps a string to a `Guard`.
pub type GuardMap = HashMap<String, Guard>;

/// Transition is a struct that represents a transition in a state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    label: String,
    role: String,
    delta: Vector,
    guards: GuardMap,
    allow_reentry: bool,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            label: "".to_string(),
            role: "".to_string(),
            delta: vec![],
            guards: GuardMap::new(),
            allow_reentry: false,
        }
    }
}

/// TransitionMap is a type alias for a HashMap that maps a string to a `Transition`.
pub type TransitionMap = HashMap<String, Transition>;

/// StateMachine is a struct that holds the vectorized / executable form of a Petri-net.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    initial: Vector,
    capacity: Vector,
    places: Vec<String>,
    transitions: TransitionMap,
    roles: RoleMap,
}

impl StateMachine {

    /// Creates a new `StateMachine` object from the given `PetriNet`.
    pub fn new(declaration: fn(&mut dyn FlowDsl)) -> Self {
        PetriNet::new().declare(declaration).as_vasm()
    }

    /// Creates a new `StateMachine` object from the given `PetriNet`.
    pub fn from_model(model: &mut PetriNet) -> Self {
        model.populate_arc_attributes();
        let mut roles = RoleMap::new();
        model.transitions.iter().for_each(|(_, v)| {
            roles.insert(v.role.clone().unwrap_or("default".to_string()), true);
        });

        let vector_size = model.places.len();

        let mut transitions: TransitionMap = model.transitions.iter().map(|(k, v)| (k.clone(), Transition {
            label: k.clone(),
            role: v.role.clone().unwrap_or("default".to_string()),
            delta: vec![0; vector_size],
            guards: GuardMap::new(),
            allow_reentry: false,
        })).collect();

        model.arcs.iter().for_each(|arc| {
            let source = arc.source.clone();
            let target = arc.target.clone();
            let weight = arc.weight.unwrap_or(1);
            let consume = arc.consume.unwrap_or(false);
            let produce = arc.produce.unwrap_or(false);
            let inhibit = arc.inhibit.unwrap_or(false);
            let read = arc.read.unwrap_or(false);
            if inhibit {
                if read {
                    transitions.get_mut(&source).unwrap().guards.insert(target.clone(), Guard {
                        delta: vec![0; vector_size],
                        read: read,
                    });
                } else {
                    transitions.get_mut(&target).unwrap().guards.insert(source.clone(), Guard {
                        delta: vec![0; vector_size],
                        read: read,
                    });
                }
            } else {
                assert_ne!(produce, consume, "must be either produce or consume");
                if consume {
                    transitions.get_mut(&target).unwrap().delta[model.places.get(&source).unwrap().offset as usize] = 0 - weight;
                } else {
                    transitions.get_mut(&source).unwrap().delta[model.places.get(&target).unwrap().offset as usize] = weight;
                }
            }
        });

        let mut initial = vec![0; vector_size];
        let mut capacity = vec![0; vector_size];
        let mut places = vec!["".to_string(); vector_size];

        model.places.iter().for_each(|(k, v)| {
            initial[v.offset as usize] = v.initial.unwrap_or(0);
            capacity[v.offset as usize] = v.capacity.unwrap_or(0);
            places[v.offset as usize] = k.clone();
        });

        // REVIEW: assume maps iterate in the same order?
        Self {
            initial,
            capacity,
            places,
            transitions,
            roles,
        }
    }
}

/// `Transaction` is a struct that represents the result of a transformation in a state machine.
/// It provides information about the success of the transformation, the resulting state, the role that performed the transformation, and any errors that occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// A boolean indicating whether the transformation was successful.
    pub ok: bool,
    /// The resulting state after the transformation.
    pub output: Vector,
    /// The role that performed the transformation.
    pub role: String,
    /// An optional boolean indicating whether the transformation was inhibited.
    pub inhibited: Option<bool>,
    /// An optional boolean indicating whether an overflow occurred during the transformation.
    pub overflow: Option<bool>,
    /// An optional boolean indicating whether an underflow occurred during the transformation.
    pub underflow: Option<bool>,
}

impl Transaction {
    /// Checks if the transaction was successful.
    ///
    /// # Returns
    ///
    /// * A boolean indicating whether the transaction was successful.
    ///
    /// # Example
    ///
    /// ```
    /// if transaction.is_ok() {
    ///     println!("The transaction was successful.");
    /// } else {
    ///     println!("The transaction failed.");
    /// }
    /// ```
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn is_err(&self) -> bool {
        !self.ok
    }
}

/// `Vasm` is a trait that represents a [vector addition state machine](https://en.wikipedia.org/wiki/Vector_addition_system).
/// It provides methods to create an empty vector, get the initial Vector, and transform the state.
pub trait Vasm {
    /// Creates an empty vector with the same length as the number of places in the state machine.
    ///
    /// # Returns
    ///
    /// * A vector of zeros with the same length as the number of places in the state machine.
    fn empty_vector(&self) -> Vector;

    /// Gets the initial vector of the state machine.
    ///
    /// # Returns
    ///
    /// * The initial vector of the state machine.
    ///
    fn initial_vector(&self) -> Vector;

    /// Transforms the state of the state machine according to the given action and multiple.
    ///
    /// # Arguments
    ///
    /// * `state` - The current state of the state machine.
    /// * `action` - The action to be performed.
    /// * `multiple` - The multiple of the action to be performed.
    ///
    /// # Returns
    ///
    /// * A `Transaction` object that represents the result of the transformation.
    ///
    fn transform(&self, state: &Vector, action: &str, multiple: i32) -> Transaction;
}

impl dyn Vasm {
    pub fn new(declaration: fn(&mut dyn FlowDsl)) -> Box<Self> {
        Box::from(PetriNet::new().declare(declaration).as_vasm())
    }
}

impl Vasm for StateMachine {
    fn empty_vector(&self) -> Vector {
        vec![0; self.places.len()]
    }

    fn initial_vector(&self) -> Vector {
        self.initial.clone()
    }

    // REVIEW: test that this works properly
    fn transform(&self, state: &Vector, action: &str, multiple: i32) -> Transaction {
        let transition = self.transitions.get(action).unwrap_or_else(|| panic!("no transition for {}", action));
        let mut output = state.clone();
        let mut ok = true;
        let role = transition.role.clone();
        let mut inhibited = None;
        let mut overflow = None;
        let mut underflow = None;
        for (i, v) in transition.delta.iter().enumerate() {
            output[i] += v * multiple;
            if output[i] < 0 {
                underflow = Some(true);
                ok = false;
            }
            if output[i] > self.capacity[i] {
                ok = false;
                overflow = Some(true);
            }
        }
        for (_, v) in transition.guards.iter() {
            let guard = v;
            let mut sum = 0;
            for (i, w) in guard.delta.iter().enumerate() {
                sum += w * state[i];
            }
            if guard.read {
                if sum > 0 {
                    ok = false;
                    inhibited = Some(true);
                }
            } else {
                if sum <= 0 {
                    ok = false;
                    inhibited = Some(false);
                }
            }
        }
        for (_, v) in output.iter().enumerate() {
            if v.to_be() < 0 {
                ok = false;
                underflow = Some(true);
            }
        }
        Transaction {
            output,
            ok,
            role,
            inhibited,
            overflow,
            underflow,
        }
    }
}

#[test]
fn test_default_net() {
    let net = &mut PetriNet::new();
    let mut mm = net.declare(|m| {
        m.model_type("petriNet");
    });
    let vasm = mm.as_vasm();
    let state = vasm.initial_vector();
    assert!(state.len() == 0);
}
