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

use crate::export::bridge::{ExportCode, ExportStatus};

use object_store::aws::{AmazonS3, AmazonS3Builder};
use object_store::buffered::BufWriter;
use object_store::gcp::{GcpCredential, GoogleCloudStorage, GoogleCloudStorageBuilder};
use object_store::path::Path;
use object_store::{ObjectStore, StaticCredentialProvider};
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::io::FromRawFd;
use std::sync::Arc;
use tokio;
use tokio::io::AsyncWriteExt;

pub fn export_file_aws(
    fd: i32,
    access_key: String,
    secret_access_key: String,
    session_token: String,
    bucket_name: String,
    prefix_path: String,
    destination_path: String,
) -> ExportStatus {
    let s3_store: AmazonS3 = match AmazonS3Builder::new()
        .with_bucket_name(bucket_name)
        .with_access_key_id(access_key)
        .with_secret_access_key(secret_access_key)
        .with_token(session_token)
        .build()
    {
        Ok(store) => store,
        Err(e) => {
            return ExportStatus {
                code: ExportCode::InvalidCredentials,
                error: format!("Failed to create S3 client: {}", e),
            };
        }
    };

    match get_runtime() {
        Ok(rt) => rt.block_on(async_export_file(
            fd,
            prefix_path,
            destination_path,
            Arc::new(s3_store),
        )),
        Err(e) => e,
    }
}

pub fn export_file_gcp(
    fd: i32,
    bearer_token: String,
    bucket_name: String,
    prefix_path: String,
    destination_path: String,
) -> ExportStatus {
    let gcp_store: GoogleCloudStorage = match GoogleCloudStorageBuilder::new()
        .with_bucket_name(bucket_name)
        .with_credentials(Arc::new(StaticCredentialProvider::new(GcpCredential {
            bearer: bearer_token,
        })))
        .build()
    {
        Ok(store) => store,
        Err(e) => {
            return ExportStatus {
                code: ExportCode::InvalidCredentials,
                error: format!("Failed to create GCS client: {}", e),
            };
        }
    };

    match get_runtime() {
        Ok(rt) => rt.block_on(async_export_file(
            fd,
            prefix_path,
            destination_path,
            Arc::new(gcp_store),
        )),
        Err(e) => e,
    }
}

async fn async_export_file(
    fd: i32,
    prefix_path: String,
    destination_path: String,
    store: Arc<dyn ObjectStore>,
) -> ExportStatus {
    let full_destination_path = match prefix_path.is_empty() {
        true => destination_path,
        false => [prefix_path, destination_path].join("/"),
    };
    let path = Path::from(full_destination_path);

    // Create a File from the raw file descriptor
    let file = unsafe { File::from_raw_fd(fd) };
    let mut reader = BufReader::new(file);

    // Create a buffered writer for upload.
    let mut buf_writer = BufWriter::new(store, path.clone());

    // Stream data in 256KB chunks
    const CHUNK_SIZE: usize = 256 * 1024;
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF reached
            Ok(bytes_read) => {
                // Write the chunk to S3
                if let Err(e) = buf_writer.write_all(&buffer[..bytes_read]).await {
                    return ExportStatus {
                        code: ExportCode::UploadFailure,
                        error: format!("Failed to write to S3: {}", e),
                    };
                }
            }
            Err(e) => {
                return ExportStatus {
                    code: ExportCode::ReadFailure,
                    error: format!("Failed to read from file descriptor: {}", e),
                };
            }
        }
    }

    // Finalize the upload
    if let Err(e) = buf_writer.flush().await {
        return ExportStatus {
            code: ExportCode::UploadFailure,
            error: format!("Failed to flush S3 upload: {}", e),
        };
    }

    if let Err(e) = buf_writer.shutdown().await {
        return ExportStatus {
            code: ExportCode::UploadFailure,
            error: format!("Failed to complete S3 upload: {}", e),
        };
    }

    ExportStatus {
        code: ExportCode::Success,
        error: String::new(),
    }
}

fn get_runtime() -> Result<tokio::runtime::Runtime, ExportStatus> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => Ok(rt),
        Err(e) => Err(ExportStatus {
            code: ExportCode::InternalFailure,
            error: format!("Failed to create Tokio runtime: {}", e),
        }),
    }
}
