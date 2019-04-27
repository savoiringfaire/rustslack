extern crate slack;

use std::time;

pub trait Chatter {
    fn send_message(&self, _: String, _: String);
}

pub trait Handler {
    fn handle(&mut self, _: &Event);
}

pub struct Dispatcher<'a> {
    handlers: Vec<&'a mut Handler>
}

enum Event<'a> {
    TimerDone {
        identifier: String
    },
    Message {
        service: &'a Chatter,
        user: String,
        message: String
    }
}

pub fn build_dispatcher<'a>() -> Dispatcher<'a> {
    Dispatcher {
        handlers: Vec::new()
    }
}

impl<'a> Dispatcher<'a> {
    pub fn dispatch(&mut self, event: &Event<'a>) {
        for handler in &mut self.handlers {
            handler.handle(event);
        }
        // Dispatch the event
    }

    pub fn register_handler(&mut self, handler: &'a mut Handler) {
        self.handlers.push(handler)
    }

    pub fn handler_count(&self) -> usize {
        return self.handlers.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestHandler {
        called: bool
    }

    impl Handler for TestHandler {
        fn handle<'a>(&mut self, event: &Event<'a>) {
            self.called = true;
        }
    }

    #[test]
    fn function_called_when_event_dispatched() {
        let mut dispatcher = build_dispatcher();
        let mut handler = TestHandler{called: false};
        dispatcher.register_handler(&mut handler);
        dispatcher.dispatch(&Event::TimerDone{identifier: "test".to_string()});
        assert!(handler.called);
    }

    #[test]
    fn count_of_handlers_increases_after_register() {
        let mut dispatcher = build_dispatcher();
        assert!(dispatcher.handler_count() == 0);
        let mut handler = TestHandler{called: false};
        dispatcher.register_handler(&mut handler);
        assert!(dispatcher.handler_count() == 1);
    }
}
