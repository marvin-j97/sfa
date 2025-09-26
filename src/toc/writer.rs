// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::{checksum::Checksum, toc::entry::TocEntry};
use byteorder::WriteBytesExt;
use std::io::Write;

pub const TOC_MAGIC: &[u8] = b"TOC!";

struct ChecksummedWriter<W: std::io::Write> {
    inner: W,
    hasher: xxhash_rust::xxh3::Xxh3Default,
}

impl<W: std::io::Write> ChecksummedWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: writer,
            hasher: xxhash_rust::xxh3::Xxh3Default::new(),
        }
    }

    pub fn checksum(&self) -> Checksum {
        Checksum::from_raw(self.hasher.digest128())
    }
}

impl<W: std::io::Write> std::io::Write for ChecksummedWriter<W> {
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.inner.write(buf)
    }
}

pub struct TocWriter;

impl TocWriter {
    pub fn write_into(mut writer: impl Write, entries: &[TocEntry]) -> crate::Result<Checksum> {
        use byteorder::LE;

        log::trace!("Writing ToC");
        log::trace!("ToC: {entries:#?}");

        let mut writer = ChecksummedWriter::new(&mut writer);

        writer.write_all(TOC_MAGIC)?;
        writer.write_u32::<LE>(
            #[allow(clippy::expect_used)]
            u32::try_from(entries.len())
                .expect("table of contents should not have 4 billion or more entries"),
        )?;

        for entry in entries {
            entry.write_into(&mut writer)?;
        }

        Ok(writer.checksum())
    }
}
