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

pub struct StatefulCombination<Value, Query, Inner, Func, StateFunc, State>
where
    Query: Clone,
    Func: Fn(&mut State, &Value, Query) -> f64,
    StateFunc: Fn(&Value) -> State,
    Inner: Similarity<Value, Query>,
{
    weight: f64,
    function: Func,
    state_func: StateFunc,
    inner: Inner,
    phantom: PhantomData<(Value, Query, State)>,
}

pub trait Similarity<Value, Query>
where
    Query: Clone,
{
    type State;

    fn state(&self, value: &Value) -> Self::State;

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

    fn with_state<State, Func, StateFunc>(
        self,
        func: Func,
        state_func: StateFunc,
    ) -> StatefulCombination<Value, Query, Self, Func, StateFunc, State>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        StateFunc: Fn(&Value) -> State,
        Self: Sized,
    {
        self.with_state_and_weight(1., func, state_func)
    }

    fn with_state_and_weight<State, Func, StateFunc>(
        self,
        weight: f64,
        func: Func,
        state_func: StateFunc,
    ) -> StatefulCombination<Value, Query, Self, Func, StateFunc, State>
    where
        Func: Fn(&mut State, &Value, Query) -> f64,
        StateFunc: Fn(&Value) -> State,
        Self: Sized,
    {
        StatefulCombination {
            weight,
            function: func,
            state_func,
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

    fn state(&self, _value: &Value) -> Self::State {
        ()
    }

    fn similarity(&self, _state: &mut Self::State, _value: &Value, _query: Query) -> f64 {
        0.
    }
}

impl<Value, Query, Inner, Func, StateFunc, State> Similarity<Value, Query>
    for StatefulCombination<Value, Query, Inner, Func, StateFunc, State>
where
    Query: Clone,
    Func: Fn(&mut State, &Value, Query) -> f64,
    StateFunc: Fn(&Value) -> State,
    Inner: Similarity<Value, Query>,
{
    type State = (State, Inner::State);

    fn state(&self, value: &Value) -> Self::State {
        ((self.state_func)(value), self.inner.state(value))
    }

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

    fn state(&self, value: &Value) -> Self::State {
        self.inner.state(value)
    }

    fn similarity(&self, state: &mut Self::State, value: &Value, query: Query) -> f64 {
        let similarity = (self.function)(value, query.clone()) * self.weight;
        let inner_similarity = self.inner.similarity(state, value, query);

        similarity.max(inner_similarity)
    }
}
