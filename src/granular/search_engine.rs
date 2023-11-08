use crate::granular::similarity::{Similarity, StatefulCombination, StatelessCombination};
use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub struct SearchEngine<Value, Query, S>
where
    Query: Clone,
    S: Similarity<Value, Query>,
{
    values: Vec<(S::State, Value)>,
    similarity: S,
    phantom: PhantomData<Query>,
}

impl<Value, Query> SearchEngine<Value, Query, ()>
where
    Query: Clone,
{
    pub fn new() -> SearchEngine<Value, Query, ()> {
        SearchEngine {
            values: Vec::new(),
            similarity: (),
            phantom: Default::default(),
        }
    }
}

impl<Value, Query, S> SearchEngine<Value, Query, S>
where
    Query: Clone,
    S: Similarity<Value, Query>,
{
    pub fn add_value(&mut self, value: Value) {
        self.values.push((self.similarity.state(&value), value));
    }

    pub fn add_values(&mut self, values: Vec<Value>) {
        let values: Vec<_> = values
            .into_iter()
            .map(|v| (self.similarity.state(&v), v))
            .collect();
        self.values.extend(values);
    }

    pub fn with_value(mut self, value: Value) -> Self {
        self.values.push((self.similarity.state(&value), value));
        Self {
            values: self.values,
            similarity: self.similarity,
            phantom: Default::default(),
        }
    }

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

    pub fn with<Func>(
        self,
        function: Func,
    ) -> SearchEngine<Value, Query, StatelessCombination<Value, Query, S, Func>>
    where
        Func: Fn(&Value, Query) -> f64,
    {
        self.with_weight(1., function)
    }

    pub fn with_weight<Func>(
        self,
        weight: f64,
        function: Func,
    ) -> SearchEngine<Value, Query, StatelessCombination<Value, Query, S, Func>>
    where
        Func: Fn(&Value, Query) -> f64,
    {
        let similarity = self.similarity.with_weight(weight, function);
        SearchEngine {
            values: self.values,
            similarity,
            phantom: Default::default(),
        }
    }

    pub fn with_state<Func, StateFunc, State>(
        self,
        state_func: StateFunc,
        function: Func,
    ) -> SearchEngine<Value, Query, StatefulCombination<Value, Query, S, Func, StateFunc, State>>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        StateFunc: Fn(&Value) -> State,
    {
        self.with_state_and_weight(1., state_func, function)
    }

    pub fn with_state_and_weight<Func, StateFunc, State>(
        self,
        weight: f64,
        state_function: StateFunc,
        function: Func,
    ) -> SearchEngine<Value, Query, StatefulCombination<Value, Query, S, Func, StateFunc, State>>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        StateFunc: Fn(&Value) -> State,
    {
        let similarity = self
            .similarity
            .with_state_and_weight(weight, function, state_function);
        let values: Vec<_> = self
            .values
            .into_iter()
            .map(|(state, value)| (similarity.state(&value), value))
            .collect();
        SearchEngine {
            values,
            similarity,
            phantom: Default::default(),
        }
    }

    pub fn similarities(&mut self, query: Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .iter_mut()
            .map(|(state, value)| {
                (
                    value as &Value,
                    self.similarity.similarity(state, value, query.clone()),
                )
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn search(&mut self, query: Query) -> Vec<&Value> {
        self.similarities(query).into_iter().map(|v| v.0).collect()
    }

    pub fn into_similarities(self, query: Query) -> Vec<(Value, f64)> {
        let mut values = self
            .values
            .into_iter()
            .map(|(mut state, value)| {
                let similarity = self
                    .similarity
                    .similarity(&mut state, &value, query.clone());
                (value, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn into_search(self, query: Query) -> Vec<Value> {
        self.into_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<Value, Query, S> SearchEngine<Value, Query, S>
where
    Value: Send + Sync,
    Query: Clone + Send + Sync,
    S: Similarity<Value, Query> + Send + Sync,
    S::State: Send + Sync,
{
    pub fn par_similarities(&mut self, query: Query) -> Vec<(&Value, f64)> {
        let mut values = self
            .values
            .par_iter_mut()
            .map(|(state, value)| {
                (
                    value as &Value,
                    self.similarity.similarity(state, value, query.clone()),
                )
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn par_search(&mut self, query: Query) -> Vec<&Value> {
        self.par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }

    pub fn into_par_similarities(self, query: Query) -> Vec<(Value, f64)> {
        let mut values = self
            .values
            .into_par_iter()
            .map(|(mut state, value)| {
                let similarity = self
                    .similarity
                    .similarity(&mut state, &value, query.clone());
                (value, similarity)
            })
            .collect::<Vec<_>>();
        values.sort_unstable_by(|(_, v), (_, s)| v.partial_cmp(s).unwrap_or(Ordering::Equal));
        values
    }

    pub fn into_par_search(self, query: Query) -> Vec<Value> {
        self.into_par_similarities(query)
            .into_iter()
            .map(|v| v.0)
            .collect()
    }
}
