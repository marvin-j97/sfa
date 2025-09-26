// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{
    toc::{reader::TocReader, Toc},
    trailer::reader::TrailerReader,
};
use std::io::{Read, Seek};

/// Archive reader
pub struct Reader {
    toc: Toc,
}

impl Reader {
    /// Creates a new reader from a file path.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn new(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let trailer = TrailerReader::read_from_file(&mut file)?;
        let toc = TocReader::read_from_file(&mut file, trailer.toc_pos, trailer.toc_checksum)?;
        Ok(Self { toc })
    }

    /// Creates a new reader from a reader.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn from_reader<R: Read + Seek>(mut reader: &mut R) -> crate::Result<Self> {
        let trailer = TrailerReader::read_from_file(&mut reader)?;
        let toc = TocReader::read_from_file(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
        Ok(Self { toc })
    }

    /// Lists the table of contents.
    #[must_use]
    pub fn toc(&self) -> &Toc {
        &self.toc
    }
}
