//! This module defines the `SearchEngineState` trait and its implementations.
//! It allows for the creation of state representations for different value types
//! in a search engine context. The state can be used to facilitate efficient
//! search operations, such as incremental search where the state evolves as the
//! search query changes.

use crate::levenshtein::incremental::IncrementalLevenshtein;

/// A trait that defines how to create a state representation from a given value.
pub trait SearchEngineState<V> {
    /// Creates a new state representation for a given value.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the value from which to create the state.
    ///
    /// # Returns
    ///
    /// Returns an instance of the implementing type that represents the state of `value`.
    fn new(value: &V) -> Self;
}

impl SearchEngineState<&str> for IncrementalLevenshtein {
    fn new(value: &&str) -> Self {
        IncrementalLevenshtein::new("", value)
    }
}

impl SearchEngineState<String> for IncrementalLevenshtein {
    fn new(value: &String) -> Self {
        IncrementalLevenshtein::new("", value)
    }
}

impl<V> SearchEngineState<V> for () {
    fn new(_value: &V) -> Self {
        ()
    }
}
