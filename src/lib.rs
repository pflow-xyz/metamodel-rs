//! > **Metamodel - A Rust Library for Abstract State Machine Modeling**
//!
//! See the code below for an example of how to create a Petri-net model using the `pflow_metamodel` library.
//!
//! Also see: [Macro pflow_metamodel::pflow_dsl](macro.pflow_dsl.html) for creating models using an internal rust DSL rather than json.
//! ```
//! use pflow_metamodel::*;
//!
//! let model: Model = pflow_json!{{
//!    "modelType": "petriNet",
//!    "version": "v0",
//!    "places": {
//!      "place0": { "offset": 0, "capacity": 3, "x": 100, "y": 180 }
//!    },
//!    "transitions": {
//!      "txn0": { "role": "role0", "x": 20, "y": 100 },
//!      "txn1": { "role": "role0", "x": 180, "y": 100 },
//!      "txn2": { "role": "role0", "x": 20, "y": 260 },
//!      "txn3": { "role": "role0", "x": 180, "y": 260 }
//!    },
//!    "arcs": [
//!      { "source": "txn0", "target": "place0" },
//!      { "source": "place0", "target": "txn1", "weight": 3 },
//!      { "source": "txn2", "target": "place0", "weight": 3, "inhibit": true },
//!      { "source": "place0", "target": "txn3", "inhibit": true }
//!    ]
//! }};
//!
//! let state = model.vm.initial_vector();
//! assert_eq!(state, vec![0]);
//! let res = model.vm.transform(&state, "txn0", 1);
//! assert!(res.ok);
//! assert_eq!(state, vec![0]); // input state is _not_ mutated
//! assert_eq!(res.output, vec![1]);
//! let t = model.net.transitions.get("txn0");
//! assert!(t.is_some());
//! ```
//!
//! - Provides a DSL-driven framework for modeling and simulating Petri-nets, wf-nets, and DFAs.
//! - State machine data types are executed as a [Vector Addition State Machine (VASM)](https://en.wikipedia.org/wiki/Vector_addition_system).
//! - Data models are viewable / shareable in browsers by using [https://pflow.dev](https://pflow.dev/p/zb2rhkizUC1o2JuvgwhbH1XrLZkdK8x66pP1KR7sWAEw9c5FE/) JSON format.
//! - Models can be compressed and shared as base64 encoded blobs.
//! - Models can be consistently hashed and shared as CIDs.
//!
//! ![pflow][pflow]
//!
//! [pflow]: data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyODAiIGhlaWdodD0iMjgwIiB2aWV3Qm94PSItMzggNDIgMjgwIDI4MCI+CjxkZWZzPjxtYXJrZXIgaWQ9Im1hcmtlckFycm93MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48cGF0aCBkPSJNMiwyIEwyLDExIEwxMCw2IEwyLDIiLz48L21hcmtlcj48bWFya2VyIGlkPSJtYXJrZXJJbmhpYml0MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48Y2lyY2xlIGN4PSI1IiBjeT0iNi41IiByPSI0Ii8+PC9tYXJrZXI+PC9kZWZzPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMTAyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1NiIgeT0iMTM4IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iOTkiIHkxPSIxODMiIHgyPSIxODIiIHkyPSIxMDIiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIxMzYiIHk9IjEzOCIgZm9udC1zaXplPSJzbWFsbCI+MzwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMjYyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJJbmhpYml0MSkiIC8+Cjx0ZXh0IHg9IjU2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjM8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI5OSIgeTE9IjE4MyIgeDI9IjE4MiIgeTI9IjI2MiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckluaGliaXQxKSIgLz4KPHRleHQgeD0iMTM2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9Ijk5IiBjeT0iMTgzIiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjgxIiB5PSIxNjMiIGZvbnQtc2l6ZT0ic21hbGwiPnBsYWNlMDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSI4NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjE2NSIgeT0iNzciIGZvbnQtc2l6ZT0ic21hbGwiPnR4bjE8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjUiIHk9IjI0NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmYWI1YjAiIHJ4PSI0IiAvPjx0ZXh0IHg9IjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMjwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSIyNDUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSIxNjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMzwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNSIgeT0iODUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI1IiB5PSI3NyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMDwvdGV4dD4KPC9nPgo8L3N2Zz4=
//!
//! # Dining Philosophers Example
//!
//! The following example demonstrates the dining philosophers problem using the `PetriNet` struct.
//!
//! - read more about the [dining philosophers problem](https://pflow.dev/examples-dining-philosophers).
//! - interact with the model [dining philosophers model](https://pflow.dev/p/zb2rhimQLDKMY6yBXMLV2DJyCPqseb9kTJKdjiwKgzQEgwGvt/)
//! ```rust
//! use pflow_metamodel::*;
//!
//! let model = pflow_json!{{
//! "modelType": "petriNet",
//! "version": "v0",
//! "places": {
//!   "chopstick0": { "offset": 0, "initial": 1, "x": 403, "y": 340 },
//!   "chopstick1": { "offset": 1, "initial": 1, "x": 534, "y": 345 },
//!   "chopstick2": { "offset": 2, "initial": 1, "x": 358, "y": 467 },
//!   "chopstick3": { "offset": 3, "initial": 1, "x": 547, "y": 461 },
//!   "chopstick4": { "offset": 4, "initial": 1, "x": 451, "y": 536 },
//!   "0right": { "offset": 5, "x": 415, "y": 181 },
//!   "0left": { "offset": 6, "x": 545, "y": 177 },
//!   "1right": { "offset": 7, "x": 719, "y": 288 },
//!   "1left": { "offset": 8, "x": 769, "y": 404 },
//!   "2left": { "offset": 9, "x": 686, "y": 584 },
//!   "2right": { "offset": 10, "x": 594, "y": 678 },
//!   "3left": { "offset": 11, "x": 315, "y": 679 },
//!   "3right": { "offset": 12, "x": 216, "y": 608 },
//!   "4right": { "offset": 13, "x": 148, "y": 397 },
//!   "4left": { "offset": 14, "x": 183, "y": 289 }
//! },
//! "transitions": {
//!   "0think": { "x": 478, "y": 106 },
//!   "0eat": { "x": 473, "y": 247 },
//!   "1eat": { "x": 654, "y": 396 },
//!   "2eat": { "x": 574, "y": 573 },
//!   "3eat": { "x": 333, "y": 556 },
//!   "4eat": { "x": 267, "y": 370 },
//!   "1think": { "x": 842, "y": 304 },
//!   "4think": { "x": 72, "y": 314 },
//!   "3think": { "x": 200, "y": 726 },
//!   "2think": { "x": 740, "y": 699 }
//! },
//! "arcs": [
//!   { "source": "chopstick0", "target": "0eat" },
//!   { "source": "chopstick1", "target": "0eat" },
//!   { "source": "chopstick0", "target": "4eat" },
//!   { "source": "chopstick2", "target": "4eat" },
//!   { "source": "chopstick1", "target": "1eat" },
//!   { "source": "chopstick3", "target": "1eat" },
//!   { "source": "chopstick2", "target": "3eat" },
//!   { "source": "chopstick4", "target": "3eat" },
//!   { "source": "chopstick4", "target": "2eat" },
//!   { "source": "chopstick3", "target": "2eat" },
//!   { "source": "0eat", "target": "0right" },
//!   { "source": "0eat", "target": "0left" },
//!   { "source": "1eat", "target": "1right" },
//!   { "source": "1eat", "target": "1left" },
//!   { "source": "2eat", "target": "2left" },
//!   { "source": "2eat", "target": "2right" },
//!   { "source": "3eat", "target": "3right" },
//!   { "source": "3eat", "target": "3left" },
//!   { "source": "4eat", "target": "4right" },
//!   { "source": "4eat", "target": "4left" },
//!   { "source": "0right", "target": "0think" },
//!   { "source": "0left", "target": "0think" },
//!   { "source": "1right", "target": "1think" },
//!   { "source": "1left", "target": "1think" },
//!   { "source": "2left", "target": "2think" },
//!   { "source": "2right", "target": "2think" },
//!   { "source": "4left", "target": "4think" },
//!   { "source": "4right", "target": "4think" },
//!   { "source": "3right", "target": "3think" },
//!   { "source": "3left", "target": "3think" },
//!   { "source": "4think", "target": "chopstick0" },
//!   { "source": "4think", "target": "chopstick2" },
//!   { "source": "0think", "target": "chopstick0" },
//!   { "source": "0think", "target": "chopstick1" },
//!   { "source": "1think", "target": "chopstick1" },
//!   { "source": "1think", "target": "chopstick3" },
//!   { "source": "2think", "target": "chopstick3" },
//!   { "source": "2think", "target": "chopstick4" },
//!   { "source": "3think", "target": "chopstick2" },
//!   { "source": "3think", "target": "chopstick4" }
//! ]
//! }};
//! ```
//!
//! ![dining_philosophers][dining_philosophers]
//!
//! [dining_philosophers]: data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4OTAiIGhlaWdodD0iNzQwIiB2aWV3Qm94PSIxMiA0NiA4OTAgNzQwIj4KPGRlZnM+PG1hcmtlciBpZD0ibWFya2VyQXJyb3cxIiBtYXJrZXJXaWR0aD0iMjMiIG1hcmtlckhlaWdodD0iMTMiIHJlZlg9IjMxIiByZWZZPSI2IiBvcmllbnQ9ImF1dG8iPjxyZWN0IHdpZHRoPSIyOCIgaGVpZ2h0PSIzIiBmaWxsPSJ3aGl0ZSIgc3Ryb2tlPSJ3aGl0ZSIgeD0iMyIgeT0iNSIvPjxwYXRoIGQ9Ik0yLDIgTDIsMTEgTDEwLDYgTDIsMiIvPjwvbWFya2VyPjxtYXJrZXIgaWQ9Im1hcmtlckluaGliaXQxIiBtYXJrZXJXaWR0aD0iMjMiIG1hcmtlckhlaWdodD0iMTMiIHJlZlg9IjMxIiByZWZZPSI2IiBvcmllbnQ9ImF1dG8iPjxyZWN0IHdpZHRoPSIyOCIgaGVpZ2h0PSIzIiBmaWxsPSJ3aGl0ZSIgc3Ryb2tlPSJ3aGl0ZSIgeD0iMyIgeT0iNSIvPjxjaXJjbGUgY3g9IjUiIGN5PSI2LjUiIHI9IjQiLz48L21hcmtlcj48L2RlZnM+CjxnPgo8bGluZSB4MT0iNDAzIiB5MT0iMzQwIiB4Mj0iNDczIiB5Mj0iMjQ3IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNDM0IiB5PSIyODkiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI1MzQiIHkxPSIzNDUiIHgyPSI0NzMiIHkyPSIyNDciIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI0OTkiIHk9IjI5MiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjQwMyIgeTE9IjM0MCIgeDI9IjI2NyIgeTI9IjM3MCIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjMzMSIgeT0iMzUxIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iMzU4IiB5MT0iNDY3IiB4Mj0iMjY3IiB5Mj0iMzcwIiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMzA4IiB5PSI0MTQiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI1MzQiIHkxPSIzNDUiIHgyPSI2NTQiIHkyPSIzOTYiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1OTAiIHk9IjM2NiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjU0NyIgeTE9IjQ2MSIgeDI9IjY1NCIgeTI9IjM5NiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjU5NiIgeT0iNDI0IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iMzU4IiB5MT0iNDY3IiB4Mj0iMzMzIiB5Mj0iNTU2IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMzQxIiB5PSI1MDciIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI0NTEiIHkxPSI1MzYiIHgyPSIzMzMiIHkyPSI1NTYiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIzODgiIHk9IjU0MiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjQ1MSIgeTE9IjUzNiIgeDI9IjU3NCIgeTI9IjU3MyIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjUwOCIgeT0iNTUwIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNTQ3IiB5MT0iNDYxIiB4Mj0iNTc0IiB5Mj0iNTczIiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNTU2IiB5PSI1MTMiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI0NzMiIHkxPSIyNDciIHgyPSI0MTUiIHkyPSIxODEiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI0NDAiIHk9IjIxMCIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjQ3MyIgeTE9IjI0NyIgeDI9IjU0NSIgeTI9IjE3NyIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjUwNSIgeT0iMjA4IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNjU0IiB5MT0iMzk2IiB4Mj0iNzE5IiB5Mj0iMjg4IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNjgyIiB5PSIzMzgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI2NTQiIHkxPSIzOTYiIHgyPSI3NjkiIHkyPSI0MDQiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI3MDciIHk9IjM5NiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjU3NCIgeTE9IjU3MyIgeDI9IjY4NiIgeTI9IjU4NCIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjYyNiIgeT0iNTc0IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNTc0IiB5MT0iNTczIiB4Mj0iNTk0IiB5Mj0iNjc4IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNTgwIiB5PSI2MjEiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSIzMzMiIHkxPSI1NTYiIHgyPSIyMTYiIHkyPSI2MDgiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIyNzAiIHk9IjU3OCIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjMzMyIgeTE9IjU1NiIgeDI9IjMxNSIgeTI9IjY3OSIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjMyMCIgeT0iNjEzIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iMjY3IiB5MT0iMzcwIiB4Mj0iMTQ4IiB5Mj0iMzk3IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMjAzIiB5PSIzNzkiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSIyNjciIHkxPSIzNzAiIHgyPSIxODMiIHkyPSIyODkiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIyMjEiIHk9IjMyNSIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjQxNSIgeTE9IjE4MSIgeDI9IjQ3OCIgeTI9IjEwNiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjQ0MiIgeT0iMTM5IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNTQ1IiB5MT0iMTc3IiB4Mj0iNDc4IiB5Mj0iMTA2IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNTA3IiB5PSIxMzciIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI3MTkiIHkxPSIyODgiIHgyPSI4NDIiIHkyPSIzMDQiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI3NzYiIHk9IjI5MiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9Ijc2OSIgeTE9IjQwNCIgeDI9Ijg0MiIgeTI9IjMwNCIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjgwMSIgeT0iMzUwIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNjg2IiB5MT0iNTg0IiB4Mj0iNzQwIiB5Mj0iNjk5IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNzA5IiB5PSI2MzciIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI1OTQiIHkxPSI2NzgiIHgyPSI3NDAiIHkyPSI2OTkiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI2NjMiIHk9IjY4NCIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjE4MyIgeTE9IjI4OSIgeDI9IjcyIiB5Mj0iMzE0IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMTIzIiB5PSIyOTciIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSIxNDgiIHkxPSIzOTciIHgyPSI3MiIgeTI9IjMxNCIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjEwNiIgeT0iMzUxIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iMjE2IiB5MT0iNjA4IiB4Mj0iMjAwIiB5Mj0iNzI2IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMjA0IiB5PSI2NjMiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSIzMTUiIHkxPSI2NzkiIHgyPSIyMDAiIHkyPSI3MjYiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIyNTMiIHk9IjY5OCIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjcyIiB5MT0iMzE0IiB4Mj0iNDAzIiB5Mj0iMzQwIiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMjMzIiB5PSIzMjMiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI3MiIgeTE9IjMxNCIgeDI9IjM1OCIgeTI9IjQ2NyIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjIxMSIgeT0iMzg2IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iNDc4IiB5MT0iMTA2IiB4Mj0iNDAzIiB5Mj0iMzQwIiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNDM2IiB5PSIyMTkiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI0NzgiIHkxPSIxMDYiIHgyPSI1MzQiIHkyPSIzNDUiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1MDIiIHk9IjIyMSIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9Ijg0MiIgeTE9IjMwNCIgeDI9IjUzNCIgeTI9IjM0NSIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjY4NCIgeT0iMzIwIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iODQyIiB5MT0iMzA0IiB4Mj0iNTQ3IiB5Mj0iNDYxIiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iNjkwIiB5PSIzNzgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI3NDAiIHkxPSI2OTkiIHgyPSI1NDciIHkyPSI0NjEiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI2MzkiIHk9IjU3NiIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9Ijc0MCIgeTE9IjY5OSIgeDI9IjQ1MSIgeTI9IjUzNiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckFycm93MSkiIC8+Cjx0ZXh0IHg9IjU5MSIgeT0iNjEzIiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iMjAwIiB5MT0iNzI2IiB4Mj0iMzU4IiB5Mj0iNDY3IiBzdHJva2U9IiMwMDAwMDAiIG1hcmtlci1lbmQ9InVybCgjbWFya2VyQXJyb3cxKSIgLz4KPHRleHQgeD0iMjc1IiB5PSI1OTIiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSIyMDAiIHkxPSI3MjYiIHgyPSI0NTEiIHkyPSI1MzYiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIzMjEiIHk9IjYyNyIgZm9udC1zaXplPSJzbWFsbCI+MTwvdGV4dD4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iNTk0IiBjeT0iNjc4IiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjU3NiIgeT0iNjU4IiBmb250LXNpemU9InNtYWxsIj4ycmlnaHQ8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjIxNiIgY3k9IjYwOCIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSIxOTgiIHk9IjU4OCIgZm9udC1zaXplPSJzbWFsbCI+M3JpZ2h0PC90ZXh0Pgo8L2c+CjxnPgo8Y2lyY2xlIGN4PSI3NjkiIGN5PSI0MDQiIHI9IjE2IiBzdHJva2VXaWR0aD0iMS41IiBmaWxsPSIjZmZmZmZmIiBzdHJva2U9IiMwMDAwMDAiIG9yaWVudD0iMCIgc2hhcGVSZW5kZXJpbmc9ImF1dG8iIC8+PHRleHQgeD0iNzUxIiB5PSIzODQiIGZvbnQtc2l6ZT0ic21hbGwiPjFsZWZ0PC90ZXh0Pgo8L2c+CjxnPgo8Y2lyY2xlIGN4PSI1MzQiIGN5PSIzNDUiIHI9IjE2IiBzdHJva2VXaWR0aD0iMS41IiBmaWxsPSIjZmZmZmZmIiBzdHJva2U9IiMwMDAwMDAiIG9yaWVudD0iMCIgc2hhcGVSZW5kZXJpbmc9ImF1dG8iIC8+PHRleHQgeD0iNTE2IiB5PSIzMjUiIGZvbnQtc2l6ZT0ic21hbGwiPmNob3BzdGljazE8L3RleHQ+PGNpcmNsZSBjeD0iNTM0IiBjeT0iMzQ1IiByPSIyIiBmaWxsPSIjMDAwMDAwIiBzdHJva2U9IiMwMDAwMDAiIG9yaWVudD0iMCIgY2xhc3NOYW1lPSJ0b2tlbnMiIC8+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjM1OCIgY3k9IjQ2NyIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSIzNDAiIHk9IjQ0NyIgZm9udC1zaXplPSJzbWFsbCI+Y2hvcHN0aWNrMjwvdGV4dD4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iNzE5IiBjeT0iMjg4IiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjcwMSIgeT0iMjY4IiBmb250LXNpemU9InNtYWxsIj4xcmlnaHQ8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjU0NSIgY3k9IjE3NyIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSI1MjciIHk9IjE1NyIgZm9udC1zaXplPSJzbWFsbCI+MGxlZnQ8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjY4NiIgY3k9IjU4NCIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSI2NjgiIHk9IjU2NCIgZm9udC1zaXplPSJzbWFsbCI+MmxlZnQ8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjU0NyIgY3k9IjQ2MSIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSI1MjkiIHk9IjQ0MSIgZm9udC1zaXplPSJzbWFsbCI+Y2hvcHN0aWNrMzwvdGV4dD48Y2lyY2xlIGN4PSI1NDciIGN5PSI0NjEiIHI9IjIiIGZpbGw9IiMwMDAwMDAiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBjbGFzc05hbWU9InRva2VucyIgLz4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iMzE1IiBjeT0iNjc5IiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjI5NyIgeT0iNjU5IiBmb250LXNpemU9InNtYWxsIj4zbGVmdDwvdGV4dD4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iMTgzIiBjeT0iMjg5IiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjE2NSIgeT0iMjY5IiBmb250LXNpemU9InNtYWxsIj40bGVmdDwvdGV4dD48Y2lyY2xlIGN4PSIxODMiIGN5PSIyODkiIHI9IjIiIGZpbGw9IiMwMDAwMDAiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBjbGFzc05hbWU9InRva2VucyIgLz4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iMTQ4IiBjeT0iMzk3IiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjEzMCIgeT0iMzc3IiBmb250LXNpemU9InNtYWxsIj40cmlnaHQ8L3RleHQ+PGNpcmNsZSBjeD0iMTQ4IiBjeT0iMzk3IiByPSIyIiBmaWxsPSIjMDAwMDAwIiBzdHJva2U9IiMwMDAwMDAiIG9yaWVudD0iMCIgY2xhc3NOYW1lPSJ0b2tlbnMiIC8+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjQ1MSIgY3k9IjUzNiIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSI0MzMiIHk9IjUxNiIgZm9udC1zaXplPSJzbWFsbCI+Y2hvcHN0aWNrNDwvdGV4dD48Y2lyY2xlIGN4PSI0NTEiIGN5PSI1MzYiIHI9IjIiIGZpbGw9IiMwMDAwMDAiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBjbGFzc05hbWU9InRva2VucyIgLz4KPC9nPgo8Zz4KPGNpcmNsZSBjeD0iNDE1IiBjeT0iMTgxIiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjM5NyIgeT0iMTYxIiBmb250LXNpemU9InNtYWxsIj4wcmlnaHQ8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9IjQwMyIgY3k9IjM0MCIgcj0iMTYiIHN0cm9rZVdpZHRoPSIxLjUiIGZpbGw9IiNmZmZmZmYiIHN0cm9rZT0iIzAwMDAwMCIgb3JpZW50PSIwIiBzaGFwZVJlbmRlcmluZz0iYXV0byIgLz48dGV4dCB4PSIzODUiIHk9IjMyMCIgZm9udC1zaXplPSJzbWFsbCI+Y2hvcHN0aWNrMDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMzE2IiB5PSI1MzkiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjZmZmZmZmIiByeD0iNCIgLz48dGV4dCB4PSIzMTYiIHk9IjUzMSIgZm9udC1zaXplPSJzbWFsbCI+M2VhdDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNDU2IiB5PSIyMzAiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjZmZmZmZmIiByeD0iNCIgLz48dGV4dCB4PSI0NTYiIHk9IjIyMiIgZm9udC1zaXplPSJzbWFsbCI+MGVhdDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNjM3IiB5PSIzNzkiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI2MzciIHk9IjM3MSIgZm9udC1zaXplPSJzbWFsbCI+MWVhdDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iODI1IiB5PSIyODciIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjZmZmZmZmIiByeD0iNCIgLz48dGV4dCB4PSI4MjUiIHk9IjI3OSIgZm9udC1zaXplPSJzbWFsbCI+MXRoaW5rPC90ZXh0Pgo8L2c+CjxnPgo8cmVjdCB4PSI1NTciIHk9IjU1NiIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiM2MmZhNzUiIHJ4PSI0IiAvPjx0ZXh0IHg9IjU1NyIgeT0iNTQ4IiBmb250LXNpemU9InNtYWxsIj4yZWF0PC90ZXh0Pgo8L2c+CjxnPgo8cmVjdCB4PSIxODMiIHk9IjcwOSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjE4MyIgeT0iNzAxIiBmb250LXNpemU9InNtYWxsIj4zdGhpbms8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjU1IiB5PSIyOTciIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI1NSIgeT0iMjg5IiBmb250LXNpemU9InNtYWxsIj40dGhpbms8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjI1MCIgeT0iMzUzIiB3aWR0aD0iMzAiIGhlaWdodD0iMzAiIHN0cm9rZT0iIzAwMDAwMCIgZmlsbD0iI2ZmZmZmZiIgcng9IjQiIC8+PHRleHQgeD0iMjUwIiB5PSIzNDUiIGZvbnQtc2l6ZT0ic21hbGwiPjRlYXQ8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjcyMyIgeT0iNjgyIiB3aWR0aD0iMzAiIGhlaWdodD0iMzAiIHN0cm9rZT0iIzAwMDAwMCIgZmlsbD0iI2ZmZmZmZiIgcng9IjQiIC8+PHRleHQgeD0iNzIzIiB5PSI2NzQiIGZvbnQtc2l6ZT0ic21hbGwiPjJ0aGluazwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNDYxIiB5PSI4OSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjQ2MSIgeT0iODEiIGZvbnQtc2l6ZT0ic21hbGwiPjB0aGluazwvdGV4dD4KPC9nPgo8L3N2Zz4=
//!

