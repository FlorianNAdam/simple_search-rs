use crate::stateful::similarity::{Combination, Similarity};
use crate::stateful::state::SearchEngineState;
use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub struct SearchEngine<V, Q: Clone, S: Similarity<V, Q, State>, State: SearchEngineState<V>> {
    values: Vec<(State, V)>,
    similarity: S,
    phantom: PhantomData<Q>,
}
impl<Q: Clone, V> SearchEngine<V, Q, (), ()> {
    pub fn new<State: SearchEngineState<V>>() -> SearchEngine<V, Q, (), State> {
        SearchEngine {
            values: Vec::new(),
            similarity: (),
            phantom: Default::default(),
        }
    }
}

impl<'a, V, Q: Clone, S: Similarity<V, Q, State>, State: SearchEngineState<V>>
    SearchEngine<V, Q, S, State>
{
    pub fn add_value(&mut self, value: V) {
        self.values.push((State::new(&value), value));
    }

    pub fn add_values(&mut self, values: Vec<V>) {
        let values: Vec<_> = values.into_iter().map(|v| (State::new(&v), v)).collect();
        self.values.extend(values);
    }

    pub fn with_value(mut self, value: V) -> Self {
        self.values.push((State::new(&value), value));
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_values(mut self, values: Vec<V>) -> Self {
        let values: Vec<_> = values.into_iter().map(|v| (State::new(&v), v)).collect();
        self.values.extend(values);
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_key_fn_weighted<F: Fn(&mut State, &V, Q) -> f64>(
        self,
        weight: f64,
        function: F,
    ) -> SearchEngine<V, Q, Combination<V, Q, F, S, State>, State> {
        let similarity = self.similarity.combine_weighted(weight, function);
        SearchEngine {
            values: self.values,
            similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_key_fn<F: Fn(&mut State, &V, Q) -> f64>(
        self,
        function: F,
    ) -> SearchEngine<V, Q, Combination<V, Q, F, S, State>, State> {
        self.with_key_fn_weighted(1., function)
    }

    pub fn similarities(&mut self, query: Q) -> Vec<(&V, f64)> {
        let mut values = self
            .values
            .iter_mut()
            .map(|(state, v)| (v as &V, self.similarity.similarity(state, v, query.clone())))
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn search(&mut self, query: Q) -> Vec<&V> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }

    pub fn into_similarities(self, query: Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_iter()
            .map(|(mut state, v)| {
                let similarity = self.similarity.similarity(&mut state, &v, query.clone());
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
impl<Q, V, S: Similarity<V, Q, State>, State> SearchEngine<V, Q, S, State>
where
    Q: Clone + Send + Sync,
    V: Send + Sync,
    S: Clone + Send + Sync,
    State: SearchEngineState<V> + Send + Sync,
{
    pub fn par_similarities(&mut self, query: Q) -> Vec<(&V, f64)> {
        let mut values = self
            .values
            .par_iter_mut()
            .map(|(state, v)| {
                (
                    v as &V,
                    self.similarity.clone().similarity(state, v, query.clone()),
                )
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn par_search(&mut self, query: Q) -> Vec<&V> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }

    pub fn into_par_similarities(self, query: Q) -> Vec<(V, f64)> {
        let mut values = self
            .values
            .into_par_iter()
            .map(|(mut state, v)| {
                let similarity = self
                    .similarity
                    .clone()
                    .similarity(&mut state, &v, query.clone());
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
