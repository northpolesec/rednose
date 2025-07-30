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

use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

pub mod writer;

fn spool_path(base_dir: &Path) -> PathBuf {
    base_dir.join("spool")
}

fn tmp_path(base_dir: &Path) -> PathBuf {
    base_dir.join("tmp")
}

// Rounds up file size to the next full block (usually 4096 bytes).
fn approx_file_occupation(file_size: usize) -> usize {
    const BLOCK_SIZE: usize = 4096;
    BLOCK_SIZE * (file_size / BLOCK_SIZE + if file_size % BLOCK_SIZE != 0 { 1 } else { 0 })
}

fn approx_dir_occupation(dir: &Path) -> Result<usize> {
    let mut total = 0;
    if !dir.is_dir() {
        return Err(Error::new(ErrorKind::NotADirectory, "Not a directory"));
    }

    for entry in dir.read_dir()? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            total += approx_dir_occupation(&entry.path())?;
        } else if metadata.is_file() {
            total += approx_file_occupation(metadata.len() as usize);
        } else {
            // Ignore other types of files.
        }
    }
    Ok(total)
}
