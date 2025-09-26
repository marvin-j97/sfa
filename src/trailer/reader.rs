// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use super::writer::TRAILER_MAGIC;
use crate::{checksum::Checksum, Result};
use byteorder::ReadBytesExt;
use std::io::{Read, Seek, SeekFrom};

#[allow(clippy::cast_possible_wrap)]
const TRAILER_SIZE: i64 = TRAILER_MAGIC.len() as i64 + 1 + 16 + 8;

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedTrailer {
    pub toc_checksum: Checksum,
    pub toc_pos: u64,
}

pub struct TrailerReader;

impl TrailerReader {
    pub fn read_from_file<R: Read + Seek>(reader: &mut R) -> Result<ParsedTrailer> {
        use byteorder::LE;

        log::trace!("Reading trailer");

        reader.seek(SeekFrom::End(-TRAILER_SIZE))?;

        {
            let mut buf = [0u8; TRAILER_MAGIC.len()];
            reader.read_exact(&mut buf)?;

            if buf != TRAILER_MAGIC {
                log::error!("Invalid version");
                return Err(crate::Error::InvalidVersion);
            }
        }

        {
            let checksum_type = reader.read_u8()?;
            if checksum_type != 0x0 {
                log::error!("Invalid checksum type");
                return Err(crate::Error::UnsupportedChecksumType);
            }
        }

        let toc_checksum = Checksum::from_raw(reader.read_u128::<LE>()?);
        let toc_pos = reader.read_u64::<LE>()?;

        Ok(ParsedTrailer {
            toc_checksum,
            toc_pos,
        })
    }
}
