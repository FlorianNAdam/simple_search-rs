use crate::stateless::similarity::{Combination, Similarity};
use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub struct SearchEngine<V, Q: Clone, S: Similarity<V, Q>> {
    values: Vec<V>,
    similarity: S,
    phantom: PhantomData<Q>,
}

impl<Q: Clone, V> SearchEngine<V, Q, ()> {
    pub fn new() -> SearchEngine<V, Q, ()> {
        SearchEngine {
            values: Vec::new(),
            similarity: (),
            phantom: Default::default(),
        }
    }
}

impl<V, Q: Clone, S: Similarity<V, Q>> SearchEngine<V, Q, S> {
    pub fn add_value(&mut self, value: V) {
        self.values.push(value);
    }

    pub fn add_values(&mut self, values: Vec<V>) {
        self.values.extend(values);
    }

    pub fn with_value(mut self, value: V) -> Self {
        self.values.push(value);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_values(mut self, values: Vec<V>) -> Self {
        self.values.extend(values);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_key_fn_weighted<F: Fn(&V, Q) -> f64>(
        self,
        weight: f64,
        function: F,
    ) -> SearchEngine<V, Q, Combination<V, Q, F, S>> {
        let similarity = self.similarity.combine_weighted(weight, function);
        SearchEngine {
            values: self.values,
            similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_key_fn<F: Fn(&V, Q) -> f64>(
        self,
        function: F,
    ) -> SearchEngine<V, Q, Combination<V, Q, F, S>> {
        self.with_key_fn_weighted(1., function)
    }

    pub fn similarities(&self, query: Q) -> Vec<(&V, f64)> {
        let mut values = self
            .values
            .iter()
            .map(|v| (v, self.similarity.similarity(&v, query.clone())))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn search(&self, query: Q) -> Vec<&V> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }

    pub fn into_similarities(self, query: Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_iter()
            .map(|v| {
                let similarity = self.similarity.similarity(&v, query.clone());
                (v, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn into_search(self, query: Q) -> Vec<V> {
        self.into_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<Q, V, S: Similarity<V, Q>> SearchEngine<V, Q, S>
where
    Q: Clone + Send + Sync,
    V: Send + Sync,
    S: Clone + Send + Sync,
{
    pub fn par_similarities(&self, query: Q) -> Vec<(&V, f64)> {
        let mut values = self
            .values
            .par_iter()
            .map(|v| (v, self.similarity.clone().similarity(&v, query.clone())))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn par_search(&self, query: Q) -> Vec<&V> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }

    pub fn into_par_similarities(self, query: Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_par_iter()
            .map(|v| {
                let similarity = self.similarity.clone().similarity(&v, query.clone());
                (v, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn into_par_search(self, query: Q) -> Vec<V> {
        self.into_par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}
