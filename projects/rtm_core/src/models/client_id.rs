use std::num::ParseIntError;

/// Represents a client id.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
#[must_use]
pub struct ClientId {
    value: u16,
}

impl ClientId {
    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        self.value
    }
}

impl From<u16> for ClientId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for ClientId {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.parse::<u16>()?;
        Ok(Self { value })
    }
}
