use crate::granular::state::SearchEngineState;
use std::marker::PhantomData;

pub struct StatelessCombination<Value, Query, Inner, Func>
where
    Query: Clone,
    Func: Fn(&Value, Query) -> f64,
    Inner: Similarity<Value, Query>,
{
    weight: f64,
    function: Func,
    inner: Inner,
    phantom: PhantomData<(Value, Query)>,
}
pub struct StatefulCombination<Value, Query, Inner, Func, State>
where
    Query: Clone,
    Func: Fn(&mut State, &Value, Query) -> f64,
    Inner: Similarity<Value, Query>,
{
    weight: f64,
    function: Func,
    inner: Inner,
    phantom: PhantomData<(Value, Query, State)>,
}

pub trait Similarity<Value, Query>
where
    Query: Clone,
{
    type State: SearchEngineState<Value>;

    fn similarity(&self, state: &mut Self::State, value: &Value, query: Query) -> f64;

    fn with<Func>(self, func: Func) -> StatelessCombination<Value, Query, Self, Func>
    where
        Func: Fn(&Value, Query) -> f64,
        Self: Sized,
    {
        self.with_weight(1., func)
    }

    fn with_weight<Func>(
        self,
        weight: f64,
        func: Func,
    ) -> StatelessCombination<Value, Query, Self, Func>
    where
        Func: Fn(&Value, Query) -> f64,
        Self: Sized,
    {
        StatelessCombination {
            weight,
            function: func,
            inner: self,
            phantom: Default::default(),
        }
    }

    fn with_state<State, Func>(
        self,
        func: Func,
    ) -> StatefulCombination<Value, Query, Self, Func, State>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        Self: Sized,
    {
        self.with_state_and_weight(1., func)
    }

    fn with_state_and_weight<State, Func>(
        self,
        weight: f64,
        func: Func,
    ) -> StatefulCombination<Value, Query, Self, Func, State>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        Self: Sized,
    {
        StatefulCombination {
            weight,
            function: func,
            inner: self,
            phantom: Default::default(),
        }
    }
}

impl<Value, Query> Similarity<Value, Query> for ()
where
    Query: Clone,
{
    type State = ();

    fn similarity(&self, _state: &mut Self::State, _value: &Value, _query: Query) -> f64 {
        0.
    }
}

impl<Value, Query, Inner, Func, State> Similarity<Value, Query>
    for StatefulCombination<Value, Query, Inner, Func, State>
where
    Query: Clone,
    Func: Fn(&mut State, &Value, Query) -> f64,
    Inner: Similarity<Value, Query>,
    State: SearchEngineState<Value>,
{
    type State = (State, Inner::State);

    fn similarity(&self, state: &mut Self::State, value: &Value, query: Query) -> f64 {
        let (state, inner_state) = (&mut state.0, &mut state.1);

        let similarity = (self.function)(state, value, query.clone()) * self.weight;
        let inner_similarity = self.inner.similarity(inner_state, value, query);

        similarity.max(inner_similarity)
    }
}

impl<Value, Query, Inner, Func> Similarity<Value, Query>
    for StatelessCombination<Value, Query, Inner, Func>
where
    Query: Clone,
    Func: Fn(&Value, Query) -> f64,
    Inner: Similarity<Value, Query>,
{
    type State = Inner::State;

    fn similarity(&self, state: &mut Self::State, value: &Value, query: Query) -> f64 {
        let similarity = (self.function)(value, query.clone()) * self.weight;
        let inner_similarity = self.inner.similarity(state, value, query);

        similarity.max(inner_similarity)
    }
}

impl<Value, State: SearchEngineState<Value>, InnerState: SearchEngineState<Value>>
    SearchEngineState<Value> for (State, InnerState)
{
    fn new(value: &Value) -> Self {
        (State::new(value), InnerState::new(value))
    }
}
