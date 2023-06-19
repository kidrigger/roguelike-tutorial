use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Default)]
pub struct ResourceHandler {
    holder: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceHandler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResourceHandler {
    pub fn insert<T: Any + Sized>(&mut self, res: T) -> Option<T> {
        let val = self.holder.insert(TypeId::of::<T>(), Box::new(res));
        val.and_then(|b| b.downcast::<T>().ok()).map(|bx| *bx)
    }

    pub fn fetch<T: Any + Sized>(&self) -> &T {
        self.holder
            .get(&TypeId::of::<T>())
            .expect("No resource of requested type")
            .as_ref()
            .downcast_ref::<T>()
            .unwrap()
    }

    pub fn fetch_mut<T: Any + Sized>(&mut self) -> &mut T {
        self.holder
            .get_mut(&TypeId::of::<T>())
            .expect("No resource of requested type")
            .as_mut()
            .downcast_mut::<T>()
            .unwrap()
    }
}
