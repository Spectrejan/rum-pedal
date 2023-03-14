use crate::{Event, IEventEngine};
use rand;
use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};

pub struct StandardEvents {
    ev: Arc<Mutex<Option<Event>>>,
    cvar: Arc<Condvar>,
}

impl StandardEvents {
    fn event_loop(ev_ref: Arc<Mutex<Option<Event>>>, cvar: Arc<Condvar>) {
        let mut rng = rand::thread_rng();
        loop {
            {
                let mut ev = ev_ref.lock().unwrap();
                let event: Event = rand::random();
                *ev = Some(event);
                cvar.notify_all();
            }
            let delay: u8 = rng.gen_range(3..30);
            std::thread::sleep(std::time::Duration::from_secs(delay as u64));
        }
    }
}

impl IEventEngine for StandardEvents {
    fn new() -> StandardEvents {
        StandardEvents {
            ev: Arc::new(Mutex::new(None)),
            cvar: Arc::new(Condvar::new()),
        }
    }

    fn get_next_event(&self) -> Option<Event> {
        return *self.ev.lock().unwrap();
    }

    fn start(&self) {
        let ev = self.ev.clone();
        let cv = self.cvar.clone();
        std::thread::spawn(move || Self::event_loop(ev, cv));
    }

    fn stop(&self) {}

    fn wait_for_event(&self) {
        let ev2 = Arc::clone(&self.ev);
        let mut event = ev2.lock().unwrap();
        self.cvar.wait(event);
    }
}
