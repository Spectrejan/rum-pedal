// Crate modules
use crate::button::Button;

// Workspace modules
use events::Event;

// External modules
use core::marker::PhantomData;

pub struct Rum<T, const N: usize> {
    buttons: [Button; N],
    phantom: PhantomData<T>,
}

impl Rum<crate::button::Button6, 6> {
    pub const fn new() -> Rum<crate::button::Button6, 6> {
        Rum {
            buttons: crate::button::get_button6(),
            phantom: PhantomData,
        }
    }
}

impl Rum<crate::button::Button12, 12> {
    pub const fn new() -> Rum<crate::button::Button12, 12> {
        Rum {
            buttons: crate::button::get_button12(),
            phantom: PhantomData,
        }
    }
}

impl<T, const N: usize> Rum<T, N> {
    // A one, a two, I don't know what to do
    pub fn process_event(&self, event: events::Event) {
        let button_index = match event {
            Event::BUTTON0 => 0,
            Event::BUTTON1 => 1,
            Event::BUTTON2 => 2,
        };

        vprintln!("{}", self.buttons[button_index]);
    }
}
