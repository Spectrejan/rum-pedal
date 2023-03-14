use events::IEventEngine;
use events::StandardEvents;
use midi;
use midi::button::Button6;
use midi::rum::Rum;

fn main() {
    println!("6 Button pedal example");

    let engine = StandardEvents::StandardEvents::new();
    let rumi = Rum::<Button6, 6>::new();

    engine.start();

    loop {
        engine.wait_for_event();

        let event = engine.get_next_event();

        rumi.process_event(event.unwrap());
    }
}
