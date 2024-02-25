use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dsl::FlowDsl;
use crate::petri_net::PetriNet;

pub type Label = String;

pub type RoleMap = HashMap<String, bool>;

pub type Vector = Vec<i32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guard {
    delta: Vector,
    read: bool,
}

pub type GuardMap = HashMap<String, Guard>;

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

pub type TransitionMap = HashMap<String, Transition>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    initial: Vector,
    capacity: Vector,
    places: Vec<String>,
    transitions: TransitionMap,
    roles: RoleMap,
}

impl StateMachine {
    pub fn new(declaration: fn(&mut dyn FlowDsl)) -> Self {
        PetriNet::new().declare(declaration).as_vasm()
    }

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
                // FIXME
                // transitions.get_mut(&source).unwrap().guards.insert(target.clone(), Guard {
                //     delta: vec![0; vector_size],
                //     read: read,
                // });
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub ok: bool,
    pub output: Vector,
    pub role: String,
    pub inhibited: Option<bool>,
    pub overflow: Option<bool>,
    pub underflow: Option<bool>,
}

impl Transaction {
    pub fn is_ok(&self) -> bool {
        self.ok
    }
}

pub trait Vasm {
    fn empty_vector(&self) -> Vector;
    fn initial_vector(&self) -> Vector;
    fn transform(&self, state: &Vector, action: &str, multiple: i32) -> Transaction;
}

pub fn declare(declaration: fn(&mut dyn FlowDsl)) -> Box<dyn Vasm> {
    Box::from(PetriNet::new().declare(declaration).as_vasm())
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
        for (i, v) in transition.delta.iter().enumerate() {
            output[i] += v * multiple;
        }
        let mut ok = true;
        let mut role = transition.role.clone();
        let mut inhibited = None;
        let overflow = None;
        let mut underflow = None;
        for (k, v) in transition.guards.iter() {
            let guard = v;
            let mut sum = 0;
            for (i, w) in guard.delta.iter().enumerate() {
                sum += w * state[i];
            }
            if guard.read {
                if sum > 0 {
                    ok = false;
                    role = k.clone();
                    inhibited = Some(true);
                }
            } else {
                if sum <= 0 {
                    ok = false;
                    role = k.clone();
                    inhibited = Some(false);
                }
            }
        }
        for (_, v) in output.iter().enumerate() {
            if v.to_be() < 0 {
                ok = false;
                role = transition.label.clone();
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