// Rustc lints
// <https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html>
#![warn(
    anonymous_parameters,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
)]

// Clippy lints
// <https://rust-lang.github.io/rust-clippy/current/>
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::dbg_macro,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::mem_forget,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unwrap_used,
)]

// Allow some clippy lints
#![allow(
    clippy::cargo_common_metadata,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::enum_variant_names,
    clippy::if_not_else,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::struct_excessive_bools,
    clippy::use_self,
    clippy::multiple_crate_versions,
    clippy::struct_field_names,
    clippy::similar_names,
)]

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

/// The `zblob` contains utilities to facilitate loading zipped blob data as petri-nets.
pub mod zblob;

/// The `model` encapsulates the `PetriNet` and `Vasm` objects into a single `Model` object.
pub mod model;

pub use crate::model::*;
pub use crate::vasm::*;


/// Create a model using the pflow DSL
/// This is the primary way to create a model for most use cases
///
/// ![pflow][pflow]
///
/// [pflow]: data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyODAiIGhlaWdodD0iMjgwIiB2aWV3Qm94PSItMzggNDIgMjgwIDI4MCI+CjxkZWZzPjxtYXJrZXIgaWQ9Im1hcmtlckFycm93MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48cGF0aCBkPSJNMiwyIEwyLDExIEwxMCw2IEwyLDIiLz48L21hcmtlcj48bWFya2VyIGlkPSJtYXJrZXJJbmhpYml0MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48Y2lyY2xlIGN4PSI1IiBjeT0iNi41IiByPSI0Ii8+PC9tYXJrZXI+PC9kZWZzPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMTAyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1NiIgeT0iMTM4IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iOTkiIHkxPSIxODMiIHgyPSIxODIiIHkyPSIxMDIiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIxMzYiIHk9IjEzOCIgZm9udC1zaXplPSJzbWFsbCI+MzwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMjYyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJJbmhpYml0MSkiIC8+Cjx0ZXh0IHg9IjU2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjM8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI5OSIgeTE9IjE4MyIgeDI9IjE4MiIgeTI9IjI2MiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckluaGliaXQxKSIgLz4KPHRleHQgeD0iMTM2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9Ijk5IiBjeT0iMTgzIiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjgxIiB5PSIxNjMiIGZvbnQtc2l6ZT0ic21hbGwiPnBsYWNlMDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSI4NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjE2NSIgeT0iNzciIGZvbnQtc2l6ZT0ic21hbGwiPnR4bjE8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjUiIHk9IjI0NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmYWI1YjAiIHJ4PSI0IiAvPjx0ZXh0IHg9IjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMjwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSIyNDUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSIxNjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMzwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNSIgeT0iODUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI1IiB5PSI3NyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMDwvdGV4dD4KPC9nPgo8L3N2Zz4=
/// # Example
///
/// ```
/// use pflow_metamodel::*;
///
/// let model: Model = pflow_dsl!{
///    declare "petriNet"
///    cell "place0", 0, 3, [100, 180]
///    func "txn0", "default", [20, 100]
///    func "txn1", "default", [180, 100]
///    func "txn2", "default", [20, 260]
///    func "txn3", "default", [180, 260]
///    arrow "txn0", "place0", 1
///    arrow "place0", "txn1", 3
///    guard "txn2", "place0", 3
///    guard "place0", "txn3", 1
/// };
///
/// let state = model.vm.initial_vector();
/// assert_eq!(state, vec![0]);
/// let res = model.vm.transform(&state, "txn0", 1);
/// assert!(res.ok);
/// assert_eq!(state, vec![0]); // input state is _not_ mutated
/// assert_eq!(res.output, vec![1]);
/// let t = model.net.transitions.get("txn0");
/// assert!(t.is_some());
/// ```
#[macro_export]
macro_rules! pflow_dsl {
    ($($name:ident $($args:expr),*)*) => {{
        declaration_function! {
            |p: &mut dyn dsl::Dsl| {
                #[allow(unused)]
                fn declare(p: &mut dyn dsl::Dsl, model_type: &str) {
                    p.model_type(model_type);
                }
                #[allow(unused)]
                fn cell(p: &mut dyn dsl::Dsl, name: &str, initial: i32, capacity: i32, coords: [i32; 2]) {
                    p.cell(name, Option::from(initial), Option::from(capacity), coords[0],coords[1]);
                }
                #[allow(unused)]
                fn func(p: &mut dyn dsl::Dsl, name: &str, role: &str, coords: [i32; 2]) {
                    p.func(name, role, coords[0], coords[1]);
                }
                #[allow(unused)]
                fn arrow(p: &mut dyn dsl::Dsl, source: &str, target: &str, weight: i32) {
                    p.arrow(source, target, weight);
                }
                #[allow(unused)]
                fn guard(p: &mut dyn dsl::Dsl, source: &str, target: &str, weight: i32) {
                    p.guard(source, target, weight);
                }
                $(
                    $name(p, $($args),*);
                )*
            }
        }
    }};
}

