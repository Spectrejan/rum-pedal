#![cfg_attr(not(debug_assertions), no_std)]

macro_rules! vprintln {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
        #[cfg(not(debug_assertions))]
        {

        }
    }};
}

pub mod button;
mod midi_function;
pub mod rum;
