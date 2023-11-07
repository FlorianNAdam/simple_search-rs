use crate::levenshtein::incremental::IncrementalLevenshtein;

pub trait SearchEngineState<V> {
    fn new(value: &V) -> Self;
}

impl SearchEngineState<&str> for IncrementalLevenshtein {
    fn new(value: &&str) -> Self {
        IncrementalLevenshtein::new("", value)
    }
}

impl SearchEngineState<String> for IncrementalLevenshtein {
    fn new(value: &String) -> Self {
        IncrementalLevenshtein::new("", value)
    }
}

impl<V> SearchEngineState<V> for () {
    fn new(_value: &V) -> Self {
        ()
    }
}