/// Create a model using the internal DSL functions without macro rewriting
/// Generally not used directly, but may be useful for deeper integration with other libraries
///
/// ![pflow][pflow]
///
/// [pflow]: data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyODAiIGhlaWdodD0iMjgwIiB2aWV3Qm94PSItMzggNDIgMjgwIDI4MCI+CjxkZWZzPjxtYXJrZXIgaWQ9Im1hcmtlckFycm93MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48cGF0aCBkPSJNMiwyIEwyLDExIEwxMCw2IEwyLDIiLz48L21hcmtlcj48bWFya2VyIGlkPSJtYXJrZXJJbmhpYml0MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48Y2lyY2xlIGN4PSI1IiBjeT0iNi41IiByPSI0Ii8+PC9tYXJrZXI+PC9kZWZzPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMTAyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1NiIgeT0iMTM4IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iOTkiIHkxPSIxODMiIHgyPSIxODIiIHkyPSIxMDIiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIxMzYiIHk9IjEzOCIgZm9udC1zaXplPSJzbWFsbCI+MzwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMjYyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJJbmhpYml0MSkiIC8+Cjx0ZXh0IHg9IjU2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjM8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI5OSIgeTE9IjE4MyIgeDI9IjE4MiIgeTI9IjI2MiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckluaGliaXQxKSIgLz4KPHRleHQgeD0iMTM2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9Ijk5IiBjeT0iMTgzIiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjgxIiB5PSIxNjMiIGZvbnQtc2l6ZT0ic21hbGwiPnBsYWNlMDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSI4NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjE2NSIgeT0iNzciIGZvbnQtc2l6ZT0ic21hbGwiPnR4bjE8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjUiIHk9IjI0NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmYWI1YjAiIHJ4PSI0IiAvPjx0ZXh0IHg9IjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMjwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSIyNDUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSIxNjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMzwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNSIgeT0iODUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI1IiB5PSI3NyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMDwvdGV4dD4KPC9nPgo8L3N2Zz4=
///
/// # Example
///
/// ```
/// use pflow_metamodel::*;
///
/// let model = declaration_function! {
///    |p: &mut dyn dsl::Dsl| {
///       p.model_type("petriNet");
///       p.cell("place0", Option::from(0), Option::from(3), 100, 180);
///       p.func("txn0", "default", 20, 100);
///       p.func("txn1", "default", 180, 100);
///       p.func("txn2", "default", 20, 260);
///       p.func("txn3", "default", 180, 260);
///       p.arrow("txn0", "place0", 1);
///       p.arrow("place0", "txn1", 3);
///       p.guard("txn2", "place0", 3);
///       p.guard("place0", "txn3", 1);
///     }
/// };
///
/// let state = model.vm.initial_vector();
/// assert_eq!(state, vec![0]);
/// let res = model.vm.transform(&state, "txn0", 1);
/// assert!(res.ok);
/// assert_eq!(state, vec![0]); // input state is _not_ mutated
/// assert_eq!(res.output, vec![1]);
/// let t = model.net.transitions.get("txn0");
/// assert!(t.is_some());
/// ```
#[macro_export]
macro_rules! declaration_function {
    ($($flow_dsl:tt)*) => {{
        let model = model::Model::new(
            $($flow_dsl)*
        );
        model
    }};
}

