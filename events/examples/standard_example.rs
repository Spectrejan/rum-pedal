use events::IEventEngine;
use events::StandardEvents;

fn main() {
    println!("Fuck them fines");

    let engine = StandardEvents::StandardEvents::new();
    engine.start();

    loop {
        engine.wait_for_event();

        let event = engine.get_next_event();

        println!("Event: {:?}", event);
    }
}
