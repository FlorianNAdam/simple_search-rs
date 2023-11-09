//! This module provides a generic `SearchEngine` struct for building a stateless search engine using the builder pattern.

use crate::stateless::similarity::{Combination, Similarity};
use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// A generic struct for performing similarity-based searches.
/// This `SearchEngine` allows for building a stateless search engine using the builder pattern.
pub struct SearchEngine<'a, V, Q: ?Sized, S: Similarity<'a, V, Q>> {
    values: Vec<V>,
    similarity: S,
    phantom: PhantomData<&'a Q>,
}

impl<'a, Q: ?Sized, V> SearchEngine<'a, V, Q, ()> {
    pub fn new() -> SearchEngine<'a, V, Q, ()> {
        SearchEngine {
            values: Vec::new(),
            similarity: (),
            phantom: Default::default(),
        }
    }
}

impl<'a, V, Q: ?Sized, S: Similarity<'a, V, Q>> SearchEngine<'a, V, Q, S> {
    /// Adds a single value to the search engine.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the search engine.
    pub fn add_value(&mut self, value: V) {
        self.values.push(value);
    }

    /// Adds multiple values to the search engine.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values to be added to the search engine.
    pub fn add_values(&mut self, values: Vec<V>) {
        self.values.extend(values);
    }

    /// Adds a single value to the search engine with the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the search engine.
    pub fn with_value(mut self, value: V) -> Self {
        self.values.push(value);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    /// Adds multiple values to the search engine with the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values to be added to the search engine.
    pub fn with_values(mut self, values: Vec<V>) -> Self {
        self.values.extend(values);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    /// Adds a key function to use for determining the similarity of a value to the query.
    /// The max value of the all applied functions weighted with their weights will be used as the similarity score.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values to be added to the search engine.
    pub fn with_key_fn_weighted<F: Fn(&V, &Q) -> f64>(
        self,
        weight: f64,
        function: F,
    ) -> SearchEngine<'a, V, Q, Combination<'a, V, Q, F, S>> {
        let similarity = self.similarity.combine_weighted(weight, function);
        SearchEngine {
            values: self.values,
            similarity,
            phantom: Default::default(),
        }
    }

    /// Adds a key function to use for determining the similarity of a value to the query.
    /// This is identical to `with_key_fn_weighted` with a weight of 1.0.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values to be added to the search engine.
    pub fn with_key_fn<F: Fn(&V, &Q) -> f64>(
        self,
        function: F,
    ) -> SearchEngine<'a, V, Q, Combination<'a, V, Q, F, S>> {
        self.with_key_fn_weighted(1., function)
    }

    /// Retrieves a sorted vector of tuples containing references to the values and their similarity scores
    /// to the given query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to rank the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples where the first element is a reference to a value and the second element
    /// is its similarity score as a floating-point number.
    pub fn similarities<'b>(&'a self, query: &'b Q) -> Vec<(&'a V, f64)> {
        let mut values = self
            .values
            .iter()
            .map(|v| (v, self.similarity.similarity(&v, query)))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    /// Performs a search based on the given query and returns a vector of references to the values ranked
    /// by similarity.
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to search the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of references to the values ranked by their similarity to the query.
    pub fn search<'b>(&'a self, query: &'b Q) -> Vec<&'a V> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }

    /// Retrieves a sorted vector of tuples containing the values and their similarity scores
    /// to the given query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to rank the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples where the first element is a reference to a value and the second element
    /// is its similarity score as a floating-point number.
    pub fn into_similarities(self, query: &'a Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_iter()
            .map(|v| {
                let similarity = self.similarity.similarity(&v, query);
                (v, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    /// Performs a search based on the given query and returns a vector of the values ranked
    /// by similarity.
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to search the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of references to the values ranked by their similarity to the query.
    pub fn into_search(self, query: &'a Q) -> Vec<V> {
        self.into_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<'a, Q, V, S: Similarity<'a, V, Q>> SearchEngine<'a, V, Q, S>
where
    Q: Send + Sync,
    V: Send + Sync,
    S: Clone + Send + Sync,
{
    pub fn par_similarities(&self, query: &'a Q) -> Vec<(&V, f64)> {
        let mut values = self
            .values
            .par_iter()
            .map(|v| (v, self.similarity.clone().similarity(&v, query)))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn par_search(&self, query: &'a Q) -> Vec<&V> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }

    pub fn into_par_similarities(self, query: &'a Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_par_iter()
            .map(|v| {
                let similarity = self.similarity.clone().similarity(&v, query);
                (v, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn into_par_search(self, query: &'a Q) -> Vec<V> {
        self.into_par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}
