//! This module provides a band-aid solution for storing engines with erased types.
//! There will be a more elegant solution, once the approved [RFC 2515](https://rust-lang.github.io/impl-trait-initiative/RFC.html) is part of stable rust.
pub mod cloneable;
pub mod non_cloneable;
