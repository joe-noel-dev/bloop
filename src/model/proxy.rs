pub struct Proxy<T, U>
where
    T: Clone,
    U: Fn(&T) -> (),
{
    object: T,
    on_change: U,
}

impl<T, U> Proxy<T, U>
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

    pub fn get(&self) -> T {
        self.object.clone()
    }

    pub fn set(&mut self, object: T) {
        if self.object != object {
            self.object = object;
            (self.on_change)(&self.object);
        }
    }
}