/// Create a model from a JSON string compatible with pflow.xyz
///
/// ![pflow][pflow]
///
/// [pflow]: data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyODAiIGhlaWdodD0iMjgwIiB2aWV3Qm94PSItMzggNDIgMjgwIDI4MCI+CjxkZWZzPjxtYXJrZXIgaWQ9Im1hcmtlckFycm93MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48cGF0aCBkPSJNMiwyIEwyLDExIEwxMCw2IEwyLDIiLz48L21hcmtlcj48bWFya2VyIGlkPSJtYXJrZXJJbmhpYml0MSIgbWFya2VyV2lkdGg9IjIzIiBtYXJrZXJIZWlnaHQ9IjEzIiByZWZYPSIzMSIgcmVmWT0iNiIgb3JpZW50PSJhdXRvIj48cmVjdCB3aWR0aD0iMjgiIGhlaWdodD0iMyIgZmlsbD0id2hpdGUiIHN0cm9rZT0id2hpdGUiIHg9IjMiIHk9IjUiLz48Y2lyY2xlIGN4PSI1IiBjeT0iNi41IiByPSI0Ii8+PC9tYXJrZXI+PC9kZWZzPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMTAyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSI1NiIgeT0iMTM4IiBmb250LXNpemU9InNtYWxsIj4xPC90ZXh0Pgo8L2c+CjxnPgo8bGluZSB4MT0iOTkiIHkxPSIxODMiIHgyPSIxODIiIHkyPSIxMDIiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJBcnJvdzEpIiAvPgo8dGV4dCB4PSIxMzYiIHk9IjEzOCIgZm9udC1zaXplPSJzbWFsbCI+MzwvdGV4dD4KPC9nPgo8Zz4KPGxpbmUgeDE9IjIyIiB5MT0iMjYyIiB4Mj0iOTkiIHkyPSIxODMiIHN0cm9rZT0iIzAwMDAwMCIgbWFya2VyLWVuZD0idXJsKCNtYXJrZXJJbmhpYml0MSkiIC8+Cjx0ZXh0IHg9IjU2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjM8L3RleHQ+CjwvZz4KPGc+CjxsaW5lIHgxPSI5OSIgeTE9IjE4MyIgeDI9IjE4MiIgeTI9IjI2MiIgc3Ryb2tlPSIjMDAwMDAwIiBtYXJrZXItZW5kPSJ1cmwoI21hcmtlckluaGliaXQxKSIgLz4KPHRleHQgeD0iMTM2IiB5PSIyMTgiIGZvbnQtc2l6ZT0ic21hbGwiPjE8L3RleHQ+CjwvZz4KPGc+CjxjaXJjbGUgY3g9Ijk5IiBjeT0iMTgzIiByPSIxNiIgc3Ryb2tlV2lkdGg9IjEuNSIgZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSIjMDAwMDAwIiBvcmllbnQ9IjAiIHNoYXBlUmVuZGVyaW5nPSJhdXRvIiAvPjx0ZXh0IHg9IjgxIiB5PSIxNjMiIGZvbnQtc2l6ZT0ic21hbGwiPnBsYWNlMDwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSI4NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmZmZmZmYiIHJ4PSI0IiAvPjx0ZXh0IHg9IjE2NSIgeT0iNzciIGZvbnQtc2l6ZT0ic21hbGwiPnR4bjE8L3RleHQ+CjwvZz4KPGc+CjxyZWN0IHg9IjUiIHk9IjI0NSIgd2lkdGg9IjMwIiBoZWlnaHQ9IjMwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiNmYWI1YjAiIHJ4PSI0IiAvPjx0ZXh0IHg9IjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMjwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iMTY1IiB5PSIyNDUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSIxNjUiIHk9IjIzNyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMzwvdGV4dD4KPC9nPgo8Zz4KPHJlY3QgeD0iNSIgeT0iODUiIHdpZHRoPSIzMCIgaGVpZ2h0PSIzMCIgc3Ryb2tlPSIjMDAwMDAwIiBmaWxsPSIjNjJmYTc1IiByeD0iNCIgLz48dGV4dCB4PSI1IiB5PSI3NyIgZm9udC1zaXplPSJzbWFsbCI+dHhuMDwvdGV4dD4KPC9nPgo8L3N2Zz4=
///
///
/// # Example
///
/// ```
/// use pflow_metamodel::*;
///
/// let model: Model = pflow_json!{{
///    "modelType": "petriNet",
///    "version": "v0",
///    "places": {
///      "place0": { "offset": 0, "capacity": 3, "x": 100, "y": 180 }
///    },
///    "transitions": {
///      "txn0": { "role": "role0", "x": 20, "y": 100 },
///      "txn1": { "role": "role0", "x": 180, "y": 100 },
///      "txn2": { "role": "role0", "x": 20, "y": 260 },
///      "txn3": { "role": "role0", "x": 180, "y": 260 }
///    },
///    "arcs": [
///      { "source": "txn0", "target": "place0" },
///      { "source": "place0", "target": "txn1", "weight": 3 },
///      { "source": "txn2", "target": "place0", "weight": 3, "inhibit": true },
///      { "source": "place0", "target": "txn3", "inhibit": true }
///    ]
/// }};
///
/// let state = model.vm.initial_vector();
/// assert_eq!(state, vec![0]);
/// let res = model.vm.transform(&state, "txn0", 1);
/// assert!(res.ok);
/// assert_eq!(state, vec![0]); // input state is _not_ mutated
/// assert_eq!(res.output, vec![1]);
/// let t = model.net.transitions.get("txn0");
/// assert!(t.is_some());
/// ```
#[macro_export]
macro_rules! pflow_json {
    ($($flow_json:tt)*) => {{
        let mut net = petri_net::PetriNet::from_json_value(
                serde_json::json!($($flow_json)*)
        ).expect("json fault");

        let sm = vasm::StateMachine::from_model(&mut net);
        model::Model {
            net,
            declaration: Vec::new(),
            vm: Box::new(sm),
        }
    }};
}

