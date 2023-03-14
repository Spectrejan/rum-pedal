pub mod StandardEvents;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    BUTTON0,
    BUTTON1,
    BUTTON2,
}

impl Distribution<Event> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Event {
        match rng.gen_range(0..=2) {
            0 => Event::BUTTON0,
            1 => Event::BUTTON1,
            2 => Event::BUTTON2,
            _ => panic!("Value out of range"),
        }
    }
}

pub trait IEventEngine {
    fn new() -> Self;
    fn get_next_event(&self) -> Option<Event>;
    fn start(&self);
    fn stop(&self);
    fn wait_for_event(&self);
}
