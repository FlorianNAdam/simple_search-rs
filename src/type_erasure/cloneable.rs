//! This module provides a band-aid solution for storing cloneable engines with erased types.

use crate::search_engine::{Immutable, Mutable, SearchEngine};
use crate::similarity::Similarity;

impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Immutable>
where
    Value: 'static,
    Query: 'static,
    S: Similarity<Value, Query, State = ()> + 'static,
    Self: Clone,
{
    pub fn erase_type_cloneable(self) -> ImmutableSearchEngine<Value, Query> {
        ImmutableSearchEngine {
            engine: Box::new(self),
        }
    }
}

impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Mutable>
where
    Value: 'static,
    Query: 'static,
    S: Similarity<Value, Query> + 'static,
    Self: Clone,
{
    pub fn erase_type_cloneable(self) -> MutableSearchEngine<Value, Query> {
        MutableSearchEngine {
            engine: Box::new(self),
        }
    }
}

trait ImmutableSearchEngineCloneTrait<Value, Query: ?Sized> {
    fn clone_box(&self) -> Box<dyn ImmutableSearchEngineTrait<Value, Query>>;
}

trait MutableSearchEngineCloneTrait<Value, Query: ?Sized> {
    fn clone_box(&self) -> Box<dyn MutableSearchEngineTrait<Value, Query>>;
}

trait ImmutableSearchEngineTrait<Value, Query: ?Sized>:
    ImmutableSearchEngineCloneTrait<Value, Query>
{
    fn similarities_wrapper(&self, query: &Query) -> Vec<(&Value, f64)>;

    fn search_wrapper(&self, query: &Query) -> Vec<&Value>;
}
trait MutableSearchEngineTrait<Value, Query: ?Sized>:
    MutableSearchEngineCloneTrait<Value, Query>
{
    fn similarities_wrapper(&mut self, query: &Query) -> Vec<(&Value, f64)>;

    fn search_wrapper(&mut self, query: &Query) -> Vec<&Value>;
}

impl<Value, Query: ?Sized, S> ImmutableSearchEngineCloneTrait<Value, Query>
    for SearchEngine<Value, Query, S, Immutable>
where
    Value: 'static,
    Query: 'static,
    S: 'static,
    S: Similarity<Value, Query, State = ()>,
    Self: Clone,
{
    fn clone_box(&self) -> Box<dyn ImmutableSearchEngineTrait<Value, Query>> {
        Box::new(self.clone())
    }
}
impl<Value, Query: ?Sized, S> ImmutableSearchEngineTrait<Value, Query>
    for SearchEngine<Value, Query, S, Immutable>
where
    Value: 'static,
    Query: 'static,
    S: 'static,
    S: Similarity<Value, Query, State = ()>,
    Self: Clone,
{
    fn similarities_wrapper(&self, query: &Query) -> Vec<(&Value, f64)> {
        self.similarities(query)
    }

    fn search_wrapper(&self, query: &Query) -> Vec<&Value> {
        <SearchEngine<Value, Query, S, Immutable>>::search(self, query)
    }
}

impl<Value, Query: ?Sized, S> MutableSearchEngineCloneTrait<Value, Query>
    for SearchEngine<Value, Query, S, Mutable>
where
    Value: 'static,
    Query: 'static,
    S: 'static,
    S: Similarity<Value, Query>,
    Self: Clone,
{
    fn clone_box(&self) -> Box<dyn MutableSearchEngineTrait<Value, Query>> {
        Box::new(self.clone())
    }
}

impl<Value, Query: ?Sized, S> MutableSearchEngineTrait<Value, Query>
    for SearchEngine<Value, Query, S, Mutable>
where
    Value: 'static,
    Query: 'static,
    S: 'static,
    S: Similarity<Value, Query>,
    Self: Clone,
{
    fn similarities_wrapper(&mut self, query: &Query) -> Vec<(&Value, f64)> {
        self.similarities(query)
    }

    fn search_wrapper(&mut self, query: &Query) -> Vec<&Value> {
        self.search(query)
    }
}

/// Wrapper struct for type erased search engines not requiring mutable access due to being stateless.
pub struct ImmutableSearchEngine<Value, Query: ?Sized> {
    engine: Box<dyn ImmutableSearchEngineTrait<Value, Query>>,
}

impl<Value, Query: ?Sized> ImmutableSearchEngine<Value, Query> {
    pub fn similarities(&self, query: &Query) -> Vec<(&Value, f64)> {
        self.engine.similarities_wrapper(query)
    }

    pub fn search(&self, query: &Query) -> Vec<&Value> {
        self.engine.search_wrapper(query)
    }
}

/// Wrapper struct for type erased search engines requiring mutable access due to being stateful.
pub struct MutableSearchEngine<Value, Query: ?Sized> {
    engine: Box<dyn MutableSearchEngineTrait<Value, Query>>,
}

impl<Value, Query: ?Sized> MutableSearchEngine<Value, Query> {
    pub fn similarities(&mut self, query: &Query) -> Vec<(&Value, f64)> {
        self.engine.similarities_wrapper(query)
    }

    pub fn search(&mut self, query: &Query) -> Vec<&Value> {
        self.engine.search_wrapper(query)
    }
}

impl<Value, Query: ?Sized> Clone for MutableSearchEngine<Value, Query> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone_box(),
        }
    }
}

impl<Value, Query: ?Sized> Clone for ImmutableSearchEngine<Value, Query> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone_box(),
        }
    }
}
