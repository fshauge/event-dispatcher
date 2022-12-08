use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type EventListener = dyn Fn(&dyn Any);

#[derive(Default)]
pub struct EventDispatcher {
    listeners: HashMap<TypeId, HashMap<TypeId, Box<EventListener>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_listener<E: 'static, L: Fn(&E) + 'static>(&mut self, listener: L) {
        self.listeners.entry(TypeId::of::<E>()).or_default().insert(
            TypeId::of::<L>(),
            Box::new(move |event| {
                if let Some(event) = event.downcast_ref::<E>() {
                    listener(event);
                }
            }),
        );
    }

    pub fn remove_listener<E: 'static, L: Fn(&E) + 'static>(&mut self, _: L) {
        self.listeners
            .entry(TypeId::of::<E>())
            .or_default()
            .remove(&TypeId::of::<L>());
    }

    pub fn dispatch<E: 'static>(&self, event: &E) {
        if let Some(listeners) = self.listeners.get(&TypeId::of::<E>()) {
            for listener in listeners.values() {
                listener(event);
            }
        };
    }
}
