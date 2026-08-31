# Coheron

[![Crates.io](https://img.shields.io/crates/v/coheron.svg?style=flat-square)](https://crates.io/crates/coheron)
[![Docs.rs](https://img.shields.io/docsrs/coheron?style=flat-square)](https://docs.rs/coheron)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/neil-crago/coheron/actions/workflows/rust.yml/badge.svg)](https://github.com/neil-crago/coheron/actions/workflows/rust.yml)

A small Rust library for belief representations, probabilistic fusion, and semantic control primitives.

## What is in here

The core crate currently focuses on a few concrete, testable building blocks:

- `GaussianBelief` for scalar Gaussian estimates with drift and variance updates
- `KalmanBelief` for a lightweight 1D state-space model
- `PolynomialBelief` for simple polynomial approximation
- `DirichletBelief` for categorical concentration parameters
- `BeliefFusion` and `FusionStrategy` for fusing multiple beliefs into a single posterior
- `SemanticState` and `ControlLaw` for simple control-oriented state values

This is intentionally a small, low-friction toolkit rather than a complete semantic engine framework.

## Current status

The library still builds on modern Rust, and the core belief/fusion APIs are covered by regression tests. The project had drifted toward aspirational architecture language, but the actual implementation is simpler and more practical than the original README suggested.

## Example usage

```rust
use coheron::beliefs::GaussianBelief;
use coheron::fusion::{BeliefFusion, GaussianFusion};

let beliefs = vec![
    GaussianBelief { mean: 0.0, variance: 1.0, drift: 0.0 },
    GaussianBelief { mean: 2.0, variance: 0.25, drift: 0.0 },
];

let fused = GaussianFusion::fuse(&beliefs);
println!("mean = {}, variance = {}", fused.mean, fused.variance);
```

## Modernization notes

- The library is kept intentionally compact and dependency-light.
- The stale example crates under `examples/` were not part of the active crate surface and still reference older external assumptions.
- The core library is now validated with real tests instead of relying on the build alone.

## Author

Neil Crago
