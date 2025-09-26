// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::checksum::Checksum;
use byteorder::WriteBytesExt;

pub const TRAILER_MAGIC: &[u8] = b"TRA1";

pub struct TrailerWriter;

impl TrailerWriter {
    pub fn write_into<W: std::io::Write>(
        mut writer: W,
        toc_checksum: Checksum,
        toc_pos: u64,
    ) -> crate::Result<()> {
        use byteorder::LE;

        log::trace!("Writing trailer");

        writer.write_all(TRAILER_MAGIC)?;
        writer.write_u8(0x0)?; // Checksum type, xxh3 = 0x0
        writer.write_u128::<LE>(toc_checksum.into_u128())?;
        writer.write_u64::<LE>(toc_pos)?;

        Ok(())
    }
}
