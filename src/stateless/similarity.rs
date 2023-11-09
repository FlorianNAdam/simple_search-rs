use std::marker::PhantomData;

pub struct Combination<'a, V, Q: ?Sized, F: FnMut(&V, &Q) -> f64, S: Similarity<'a, V, Q>> {
    weight: f64,
    function: F,
    inner: S,
    phantom: PhantomData<(V, &'a Q)>,
}

pub trait Similarity<'a, V, Q: ?Sized> {
    fn similarity<'b>(&self, value: &V, query: &'b Q) -> f64;

    fn combine<F: Fn(&V, &Q) -> f64>(self, function: F) -> Combination<'a, V, Q, F, Self>
    where
        Self: Sized,
    {
        self.combine_weighted(1., function)
    }

    fn combine_weighted<F: Fn(&V, &Q) -> f64>(
        self,
        weight: f64,
        function: F,
    ) -> Combination<'a, V, Q, F, Self>
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

impl<'a, V, Q: ?Sized, F: Fn(&V, &Q) -> f64 + Clone, S: Similarity<'a, V, Q> + Clone> Clone
    for Combination<'a, V, Q, F, S>
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

impl<'a, V, Q: ?Sized> Similarity<'a, V, Q> for () {
    fn similarity<'b>(&self, _value: &V, _query: &'b Q) -> f64 {
        0.
    }
}

impl<'a, V, Q: ?Sized, F: Fn(&V, &Q) -> f64, S: Similarity<'a, V, Q>> Similarity<'a, V, Q>
    for Combination<'a, V, Q, F, S>
{
    fn similarity<'b>(&self, value: &V, query: &'b Q) -> f64 {
        self.inner
            .similarity(value, query)
            .max(self.weight * (self.function)(value, query))
    }
}
