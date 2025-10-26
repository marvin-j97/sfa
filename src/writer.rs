// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{
    checksum_writer::ChecksummedWriter,
    toc::{
        entry::{SectionName, TocEntry},
        writer::TocWriter,
    },
    trailer::writer::TrailerWriter,
    Checksum,
};
use std::{
    fs::File,
    io::{BufWriter, Seek, Write},
    path::PathBuf,
};

/// Archive writer
#[allow(clippy::struct_field_names)]
pub struct Writer {
    writer: ChecksummedWriter<BufWriter<File>>,
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
        Ok(Self::from_writer(BufWriter::new(file)))
    }

    /// Returns a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> impl Write + Seek + '_ {
        self.writer.inner()
    }

    /// Creates a new writer with the given I/O writer.
    #[must_use]
    pub fn from_writer(writer: BufWriter<File>) -> Self {
        Self {
            writer: ChecksummedWriter::new(writer),
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
        let file_pos = self.writer.inner().stream_position()?;

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

    fn append_trailer(
        mut writer: &mut ChecksummedWriter<BufWriter<File>>,
        toc: &[TocEntry],
    ) -> crate::Result<()> {
        // Write ToC
        let toc_pos = writer.inner().stream_position()?;
        let toc_checksum = TocWriter::write_into(&mut writer, toc)?;

        let after_toc_pos = writer.inner().stream_position()?;
        let toc_len = after_toc_pos - toc_pos;

        // Write trailer
        TrailerWriter::write_into(writer, toc_checksum, toc_pos, toc_len)
    }

    /// Finishes the file.
    ///
    /// Returns a full-file checksum.
    ///
    /// # Errors
    ///
    /// Returns error, if an IO error occurred.
    #[allow(clippy::missing_panics_doc)]
    pub fn finish(mut self) -> crate::Result<Checksum> {
        self.append_toc_entry()?;
        Self::append_trailer(&mut self.writer, &self.toc)?;

        // Flush & sync
        log::trace!("Syncing file");

        self.writer.flush()?;
        self.writer.inner().get_mut().sync_all()?;

        Ok(self.writer.checksum())
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
        let trailer = TrailerReader::from_reader(&mut reader)?;
        assert_eq!(0, trailer.toc_pos);

        let toc = TocReader::from_reader(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
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
        let trailer = TrailerReader::from_reader(&mut reader)?;
        assert_eq!(data.len() as u64, trailer.toc_pos);

        let toc = TocReader::from_reader(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
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
        let trailer = TrailerReader::from_reader(&mut reader)?;
        assert_eq!(
            data.len() as u64 + data2.len() as u64 + data3.len() as u64,
            trailer.toc_pos,
        );

        let toc = TocReader::from_reader(&mut reader, trailer.toc_pos, trailer.toc_checksum)?;
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
