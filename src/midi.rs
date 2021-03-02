use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::sync::mpsc::{self, Receiver};

/// A MIDI note byte.
// TODO: Should the midi note expose the raw value?
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MidiNote(pub u8);

impl From<u8> for MidiNote {
    fn from(note: u8) -> Self {
        Self(note)
    }
}

impl From<MidiNote> for u8 {
    fn from(MidiNote(note): MidiNote) -> Self {
        note
    }
}

// TODO: Use MidiNote instead of raw values.
#[derive(Debug)]
enum InputMessage {
    NoteOn(u8),
    NoteOff(u8),
    ControlChange(u8, u8),
}

impl InputMessage {
    fn try_from_raw(msg: &[u8]) -> Option<InputMessage> {
        if let &[cmd, note, vel] = msg {
            match cmd {
                0x90 => Some(InputMessage::NoteOn(note)),
                0x80 => Some(InputMessage::NoteOff(note)),
                0xB0 => Some(InputMessage::ControlChange(note, vel)),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub struct MidiConnection {
    input: MidiInputConnection<()>,
    output: MidiOutputConnection,
    rx: Receiver<InputMessage>,
}

impl MidiConnection {
    pub fn new(port_name: &str) -> Result<Self, MidiConnectionError> {
        let (tx, rx) = mpsc::channel();

        let input = {
            let input = MidiInput::new("midi_input")?;
            let ports = input.ports();
            let id = ports
                .iter()
                .find(|port| {
                    if let Ok(name) = input.port_name(*port) {
                        name.starts_with(port_name)
                    } else {
                        false
                    }
                })
                .ok_or(MidiConnectionError::NameNotFound)?;
            let name = format!("{} Input", port_name);

            let callback = move |_, msg: &[u8], _: &mut ()| {
                if let Some(msg) = InputMessage::try_from_raw(msg) {
                    let _ = tx.send(msg);
                }
            };

            input.connect(id, &name, callback, ())?
        };

        let output = {
            let output = MidiOutput::new("midi_output")?;
            let ports = output.ports();
            let id = ports
                .iter()
                .find(|port| {
                    if let Ok(name) = output.port_name(*port) {
                        name.starts_with(port_name)
                    } else {
                        false
                    }
                })
                .ok_or(MidiConnectionError::NameNotFound)?;
            let name = format!("{} Output", port_name);

            output.connect(id, &name)?
        };

        Ok(Self { input, output, rx })
    }
}

pub enum MidiConnectionError {
    MidirInitError(midir::InitError),
    MidirInputConnectError(midir::ConnectError<MidiInput>),
    MidirOutputConnectError(midir::ConnectError<MidiOutput>),
    NameNotFound,
}

impl From<midir::InitError> for MidiConnectionError {
    fn from(error: midir::InitError) -> Self {
        Self::MidirInitError(error)
    }
}

impl From<midir::ConnectError<MidiInput>> for MidiConnectionError {
    fn from(error: midir::ConnectError<MidiInput>) -> Self {
        Self::MidirInputConnectError(error)
    }
}

impl From<midir::ConnectError<MidiOutput>> for MidiConnectionError {
    fn from(error: midir::ConnectError<MidiOutput>) -> Self {
        Self::MidirOutputConnectError(error)
    }
}
