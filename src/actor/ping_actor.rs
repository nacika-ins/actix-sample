use actix::prelude::*;
use std::thread;
use std::time::Duration;
use super::sample_actor::SampleActor;
use tokio;

pub struct PingActor;

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am PingActor!");






    }
}

#[derive(Message)]
pub struct Ping;

#[derive(Message)]
pub struct PingWatch {
    pub actor: Addr<SampleActor>,
    pub count: i64
}

impl Handler<PingWatch> for PingActor {
    type Result = ();
    fn handle(&mut self, mut msg: PingWatch, _: &mut Context<Self>) {
        thread::Builder::new().name("ping-thread".to_string()).spawn(move || {
            loop {
                println!("ping");
                msg.actor.do_send(Ping{});
                msg.count += 1;
                thread::sleep(Duration::from_secs(1));
                if msg.count > 5 {
                    break;
                }
            };
            println!("loop end");
        });

    }
}
