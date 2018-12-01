extern crate actix;
use actix::prelude::*;
mod actor;
use actor::supervisor_actor::SupervisorActor;
use actor::supervisor_actor::Die;
use actix::{msgs, Actor, Supervised};
extern crate tokio;

fn main() {

    println!("Hello, world!");
    System::run(|| {
        let addr = actix::Supervisor::start(|_| SupervisorActor);
        addr.do_send(Die);
    });
}
