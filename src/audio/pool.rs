use crate::model::id::ID;
use std::collections::HashMap;

pub struct Pool<T> {
    items: HashMap<ID, Box<T>>,
}

const MAX_ITEMS: usize = 128;

impl<T> Default for Pool<T> {
    fn default() -> Self {
        let mut items = HashMap::new();
        items.reserve(MAX_ITEMS);
        Self { items }
    }
}

impl<T> Pool<T> {
    pub fn get(&self, id: &ID) -> Option<&T> {
        self.items.get(id).map(|value| value.as_ref())
    }

    pub fn get_mut(&mut self, id: &ID) -> Option<&mut T> {
        self.items.get_mut(id).map(|value| value.as_mut())
    }

    pub fn add(&mut self, id: ID, item: Box<T>) {
        self.items.insert(id, item);
    }

    pub fn remove(&mut self, id: &ID) -> Option<Box<T>> {
        self.items.remove(id)
    }
}
