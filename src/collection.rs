use std::ops::{Deref, DerefMut};
pub type Index = generational_arena::Index;

#[derive(Clone)]
pub struct Collection<T> {
    arena:generational_arena::Arena<T>
}
impl<T> Default for Collection<T> {
    fn default() -> Self {
        Self { arena: Default::default() }
    }
}

impl<T:PartialEq> Collection<T> {
    pub fn find_or_insert(&mut self, value: T) -> Index {
        for (index, v) in self.iter() {
            if *v == value {
                return index;
            }
        }

        return self.insert(value);
    }
}

impl<T> Deref for Collection<T> {
    type Target = generational_arena::Arena<T>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl<T> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arena
    }
}