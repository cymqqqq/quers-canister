use std::rc::{Rc, Weak};
use std::sync::{Arc};

enum EventType {
    Click,
    DoubleClick,
    Touch,
    None,
}

pub struct Event {
    e_type: EventType,
}

pub trait HandleEvent {
    fn on_event(&self, event: &Event);
}

pub struct EventHandler;

impl HandleEvent for EventHandler {
    fn on_event(&self, event: &Event) {
        match event.e_type {
            EventType::Click => {
                println!("clicked");
            },
            EventType::DoubleClick => {
                println!("double clicked!");
            },
            EventType::Touch => {
                println!("touched");
            },
            _ => {
                println!("none");
            },
        }
    }
}

impl EventHandler {
    pub fn other_func(&self) {
        println!("other_func");
    }
}

pub struct Dispatcher<'a> {
    events: Vec<Event>,
    handlers: Vec<Arc<Weak<dyn HandleEvent + 'a>>>,
}

impl<'a> Dispatcher<'a> {
    pub fn register_handle(&mut self, cb: Weak<impl HandleEvent + 'a>) -> usize {
        self.handlers.push(Arc::new(cb));
        let id = self.handlers.len();
        id - 1
    }

    pub fn dispatch_event(&self) {
        for h in self.handlers.iter() {
            for event in self.events.iter() {
                h.upgrade().unwrap().on_event(event);
            }
        }
    }

    pub fn unregister_handler(&mut self, id: usize) {
        self.handlers.remove(id);
    }
}

fn main() {
    let mut dispatcher = Dispatcher {
        events: vec![Event {
            e_type: EventType::Click,
        }, Event {
            e_type: EventType::DoubleClick,
        }, Event {
            e_type: EventType::Touch,
        }, Event {
            e_type: EventType::None,
        }],
        handlers: vec![],
    };
    let handler = Rc::new(EventHandler);
    let handler_id = dispatcher.register_handle(Rc::downgrade(&handler));
    println!("handler id: {}", handler_id);
    handler.other_func();
    let handler1 = Rc::new(EventHandler);
    let handler1_id = dispatcher.register_handle(Rc::downgrade(&handler1));
    println!("handler1 id: {}", handler1_id);
    handler1.other_func();
    dispatcher.dispatch_event();
    dispatcher.unregister_handler(handler1_id);
    dispatcher.dispatch_event();
}