use std::marker::PhantomData;

pub struct Combination<
    V,
    Q: Clone,
    F: FnMut(&mut State, &V, Q) -> f64,
    S: Similarity<V, Q, State>,
    State,
> {
    weight: f64,
    function: F,
    inner: S,
    phantom: PhantomData<(V, Q, State)>,
}

pub trait Similarity<Value, Query: Clone, State> {
    fn similarity(&self, state: &mut State, value: &Value, query: Query) -> f64;

    fn combine<F: Fn(&mut State, &Value, Query) -> f64>(
        self,
        function: F,
    ) -> Combination<Value, Query, F, Self, State>
    where
        Self: Sized,
    {
        self.combine_weighted(1., function)
    }

    fn combine_weighted<F: Fn(&mut State, &Value, Query) -> f64>(
        self,
        weight: f64,
        function: F,
    ) -> Combination<Value, Query, F, Self, State>
    where
        Self: Sized,
    {
        Combination {
            weight,
            function,
            inner: self,
            phantom: Default::default(),
        }
    }
}

impl<
        V,
        Q: Clone,
        F: Fn(&mut State, &V, Q) -> f64 + Clone,
        S: Similarity<V, Q, State> + Clone,
        State,
    > Clone for Combination<V, Q, F, S, State>
{
    fn clone(&self) -> Self {
        Self {
            weight: self.weight.clone(),
            function: self.function.clone(),
            inner: self.inner.clone(),
            phantom: Default::default(),
        }
    }
}

impl<V, Q: Clone, State> Similarity<V, Q, State> for () {
    fn similarity(&self, _state: &mut State, _value: &V, _query: Q) -> f64 {
        0.
    }
}

impl<V, Q: Clone, F: Fn(&mut State, &V, Q) -> f64, S: Similarity<V, Q, State>, State>
    Similarity<V, Q, State> for Combination<V, Q, F, S, State>
{
    fn similarity(&self, state: &mut State, value: &V, query: Q) -> f64 {
        self.inner
            .similarity(state, value, query.clone())
            .max(self.weight * (self.function)(state, value, query))
    }
}
