// Copyright 2025 North Pole Security, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Telemetry writer over a spool writer.

use std::path::Path;

use crate::spool;

use super::traits::{TableBuilder, autocomplete_row};

/// Wraps a spool writer for the given table builder type. Simplifies writing
/// data in a single tabular format to a spool.
pub struct Writer<T: TableBuilder> {
    table_builder: T,
    inner: spool::writer::Writer,
    batch_size: usize,
    buffered_rows: usize,
}

impl<T: TableBuilder> Writer<T> {
    pub fn new(batch_size: usize, writer: spool::writer::Writer, table_builder: T) -> Self {
        Self {
            table_builder: table_builder,
            inner: writer,
            batch_size: batch_size,
            buffered_rows: 0,
        }
    }

    pub fn table_builder(&mut self) -> &mut T {
        &mut self.table_builder
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        if self.buffered_rows == 0 {
            return Ok(());
        }
        let batch = self.table_builder.flush()?;
        self.buffered_rows = 0;
        self.inner.write_record_batch(batch, None)?;
        Ok(())
    }

    /// Attempts to autofill any nullable fields. See [autocomplete_row] for
    /// details.
    pub fn autocomplete(&mut self) -> anyhow::Result<()> {
        autocomplete_row(&mut self.table_builder)?;

        #[cfg(test)]
        {
            let (lo, hi) = self.table_builder.row_count();
            assert_eq!(lo, hi);
            assert_eq!(lo, self.buffered_rows);
        }

        // Write the batch to the spool if it's full.
        self.buffered_rows += 1;
        if self.buffered_rows >= self.batch_size {
            self.flush()?;
        }
        Ok(())
    }

    /// Returns the path to the spool directory.
    pub fn path(&self) -> &Path {
        &self.inner.path()
    }
}
