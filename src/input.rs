use crate::element_idx::{ButtonIdx, SliderIdx};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Display},
};

pub enum InputEvent {
    ButtonEvent { idx: ButtonIdx, is_pressed: bool },
    SliderEvent { idx: SliderIdx, value: SliderValue },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliderValue(u8);

impl SliderValue {
    pub fn new(value: u8) -> Option<Self> {
        if value < 128 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn as_percent(self) -> f32 {
        (self.0 as f32) / 127f32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliderValueFromMidiError;

impl Display for SliderValueFromMidiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid velocity value for slider value")
    }
}

impl Error for SliderValueFromMidiError {}

impl TryFrom<u8> for SliderValue {
    type Error = SliderValueFromMidiError;

    fn try_from(vel: u8) -> Result<Self, Self::Error> {
        SliderValue::new(vel).ok_or(SliderValueFromMidiError)
    }
}
