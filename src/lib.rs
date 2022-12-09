use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type EventListener<'a> = dyn FnMut(&dyn Any) + 'a;

#[derive(Default)]
pub struct EventDispatcher<'a> {
    listeners: HashMap<TypeId, Vec<Box<EventListener<'a>>>>,
}

impl<'a> EventDispatcher<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_listener<E: 'static, L: FnMut(&E) + 'a>(&mut self, mut listener: L) {
        self.listeners
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Box::new(move |event| {
                if let Some(event) = event.downcast_ref::<E>() {
                    listener(event);
                }
            }));
    }

    pub fn dispatch<E: 'static>(&mut self, event: &E) {
        if let Some(listeners) = self.listeners.get_mut(&TypeId::of::<E>()) {
            for listener in listeners {
                listener(event);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::EventDispatcher;

    #[test]
    fn it_works() {
        struct Event;

        let event_count = RefCell::new(0);
        let mut dispatcher = EventDispatcher::new();
        dispatcher.add_listener(|_: &Event| *event_count.borrow_mut() += 1);

        dispatcher.dispatch(&Event);
        assert_eq!(*event_count.borrow(), 1);

        dispatcher.dispatch(&Event);
        assert_eq!(*event_count.borrow(), 2);
    }
}
