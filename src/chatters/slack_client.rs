extern crate slack;

use slack::{Event, RtmClient, EventHandler};
use crate::dispatcher;
use crate::chatters;

pub struct Slack {
    api_key: String
}

#[allow(unused_variables)]
impl EventHandler for Slack {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        // find the general channel id from the `StartResponse`
        let general_channel_id = cli.start_response()
            .channels
            .as_ref()
            .and_then(|channels| {
                          channels
                              .iter()
                              .find(|chan| match chan.name {
                                        None => false,
                                        Some(ref name) => name == "general",
                                    })
                      })
            .and_then(|chan| chan.id.as_ref())
            .expect("general channel not found");
        let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
        // Send a message over the real time api websocket
    }
}

impl chatters::Chatter for Slack {
    fn send_message(&self, channel: String, message: String) {}
    fn listen(&mut self) {
        let api_key: &mut String = &mut self.api_key;
        let r = RtmClient::login_and_run(api_key, self);
        match r {
            Ok(_) => {}
            Err(err) => panic!("Error: {}", err),
        }
    }
}

pub fn build_slack(api_key: String, dispatch: &mut dispatcher::Dispatcher) -> Slack {
    return Slack{
        api_key
    }
}
