// Indicator traits
//
pub trait Reset {
    fn reset(&mut self);
}

pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}


// Bar traits
//
pub trait Open {
    fn open(&self) -> f64;
}

pub trait Close {
    fn close(&self) -> f64;
}
