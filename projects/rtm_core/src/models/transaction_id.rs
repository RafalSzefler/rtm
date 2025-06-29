use std::num::ParseIntError;

/// Represents a transaction id.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(transparent)]
#[must_use]
pub struct TransactionId {
    value: u32,
}

impl TransactionId {
    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.value
    }
}

impl From<u32> for TransactionId {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for TransactionId {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.parse::<u32>()?;
        Ok(Self { value })
    }
}
