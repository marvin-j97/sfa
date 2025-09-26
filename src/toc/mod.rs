// Copyright (c) 2025-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use crate::TocEntry;

pub mod entry;
pub mod reader;
pub mod writer;

/// Table of contents
pub struct Toc(pub(crate) Vec<TocEntry>);

impl Toc {
    /// Helper method to find a section by name.
    #[must_use]
    pub fn section(&self, name: &[u8]) -> Option<&TocEntry> {
        self.iter().find(|entry| entry.name() == name)
    }
}

impl std::ops::Deref for Toc {
    type Target = [TocEntry];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
