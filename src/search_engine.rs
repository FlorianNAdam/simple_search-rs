//! This module provides a generic `SearchEngine` struct for building a search engine using the builder pattern.

use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::similarity::{Similarity, StatefulCombination, StatelessCombination};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Marker trait for search engine mutability.
/// Only implemented by [Mutable] and [Immutable].
/// This Trait is used internally to allow a stateless engine being used immutably.
pub trait Mutability {}

/// Marker struct for search engines requiring mutable access due to being stateful.
pub struct Mutable;

/// Marker struct for search engines not requiring mutable access due to being stateless.
pub struct Immutable;

impl Mutability for Mutable {}

impl Mutability for Immutable {}

/// A generic struct for performing similarity-based searches.
///
/// This `SearchEngine` allows for building a search engine using the builder pattern.
///
/// Multiple similarity functions can be combined using the builder pattern. \
/// The similarity is determined by the maximum of the individual similarities weighted by the weight of the respective function. \
/// Similarity functions can be stateless or stateful.
///
pub struct SearchEngine<Value, Query: ?Sized, S, M: Mutability>
where
    S: Similarity<Value, Query>,
{
    values: Vec<(S::State, Value)>,
    similarity: S,
    phantom: PhantomData<(M, Query)>,
}

impl<Value, Query: ?Sized> SearchEngine<Value, Query, (), Immutable> {
    /// Creates a new `SearchEngine` with no values and no similarity functions.\
    /// The similarity defaults to just being 0.0 for all values.
    pub fn new() -> SearchEngine<Value, Query, (), Immutable> {
        SearchEngine {
            values: Vec::new(),
            similarity: (),
            phantom: Default::default(),
        }
    }
}

