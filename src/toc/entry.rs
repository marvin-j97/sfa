// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::Path,
};

pub type SectionName = Vec<u8>;

/// Entry in the table of contents (a section in the archive)
#[derive(Debug)]
pub struct TocEntry {
    pub(crate) name: SectionName,
    pub(crate) pos: u64,
    pub(crate) len: u64,
}

impl TocEntry {
    /// Returns the section name.
    #[must_use]
    pub fn name(&self) -> &[u8] {
        &self.name
    }

    /// Returns the section position.
    #[must_use]
    pub fn pos(&self) -> u64 {
        self.pos
    }

    /// Returns the section length in bytes.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u64 {
        self.len
    }

    #[doc(hidden)]
    pub fn buf_reader(&self, path: &Path) -> std::io::Result<impl std::io::BufRead> {
        let mut file = BufReader::new(File::open(path)?);
        file.seek(std::io::SeekFrom::Start(self.pos))?;
        Ok(file.take(self.len))
    }

    pub(crate) fn write_into(&self, mut writer: impl Write) -> crate::Result<()> {
        use byteorder::LE;

        writer.write_u64::<LE>(self.pos())?;
        writer.write_u64::<LE>(self.len())?;

        writer.write_u16::<LE>(
            #[allow(clippy::expect_used)]
            u16::try_from(self.name().len()).expect("section name should not be longer than 65535"),
        )?;
        writer.write_all(self.name())?;

        Ok(())
    }

    pub(crate) fn read_from_file(reader: &mut impl Read) -> crate::Result<Self> {
        use byteorder::LE;

        let pos = reader.read_u64::<LE>()?;
        let len = reader.read_u64::<LE>()?;
        let section_name_len = reader.read_u16::<LE>()?;

        let mut name = vec![0; section_name_len as usize];
        reader.read_exact(&mut name)?;

        Ok(Self { name, pos, len })
    }
}
