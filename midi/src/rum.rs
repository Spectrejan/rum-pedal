// Workspace modules
use events::Event;

// External modules
use core::ops::Index;

pub struct Rum<T> {
    buttons: T,
}

impl Rum<crate::button::Button6> {
    pub const fn new() -> Rum<crate::button::Button6> {
        Rum {
            buttons: crate::button::get_button6(),
        }
    }
}

impl Rum<crate::button::Button12> {
    pub const fn new() -> Rum<crate::button::Button12> {
        Rum {
            buttons: crate::button::get_button12(),
        }
    }
}

impl<T> Rum<T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: Sized,
    <T as Index<usize>>::Output: std::fmt::Display,
{
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
