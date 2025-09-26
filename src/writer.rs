// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{
    toc::{
        entry::{SectionName, TocEntry},
        writer::TocWriter,
    },
    trailer::writer::TrailerWriter,
};
use std::{
    fs::File,
    io::{BufWriter, Seek, Write},
    path::PathBuf,
};

/// Archive writer
#[allow(clippy::struct_field_names)]
pub struct Writer {
    writer: BufWriter<File>,
    last_section_pos: u64,
    section_name: SectionName,
    toc: Vec<TocEntry>,
}

impl Writer {
    /// Creates a new writer.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn new_at_path(path: impl Into<PathBuf>) -> crate::Result<Self> {
        let path = std::path::absolute(path.into())?;
        let file = File::create_new(&path)?;
        Ok(Self::into_writer(BufWriter::new(file)))
    }

    /// Creates a new writer with the given I/O writer.
    #[must_use]
    pub fn into_writer(writer: BufWriter<File>) -> Self {
        Self {
            writer,
            last_section_pos: 0,
            section_name: SectionName::new(),
            toc: Vec::new(),
        }
    }
}

impl std::io::Write for Writer {
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }
}

impl Writer {
    /// Starts the first named section.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    pub fn start(&mut self, name: impl Into<SectionName>) -> std::io::Result<()> {
        self.append_toc_entry()?;
        self.section_name = name.into();
        Ok(())
    }

    fn append_toc_entry(&mut self) -> std::io::Result<()> {
        let file_pos = self.writer.stream_position()?;

        if file_pos > 0 {
            let name = std::mem::take(&mut self.section_name);
            self.toc.push(TocEntry {
                name,
                pos: self.last_section_pos,
                len: file_pos - self.last_section_pos,
            });
        }

        self.last_section_pos = file_pos;

        Ok(())
    }

    /// Finishes the file.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    #[allow(clippy::missing_panics_doc)]
    pub fn finish(mut self) -> crate::Result<()> {
        log::trace!("Finishing file");

        self.append_toc_entry()?;

        // Write ToC
        let toc_pos = self.writer.stream_position()?;
        let toc_checksum = TocWriter::write_into(&mut self.writer, &self.toc)?;

        let after_toc_pos = self.writer.stream_position()?;
        let toc_len = after_toc_pos - toc_pos;

        // Write trailer
        TrailerWriter::write_into(&mut self.writer, toc_checksum, toc_pos, toc_len)?;

        // Flush & sync
        log::trace!("Syncing file");

        self.writer.flush()?;
        self.writer.get_mut().sync_all()?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
mod tests {
    use super::*;
    use crate::toc::reader::TocReader;
    use crate::trailer::reader::TrailerReader;
    use std::io::Write;
    use test_log::test;

    #[test]
    fn writer_empty() -> crate::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("file.sfa");

        let writer = Writer::new_at_path(&path)?;
        writer.finish()?;

        let mut reader = File::open(&path)?;
        let trailer = TrailerReader::read_from_file(&mut reader)?;
        assert_eq!(0, trailer.toc_pos);

        let toc = TocReader::read_from_file(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
        assert_eq!(0, toc.len());
        assert!(toc.is_empty());
        assert!(toc.section(b"hello").is_none());

        Ok(())
    }

    #[test]
    fn writer_simple() -> crate::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("file.sfa");

        let data = b"hello world";

        let mut writer = Writer::new_at_path(&path)?;
        writer.write_all(data)?;
        writer.finish()?;

        let mut reader = File::open(&path)?;
        let trailer = TrailerReader::read_from_file(&mut reader)?;
        assert_eq!(data.len() as u64, trailer.toc_pos);

        let toc = TocReader::read_from_file(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
        assert_eq!(1, toc.len());
        assert!(toc.section(b"hello").is_none());
        assert!(toc.section(b"").is_some());

        assert_eq!(0, toc[0].pos);
        assert_eq!(data.len() as u64, toc[0].len);
        assert_eq!(&[] as &[u8], &*toc[0].name);

        Ok(())
    }

    #[test]
    fn writer_multiple_sections() -> crate::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("file.sfa");

        let data = b"hello world";
        let data2 = b"hello world2";
        let data3 = b"hello world3";

        let mut writer = Writer::new_at_path(&path)?;
        writer.write_all(data)?;
        writer.start("section1")?;
        writer.write_all(data2)?;
        writer.start("section2")?;
        writer.write_all(data3)?;
        writer.finish()?;

        let mut reader = File::open(&path)?;
        let trailer = TrailerReader::read_from_file(&mut reader)?;
        assert_eq!(
            data.len() as u64 + data2.len() as u64 + data3.len() as u64,
            trailer.toc_pos,
        );

        let toc = TocReader::read_from_file(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
        assert_eq!(3, toc.len());
        assert!(toc.section(b"hello").is_none());
        assert!(toc.section(b"").is_some());
        assert!(toc.section(b"section1").is_some());
        assert!(toc.section(b"section2").is_some());

        assert_eq!(0, toc[0].pos);
        assert_eq!(data.len() as u64, toc[0].len());

        assert_eq!(&[] as &[u8], &*toc[0].name);
        assert_eq!(b"section1", &*toc[1].name);
        assert_eq!(b"section2", &*toc[2].name);

        assert_eq!(data.len() as u64, toc[0].len);
        assert_eq!(data2.len() as u64, toc[1].len);
        assert_eq!(data3.len() as u64, toc[2].len);

        Ok(())
    }
}
