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

use std::ffi::CString;
use std::os::raw::c_char;

#[repr(u8)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Default = 2,
    Error = 3,
}

unsafe extern "C" {
    fn macos_log(level: u8, message: *const c_char);
}

pub fn log_entry(log_level: LogLevel, args: std::fmt::Arguments) {
    let message = std::fmt::format(args);

    #[cfg(target_os = "macos")]
    {
        let c_message = match CString::new(message) {
            Ok(c_str) => c_str,
            Err(_) => {
                // Handle strings with null bytes (unlikely but possible)
                let fallback = "Error: Log message contains null bytes";
                CString::new(fallback).unwrap()
            }
        };

        unsafe {
            macos_log(log_level as u8, c_message.as_ptr());
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Fallback for other OSes or if not macOS
        eprintln!("[{}] {}", level_to_string_generic(log_level), message);
    }
}

// Helper for fallback logging
#[cfg(not(target_os = "macos"))]
fn level_to_string_generic(log_level: LogLevel) -> &'static str {
    match log_level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Default => "DEFAULT",
        LogLevel::Error => "ERROR",
    }
}

#[macro_export]
macro_rules! rlog_debug {
    ($($arg:tt)+) => (
        use $crate::logging::logging;
        logging::log_entry(logging::LogLevel::Debug, format_args!($($arg)+))
    );
}

#[macro_export]
macro_rules! rlog_info {
    ($($arg:tt)+) => (
        logging::log_entry(logging::LogLevel::Info, format_args!($($arg)+))
    );
}

#[macro_export]
macro_rules! rlog {
    ($($arg:tt)+) => (
        logging::log_entry(logging::LogLevel::Default, format_args!($($arg)+))
    );
}

#[macro_export]
macro_rules! rlog_error {
    ($($arg:tt)+) => (
        logging::log_entry(logging::LogLevel::Error, format_args!($($arg)+))
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_logging() {
        rlog_debug!("Hey debug");
        rlog_info!("Hey info");
        rlog!("Hey default");
        rlog_error!("Hey error");
    }
}
