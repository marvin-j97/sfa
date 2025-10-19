// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{
    toc::{reader::TocReader, Toc},
    trailer::reader::TrailerReader,
};
use std::io::{BufReader, Read, Seek};

/// Archive reader
pub struct Reader {
    toc: Toc,
}

impl Reader {
    /// Creates a new [`Reader`] from a file path.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn new(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let file = std::fs::File::open(path)?;
        let mut file = BufReader::with_capacity(4_096, file);
        let trailer = TrailerReader::from_reader(&mut file)?;
        let toc = TocReader::from_reader(&mut file, trailer.toc_pos, trailer.toc_checksum)?;
        Ok(Self { toc })
    }

    /// Creates a new [`Reader`] from a reader.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn from_reader<R: Read + Seek>(mut reader: &mut R) -> crate::Result<Self> {
        let trailer = TrailerReader::from_reader(&mut reader)?;
        let toc = TocReader::from_reader(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
        Ok(Self { toc })
    }

    /// Lists the table of contents.
    #[must_use]
    pub fn toc(&self) -> &Toc {
        &self.toc
    }
}
