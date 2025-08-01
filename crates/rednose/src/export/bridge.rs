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

use crate::export::export::{export_file_aws, export_file_gcp};

#[cxx::bridge(namespace = "rednose")]
mod ffi {
    enum ExportCode {
        Success,
        InternalFailure,
        InvalidParam,
        InvalidCredentials,
        ReadFailure,
        UploadFailure,
    }

    struct ExportStatus {
        code: ExportCode,
        error: String,
    }

    extern "Rust" {
        fn export_file_aws(
            fd: i32,
            access_key: String,
            secret_access_key: String,
            session_token: String,
            bucket_name: String,
            prefix_path: String,
            destination_path: String,
        ) -> ExportStatus;

        fn export_file_gcp(
            fd: i32,
            bearer_token: String,
            bucket_name: String,
            prefix_path: String,
            destination_path: String,
        ) -> ExportStatus;
    }
}

// Re-export FFI types
pub use ffi::ExportCode;
pub use ffi::ExportStatus;
