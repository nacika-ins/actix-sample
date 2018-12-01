use actix::prelude::*;
use std::thread;
use std::time::Duration;
use super::ping_actor::Ping;

pub struct SampleActor;

impl Actor for SampleActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am SampleActor!");
        thread::sleep(Duration::from_secs(3));
    }
}

#[derive(Message)]
pub struct Mes {
    pub text: String
}

impl Handler<Mes> for SampleActor {
    type Result = ();

    fn handle(&mut self, msg: Mes, _: &mut Context<Self>) {
        println!("SampleActor: {}", msg.text);

    }
}

impl Handler<Ping> for SampleActor {
    type Result = ();

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) {
        println!("pong");
    }
}