/// Create a model from a diagram string
///
/// Example:
///
/// ```rust
/// use pflow_metamodel::*;
/// use std::sync::{Arc, Mutex};
///
/// let model = pflow_diagram!{ ModelType::Workflow;
///     Water --> boil_water;
///     CoffeeBeans --> grind_beans;
///     BoiledWater --> brew_coffee;
///     GroundCoffee --> brew_coffee;
///     Filter --> brew_coffee;
///     CoffeeInPot --> pour_coffee;
///     Cup --> pour_coffee;
/// };
/// ```
#[macro_export]
macro_rules! pflow_diagram {
    ($($workflow_declaration:tt)*) => {
        {
            Model::from_diagram(stringify!($($workflow_declaration)*).to_string())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_coffee_machine() {
        let coffee_machine = pflow_diagram! { ModelType::PetriNet;
            Water --> boil_water;
            CoffeeBeans --> grind_beans;
            BoiledWater --> brew_coffee;
            GroundCoffee --> brew_coffee;
            Filter --> brew_coffee;
            CoffeeInPot --> pour_coffee;
            Cup --> pour_coffee;
        };
        let zblob = coffee_machine.net.to_zblob();
        println!("https://pflow.dev?z={}", zblob.base64_zipped);
    }

    #[test]
    fn test_dsl() {
        let j = pflow_dsl! {
            declare "petriNet"
            cell "b", 1, 0, [1, 1]
            cell "a", 1, 0, [1, 2]
            func "f", "default", [2, 1]
            func "g", "default", [2, 2]
            arrow "a", "f", 1
            guard "b", "g", 1
        };

        let initial = j.vm.initial_vector();
        assert_eq!(initial, vec![1, 1]);

        j.net.transitions.get("f").expect("expected transition");

        let res = j.vm.transform(&initial, "f", 1);
        assert!(res.ok);
        assert_eq!(initial, vec![1, 1]); // input state is _not_ mutated
        assert_eq!(res.output, vec![1, 0]);
    }

    #[test]
    fn test_json_dsl() {
        let j = pflow_json! {
            {
                "modelType": "petriNet",
                "version": "v0",
                "places": {
                    "a": { "offset": 0, "initial": 1, "capacity": 1, "x": 0, "y": 0 },
                    "b": { "offset": 1, "initial": 1, "capacity": 1, "x": 0, "y": 0 }
                },
                "transitions": {
                    "f": { "role": "default", "x": 0, "y": 0 }
                },
                "arcs": [
                    { "source": "a", "target": "f", "weight": 1 },
                    { "source": "b", "target": "f", "weight": 1 }
                ]
            }
        };

        let initial = j.vm.initial_vector();
        assert_eq!(initial, vec![1, 1]);
        j.net.transitions.get("f").expect("expected transition");
    }

    #[test]
    fn test_model() {
        let model = declaration_function! {
            |p: &mut dyn dsl::Dsl| {
                p.model_type("petriNet");
                p.cell("b", Some(1), None, 0, 0);
                p.func("f", "default", 1, 1);
                p.cell("a", Some(1), None, 0, 0);
                p.func("g", "default", 1, 1);
                p.arrow("a", "f", 1);
                p.guard("b", "g", 1);
            }
        };

        assert_eq!(model.net.model_type, "petriNet");
        let zblob = model.net.to_zblob();
        assert_eq!(
            zblob.ipfs_cid,
            "zb2rhXMTtKZq96QpdSzkSYmEPKttirMw4okCG8c5QxwygAvWG"
        );
    }

    #[test]
    fn test_workflow_model() {
        let model = pflow_dsl! {
            declare "workflow"
            cell "Water", 0, 1, [100, 300]
            cell "CoffeeBeans", 0, 1, [180, 300]
            cell "BoiledWater", 0, 1, [195, 397]
            cell "GroundCoffee", 0, 1, [250, 339]
            cell "Filter", 0, 1, [290, 280]
            cell "CoffeeInPot", 0, 1, [328, 366]
            cell "Cup", 0, 1, [365, 312]
            cell "step0", 1, 1, [100, 100]
            cell "step1", 0, 1, [180, 100]
            cell "step2", 0, 1, [260, 100]
            cell "step3", 0, 1, [340, 100]
            cell "step4", 0, 1, [420, 100]
            func "boil_water", "coffee_machine", [100, 200]
            func "brew_coffee", "coffee_machine", [260, 200]
            func "grind_beans", "coffee_machine", [180, 200]
            func "pour_coffee", "coffee_machine", [340, 200]
            arrow "Water", "boil_water", 1
            arrow "CoffeeBeans", "grind_beans", 1
            arrow "BoiledWater", "brew_coffee", 1
            arrow "GroundCoffee", "brew_coffee", 1
            arrow "Filter", "brew_coffee", 1
            arrow "CoffeeInPot", "pour_coffee", 1
            arrow "Cup", "pour_coffee", 1
            arrow "step0", "boil_water", 1
            arrow "boil_water", "step1", 1
            arrow "step1", "grind_beans", 1
            arrow "grind_beans", "step2", 1
            arrow "step2", "brew_coffee", 1
            arrow "brew_coffee", "step3", 1
            arrow "step3", "pour_coffee", 1
            arrow "pour_coffee", "step4", 1
        };

        let zb = model.net.to_zblob();
        println!("https://pflow.dev?z={}", zb.base64_zipped);
        assert_eq!(
            zb.ipfs_cid,
            "zb2rhcgvzu3CJ7KaRmySuR253VD2DFPqyQftHhDMKAPaQRzjE"
        );

        let state = Arc::new(Mutex::new(model.vm.initial_vector()));
        {
            let mut state_lock = state.lock().expect("state lock");
            let res = model.vm.transform(&state_lock, "boil_water", 1);
            if res.ok {
                println!("{res:?}");
                *state_lock = res.output;
                drop(state_lock);
            } else {
                panic!("expected ok");
            }
        }
    }
}
