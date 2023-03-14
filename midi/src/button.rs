// Crate Modules
use crate::midi_function::MidiFunction;

// External module
#[cfg(debug_assertions)]
use core::fmt;
#[cfg(debug_assertions)]
use core::str;
use heapless::String;

#[derive(Debug, Clone)]
pub struct Button {
    pub id: u8,
    pub name: String<16>,
    pub function: MidiFunction,
    pub command: [u8; 3],
}

#[cfg(debug_assertions)]
impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} {:?} {}",
            self.id,
            self.name,
            self.function,
            str::from_utf8(&self.command).unwrap(),
        )
    }
}

macro_rules! assemble_button_entry {
    ($($elem:expr),*) => {
        [
        $(
            Button {
                id: $elem,
                name: String::new(),
                function: MidiFunction::ProgramChange,
                command: [0;3],
            },
        )*
        ]
    }
}

pub type Button6 = [Button; 6];
pub const fn get_button6() -> Button6 {
    assemble_button_entry!(1, 2, 3, 4, 5, 6)
}

pub type Button12 = [Button; 12];
pub const fn get_button12() -> Button12 {
    assemble_button_entry!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
}
