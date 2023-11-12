use crate::search_engine::{Immutable, Mutable, SearchEngine};
use crate::similarity::Similarity;

impl<Value, Query: ?Sized, S> SearchEngine<Value, Query, S, Immutable>
where
    Value: 'static,
    Query: 'static,
    S: Similarity<Value, Query, State = ()> + 'static,
{
    pub fn erase_type(self) -> ImmutableSearchEngine<Value, Query> {
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
{
    pub fn erase_type(self) -> MutableSearchEngine<Value, Query> {
        MutableSearchEngine {
            engine: Box::new(self),
        }
    }
}

trait ImmutableSearchEngineTrait<Value, Query: ?Sized> {
    fn similarities_wrapper(&self, query: &Query) -> Vec<(&Value, f64)>;

    fn search_wrapper(&self, query: &Query) -> Vec<&Value>;
}
trait MutableSearchEngineTrait<Value, Query: ?Sized> {
    fn similarities_wrapper(&mut self, query: &Query) -> Vec<(&Value, f64)>;

    fn search_wrapper(&mut self, query: &Query) -> Vec<&Value>;
}

impl<Value, Query: ?Sized, S> ImmutableSearchEngineTrait<Value, Query>
    for SearchEngine<Value, Query, S, Immutable>
where
    S: Similarity<Value, Query, State = ()>,
{
    fn similarities_wrapper(&self, query: &Query) -> Vec<(&Value, f64)> {
        self.similarities(query)
    }

    fn search_wrapper(&self, query: &Query) -> Vec<&Value> {
        <SearchEngine<Value, Query, S, Immutable>>::search(self, query)
    }
}

impl<Value, Query: ?Sized, S> MutableSearchEngineTrait<Value, Query>
    for SearchEngine<Value, Query, S, Mutable>
where
    S: Similarity<Value, Query>,
{
    fn similarities_wrapper(&mut self, query: &Query) -> Vec<(&Value, f64)> {
        self.similarities(query)
    }

    fn search_wrapper(&mut self, query: &Query) -> Vec<&Value> {
        self.search(query)
    }
}

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
