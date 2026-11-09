//! Linux input without libX11. Grabbing and injection are unavailable.
use crate::{DisplayError, Event, EventType, GrabError, ListenError, SimulateError};

pub struct Keyboard;

pub fn display_size() -> Result<(u64, u64), DisplayError> {
    Err(DisplayError::NoDisplay)
}

pub fn listen<T>(_callback: T) -> Result<(), ListenError>
where
    T: FnMut(Event) + 'static,
{
    Err(ListenError::MissingDisplayError)
}

pub fn simulate(_event_type: &EventType) -> Result<(), SimulateError> {
    Err(SimulateError)
}

pub fn simulate_char(_chr: char, _pressed: bool) -> Result<(), SimulateError> {
    Err(SimulateError)
}

pub fn simulate_unicode(_unicode: u16) -> Result<(), SimulateError> {
    Err(SimulateError)
}

pub fn enable_grab() {}

pub fn disable_grab() {}

pub fn is_grabbed() -> bool {
    false
}

pub fn start_grab_listen<T>(_callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    Err(GrabError::MissingDisplayError)
}

pub fn exit_grab_listen() {}
