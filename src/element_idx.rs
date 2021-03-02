use super::midi::MidiNote;
use itertools::iproduct;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Display},
    iter,
};

/// Index of a button on the APC mini.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonIdx {
    GridButtonIdx(GridButtonIdx),
    SideButtonIdx(SideButtonIdx),
    BottomButtonIdx(BottomButtonIdx),
    CornerButtonIdx(CornerButtonIdx),
}

impl From<GridButtonIdx> for ButtonIdx {
    fn from(value: GridButtonIdx) -> Self {
        ButtonIdx::GridButtonIdx(value)
    }
}

impl From<SideButtonIdx> for ButtonIdx {
    fn from(value: SideButtonIdx) -> Self {
        ButtonIdx::SideButtonIdx(value)
    }
}

impl From<BottomButtonIdx> for ButtonIdx {
    fn from(value: BottomButtonIdx) -> Self {
        ButtonIdx::BottomButtonIdx(value)
    }
}

impl From<CornerButtonIdx> for ButtonIdx {
    fn from(value: CornerButtonIdx) -> Self {
        ButtonIdx::CornerButtonIdx(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonIdxFromMidiError;

impl Display for ButtonIdxFromMidiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid note value for button")
    }
}

impl Error for ButtonIdxFromMidiError {}

impl TryFrom<MidiNote> for ButtonIdx {
    type Error = ButtonIdxFromMidiError;

    fn try_from(note: MidiNote) -> Result<Self, Self::Error> {
        GridButtonIdx::try_from(note)
            .map(Into::into)
            .ok()
            .or_else(|| SideButtonIdx::try_from(note).map(Into::into).ok())
            .or_else(|| BottomButtonIdx::try_from(note).map(Into::into).ok())
            .or_else(|| CornerButtonIdx::try_from(note).map(Into::into).ok())
            .ok_or(ButtonIdxFromMidiError)
    }
}

/// Index of a button on the main APC mini grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridButtonIdx {
    pub col: u8,
    pub row: u8,
}

impl GridButtonIdx {
    /// Create a GridButton that indexes the given row and column.
    pub fn new(col: u8, row: u8) -> Option<Self> {
        if col < 8 && row < 8 {
            Some(Self { col, row })
        } else {
            None
        }
    }

    /// Returns an iterator over all grid buttons on the APC mini.
    pub fn all() -> impl Iterator<Item = Self> {
        iproduct!(0..8, 0..8).map(|(col, row)| GridButtonIdx { col, row })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridButtonIdxFromMidiError;

impl Display for GridButtonIdxFromMidiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid note value for grid button")
    }
}

impl Error for GridButtonIdxFromMidiError {}

impl TryFrom<MidiNote> for GridButtonIdx {
    type Error = GridButtonIdxFromMidiError;

    fn try_from(MidiNote(note): MidiNote) -> Result<Self, Self::Error> {
        if note >= 64 {
            return Err(GridButtonIdxFromMidiError);
        }
        let row = 7 - note / 8;
        let col = note % 8;
        Self::new(col, row).ok_or(GridButtonIdxFromMidiError)
    }
}

impl From<GridButtonIdx> for MidiNote {
    fn from(value: GridButtonIdx) -> Self {
        (8 * (7 - value.row) + value.col).into()
    }
}

/// Index of the corner button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CornerButtonIdx;

impl CornerButtonIdx {
    pub fn all() -> impl Iterator<Item = Self> {
        iter::once(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CornerButtonIdxFromMidiError;

impl Display for CornerButtonIdxFromMidiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid note value for corner button")
    }
}

impl TryFrom<MidiNote> for CornerButtonIdx {
    type Error = CornerButtonIdxFromMidiError;

    fn try_from(MidiNote(note): MidiNote) -> Result<Self, Self::Error> {
        if note == 98 {
            Ok(Self)
        } else {
            Err(CornerButtonIdxFromMidiError)
        }
    }
}

impl From<CornerButtonIdx> for MidiNote {
    fn from(_value: CornerButtonIdx) -> Self {
        98.into()
    }
}

macro_rules! impl_midi_range {
    ($name:ident, $base:literal, $error:ident, $errname:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(pub u8);

        impl $name {
            pub fn new(index: u8) -> Option<Self> {
                if index < 8 {
                    Some(Self(index))
                } else {
                    None
                }
            }

            pub fn all() -> impl Iterator<Item = Self> {
                (0..8).map($name)
            }
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $error;

        impl Display for $error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!("invalid note value for ", $errname))
            }
        }

        impl Error for $error {}

        impl TryFrom<MidiNote> for $name {
            type Error = $error;

            fn try_from(MidiNote(note): MidiNote) -> Result<Self, Self::Error> {
                if note >= $base {
                    Self::new(note - $base).ok_or($error)
                } else {
                    Err($error)
                }
            }
        }

        impl From<$name> for MidiNote {
            fn from($name(index): $name) -> MidiNote {
                (index + $base).into()
            }
        }
    };
}

impl_midi_range!(SliderIdx, 48, SliderIdxFromMidiError, "slider");
impl_midi_range!(
    BottomButtonIdx,
    64,
    BottomButtonIdxFromMidiError,
    "bottom button"
);
impl_midi_range!(SideButtonIdx, 82, SideButtonIdxFromMidiError, "side button");
