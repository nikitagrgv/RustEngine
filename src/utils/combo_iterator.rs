pub struct ComboIterator0<T0, T1>
where
    T0: Iterator,
    T1: Iterator,
{
    it0: T0,
    it1: T1,
}

impl<T0, T1> ComboIterator0<T0, T1>
where
    T0: Iterator,
    T1: Iterator,
{
    pub fn new(it0: T0, it1: T1) -> Self {
        Self { it0, it1 }
    }
}

impl<T0, T1> Iterator for ComboIterator0<T0, T1>
where
    T0: Iterator,
    T1: Iterator,
{
    type Item = (T0::Item, T1::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let next0 = self.it0.next();
        let next1 = self.it1.next();
        match (next0, next1) {
            (Some(v0), Some(v1)) => Some((v0, v1)),
            _ => None,
        }
    }
}
