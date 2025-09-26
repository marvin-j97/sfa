// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

/// An 128-bit checksum
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Checksum(u128);

impl Checksum {
    pub(crate) fn from_raw(value: u128) -> Self {
        Self(value)
    }

    /// Converts the checksum to integer.
    #[must_use]
    pub fn into_u128(self) -> u128 {
        self.0
    }

    pub(crate) fn check(&self, expected: Self) -> crate::Result<()> {
        if self == &expected {
            Ok(())
        } else {
            Err(crate::Error::ChecksumMismatch {
                expected,
                got: *self,
            })
        }
    }
}
