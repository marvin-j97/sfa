// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::checksum::Checksum;

/// Error type
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(std::io::Error),

    /// Unsupported file format version
    InvalidVersion,

    /// Unsupported checksum type
    UnsupportedChecksumType,

    /// Checksum mismatch
    ChecksumMismatch {
        /// The calculated checksum
        got: Checksum,

        /// The expected checksum as defined in the file format
        expected: Checksum,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TftError: {self:?}")
    }
}

impl From<std::io::Error> for Error {
    fn from(inner: std::io::Error) -> Self {
        Self::Io(inner)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            _ => None,
        }
    }
}
