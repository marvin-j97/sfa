// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

//! *TFT* is a minimal, flat file archive encoding/decoding library for Rust.
//!
//! The file can be segmented into multiple sections (similar to a zip file), and individual sections accessed as a [`std::io::Read`].
//!
//! ```
//! use tft::{Writer, Reader};
//! use std::io::{Read, Write};
//! # let dir = tempfile::tempdir()?;
//! # let path = dir.path().join("hello.tft");
//!
//! let mut writer = Writer::new_at_path(&path)?;
//! writer.start("Section 1")?;
//! writer.write_all(b"Hello world!\n")?;
//! writer.finish()?;
//! // If on Unix, you probably want to fsync the directory here
//!
//! let reader = Reader::new(&path)?;
//! let toc = reader.toc();
//! assert_eq!(toc.len(), 1);
//! assert_eq!(toc[0].name(), b"Section 1");
//! assert_eq!(toc[0].len(), 13);
//!
//! let reader = toc[0].buf_reader(&path).unwrap();
//! assert_eq!(b"Hello world!\n", &*reader.bytes().collect::<Result<Vec<_>, _>>()?);
//! #
//! # Ok::<(), tft::Error>(())
//! ```

// #![doc(html_logo_url = "https://raw.githubusercontent.com/fjall-rs/lsm-tree/main/logo.png")]
// #![doc(html_favicon_url = "https://raw.githubusercontent.com/fjall-rs/lsm-tree/main/logo.png")]
#![deny(clippy::all, missing_docs, clippy::cargo)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![warn(clippy::pedantic, clippy::nursery)]
#![warn(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]
#![warn(clippy::multiple_crate_versions)]
#![allow(clippy::option_if_let_else)]
#![warn(clippy::redundant_feature_names)]

mod checksum;
mod error;
mod reader;
mod toc;
mod trailer;
mod writer;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub use checksum::Checksum;
pub use error::Error;
pub use reader::Reader;
pub use toc::{entry::TocEntry, Toc};
pub use writer::Writer;
