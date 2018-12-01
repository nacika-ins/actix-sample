use actix::prelude::*;
use super::sample_actor::SampleActor;
use super::ping_actor::PingActor;
use std::thread;
use std::time::Duration;
use super::sample_actor::Mes;
use super::ping_actor::PingWatch;

#[derive(Message)]
pub struct Die;

impl Handler<Die> for SupervisorActor {
    type Result = ();

    fn handle(&mut self, _: Die, ctx: &mut Context<SupervisorActor>) {
        println!("die");
        ctx.stop();
        // System::current().stop();
    }
}

pub struct SupervisorActor;

impl Actor for SupervisorActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        let sample = SampleActor.start();
        let ping = PingActor.start();
        println!("I am SupervisorActor!");

        sample.do_send(Mes { text: "hello".to_string() });
        ping.do_send(PingWatch{ actor: sample, count: 0 });



    }
}

impl Supervised for SupervisorActor {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        println!("actor error");
    }
}