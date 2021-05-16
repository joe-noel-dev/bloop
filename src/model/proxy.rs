pub trait Proxy<T> {
    fn get(&self) -> T;
    fn set(&mut self, object: T);
}

pub struct NotifyingProxy<T, U>
where
    T: Clone,
    U: Fn(&T) -> (),
{
    object: T,
    on_change: U,
}

impl<T, U> NotifyingProxy<T, U>
where
    T: Clone + PartialEq,
    U: Fn(&T) -> (),
{
    pub fn new(initial_value: T, on_change: U) -> Self {
        Self {
            object: initial_value,
            on_change,
        }
    }
}

impl<T, U> Proxy<T> for NotifyingProxy<T, U>
where
    T: Clone + PartialEq,
    U: Fn(&T) -> (),
{
    fn get(&self) -> T {
        self.object.clone()
    }

    fn set(&mut self, object: T) {
        if self.object != object {
            self.object = object;
            (self.on_change)(&self.object);
        }
    }
}
