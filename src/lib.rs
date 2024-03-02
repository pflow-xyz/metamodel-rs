//! > **Metamodel - A Rust Library for Abstract State Machine Modeling**
//!
//! - Provides a DSL-driven framework for modeling and simulating Petri-nets, wf-nets, and DFAs.
//! - State machine data types are executed as a [Vector Addition State Machine (VASM)](https://en.wikipedia.org/wiki/Vector_addition_system).
//! - Data models are viewable / shareable in browsers by using [https://pflow-dev.github.io/pflow-js/p/](https://pflow-dev.github.io/pflow-js/p/)

/// The `petri_net` module contains the definition and implementation of the `PetriNet` struct.
pub mod petri_net;

/// The `oid` module is used to generate CID's for the zipped blobs.
pub mod oid;

/// The `compression` module contains functions for zipping/unzipping models as sharable base64 blobs.
pub mod compression;

/// The `vasm` module contains the implementation of a Vector Addition State Machine (VASM).
pub mod vasm;

/// The `dsl` module contains `FlowDsl` and `Builder` traits for defining Petri-nets.
pub mod dsl;

/// The `fixtures` module contains test fixtures for the project (visible only in the test environment).
pub mod fixtures;

/// The `zblob` contains utilities to facilitate loading zipped blob data as petri-nets.
pub mod zblob;

/// The `model` encapsulates the `PetriNet` and `Vasm` objects into a single `Model` object.
pub mod model;
