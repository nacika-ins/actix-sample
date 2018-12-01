extern crate actix;
mod actor;
use actor::supervisor_actor::SupervisorActor;
use actix::{msgs, Actor};
extern crate tokio;

fn main() {

    println!("Hello, world!");
    let system = actix::System::new("actor");
    let supervisor = SupervisorActor.start();


    system.run();
}
