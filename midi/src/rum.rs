// Workspace modules
use events::Event;

pub struct Rum<const N: usize> {
    buttons: [crate::button::Button; N],
}

impl Rum<6> {
    pub const fn new() -> Rum<6> {
        Rum {
            buttons: crate::button::get_button6(),
        }
    }
}

impl Rum<12> {
    pub const fn new() -> Rum<12> {
        Rum {
            buttons: crate::button::get_button12(),
        }
    }
}

impl<const N: usize> Rum<N> {
    // A one, a two, I don't know what to do
    pub fn process_event(&self, event: events::Event) {
        match event {
            _ => {
                for button in self.buttons.iter() {
                    vprintln!("Button {}", button);
                }
            }
        }
    }
}