impl<Value, Query: ?Sized, S, M: Mutability> SearchEngine<Value, Query, S, M>
where
    S: Similarity<Value, Query>,
{
    /// Adds a single value to the search engine.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the search engine.
    pub fn add_value(&mut self, value: Value) {
        self.values.push((self.similarity.state(&value), value));
    }

    /// Adds multiple values to the search engine.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values to be added to the search engine.
    pub fn add_values(&mut self, values: Vec<Value>) {
        let values: Vec<_> = values
            .into_iter()
            .map(|v| (self.similarity.state(&v), v))
            .collect();
        self.values.extend(values);
    }

    /// Adds a single value to the search engine with the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the search engine.
    pub fn with_value(mut self, value: Value) -> Self {
        self.values.push((self.similarity.state(&value), value));
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
    pub fn with_values(mut self, values: Vec<Value>) -> Self {
        let values: Vec<_> = values
            .into_iter()
            .map(|v| (self.similarity.state(&v), v))
            .collect();
        self.values.extend(values);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    /// Adds a key function to use for determining the similarity of a value to the query.
    /// This is identical to `with_weight` with a weight of 1.0.
    ///
    /// # Arguments
    ///
    /// * `function` - A function for determining the similarity between a value and the query.
    pub fn with<Func>(
        self,
        function: Func,
    ) -> SearchEngine<Value, Query, StatelessCombination<Value, Query, S, Func>, M>
    where
        Func: Fn(&Value, &Query) -> f64,
    {
        self.with_weight(1., function)
    }

    /// Adds a weighted function to use for determining the similarity of a value to the query.
    ///
    /// # Arguments
    ///
    /// * `weight` - The weight of the similarity function.
    /// * `function` - A function for determining the similarity between a value and the query.
    pub fn with_weight<Func>(
        self,
        weight: f64,
        function: Func,
    ) -> SearchEngine<Value, Query, StatelessCombination<Value, Query, S, Func>, M>
    where
        Func: Fn(&Value, &Query) -> f64,
    {
        let similarity = self.similarity.with_weight(weight, function);
        SearchEngine {
            values: self.values,
            similarity,
            phantom: Default::default(),
        }
    }

    /// Adds a stateful function to use for determining the similarity of a value to the query.
    /// This is identical to `with_state_and_weight` with a weight of 1.0.
    ///
    /// # Arguments
    ///
    /// * `state_function` - A function for creating the state for a value.
    /// * `function` - A function for determining the similarity between a value and the query.
    pub fn with_state<Func, StateFunc, State>(
        self,
        state_func: StateFunc,
        function: Func,
    ) -> SearchEngine<
        Value,
        Query,
        StatefulCombination<Value, Query, S, Func, StateFunc, State>,
        Mutable,
    >
    where
        Func: Fn(&mut State, &Value, &Query) -> f64,
        StateFunc: Fn(&Value) -> State,
    {
        self.with_state_and_weight(1., state_func, function)
    }

    /// Adds a stateful and weighted function to use for determining the similarity of a value to the query.
    ///
    /// # Arguments
    ///
    /// * `weight` - The weight of the similarity function.
    /// * `state_function` - A function for creating the state for a value.
    /// * `function` - A function for determining the similarity between a value and the query.
    pub fn with_state_and_weight<Func, StateFunc, State>(
        self,
        weight: f64,
        state_function: StateFunc,
        function: Func,
    ) -> SearchEngine<
        Value,
        Query,
        StatefulCombination<Value, Query, S, Func, StateFunc, State>,
        Mutable,
    >
    where
        Func: Fn(&mut State, &Value, &Query) -> f64,
        StateFunc: Fn(&Value) -> State,
    {
        let similarity = self
            .similarity
            .with_state_and_weight(weight, function, state_function);
        let values: Vec<_> = self
            .values
            .into_iter()
            .map(|(_, value)| (similarity.state(&value), value))
            .collect();
        SearchEngine {
            values,
            similarity,
            phantom: Default::default(),
        }
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
    pub fn into_similarities(self, query: &Query) -> Vec<(Value, f64)> {
        let mut values = self
            .values
            .into_iter()
            .map(|(mut state, value)| {
                let similarity = self.similarity.similarity(&mut state, &value, query);
                (value, similarity)
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
    pub fn into_search(self, query: &Query) -> Vec<Value> {
        self.into_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }

    #[doc(hidden)]
    pub fn get_values_with_state(&self) -> &[(<S as Similarity<Value, Query>>::State, Value)] {
        &self.values
    }
}
impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Mutable>
where
    S: Similarity<Value, Query>,
{
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
    pub fn similarities(&mut self, query: &Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .iter_mut()
            .map(|(state, value)| {
                (
                    value as &Value,
                    self.similarity.similarity(state, value, query),
                )
            })
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
    pub fn search(&mut self, query: &Query) -> Vec<&Value> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }
}

impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Immutable>
where
    S: Similarity<Value, Query, State = ()>,
{
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
    pub fn similarities(&self, query: &Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .iter()
            .map(|(_, value)| (value, self.similarity.similarity(&mut (), value, query)))
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
    pub fn search(&self, query: &Query) -> Vec<&Value> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }
}

impl<Value, Query: ?Sized, S, M: Mutability> Clone for SearchEngine<Value, Query, S, M>
where
    Value: Clone,
    S::State: Clone,
    S: Similarity<Value, Query> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            similarity: self.similarity.clone(),
            phantom: Default::default(),
        }
    }
}

#[cfg(feature = "rayon")]
impl<Value, Query: ?Sized, S, M: Mutability> SearchEngine<Value, Query, S, M>
where
    Value: Send + Sync,
    Query: Send + Sync,
    S: Similarity<Value, Query> + Send + Sync,
    S::State: Send + Sync,
{
    /// Retrieves a sorted vector of tuples containing the values and their similarity scores
    /// to the given query. This is the parallelized version of [into_similarities](SearchEngine::into_similarities).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to rank the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples where the first element is a reference to a value and the second element
    /// is its similarity score as a floating-point number.
    pub fn into_par_similarities(self, query: &Query) -> Vec<(Value, f64)> {
        let mut values = self
            .values
            .into_par_iter()
            .map(|(mut state, value)| {
                let similarity = self.similarity.similarity(&mut state, &value, query);
                (value, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    /// Performs a parallel search based on the given query and returns a vector of the values ranked
    /// by similarity. This is the parallelized version of [search](SearchEngine::search).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to search the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of references to the values ranked by their similarity to the query.
    pub fn into_par_search(self, query: &Query) -> Vec<Value> {
        self.into_par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Mutable>
where
    Value: Send + Sync,
    Query: Send + Sync,
    S: Similarity<Value, Query> + Send + Sync,
    S::State: Send + Sync,
{
    /// Retrieves a sorted vector of tuples containing references to the values and their similarity scores
    /// to the given query. This is the parallelized version of [similarities](SearchEngine::similarities).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to rank the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples where the first element is a reference to a value and the second element
    /// is its similarity score as a floating-point number.
    pub fn par_similarities(&mut self, query: &Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .par_iter_mut()
            .map(|(state, value)| {
                (
                    value as &Value,
                    self.similarity.similarity(state, value, query),
                )
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    /// Performs a parallelized search based on the given query and returns a vector of the values ranked
    /// by similarity. This is the parallelized version of [search](SearchEngine::search).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to search the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of references to the values ranked by their similarity to the query.
    pub fn par_search(&mut self, query: &Query) -> Vec<&Value> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Immutable>
where
    Value: Send + Sync,
    Query: Send + Sync,
    S: Similarity<Value, Query, State = ()> + Send + Sync,
    S::State: Send + Sync,
{
    /// Retrieves a sorted vector of tuples containing references to the values and their similarity scores
    /// to the given query. This is the parallelized version of [similarities](SearchEngine::similarities).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to rank the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples where the first element is a reference to a value and the second element
    /// is its similarity score as a floating-point number.
    pub fn par_similarities(&self, query: &Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .par_iter()
            .map(|(_, value)| (value, self.similarity.similarity(&mut (), value, query)))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    /// Performs a parallelized search based on the given query and returns a vector of the values ranked
    /// by similarity. This is the parallelized version of [search](SearchEngine::search).
    ///
    /// # Arguments
    ///
    /// * `query` - The query against which to search the values.
    ///
    /// # Returns
    ///
    /// Returns a vector of references to the values ranked by their similarity to the query.
    pub fn par_search(&self, query: &Query) -> Vec<&Value> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}
