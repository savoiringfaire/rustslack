use crate::dispatcher;
use std::thread;

mod slack_client;

pub trait Chatter {
    fn send_message(&self, _: String, _: String);
    fn listen(&mut self);
}

pub fn register(dispatch: &mut dispatcher::Dispatcher) {
    let mut slack_chatter = slack_client::build_slack("api_key".to_string(), dispatch);

    let handle = thread::spawn(move || {
        slack_chatter.listen()
    });

    handle.join().unwrap();
}
