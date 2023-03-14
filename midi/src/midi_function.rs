#[derive(Debug, Copy, Clone)]
pub enum MidiFunction {
    BankChange,
    ControlChange,
    ProgramChange,
}
