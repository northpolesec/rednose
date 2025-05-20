# Copyright 2025 North Pole Security, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_cc//cc:defs.bzl", "objc_library")
load("@rules_rust//rust:defs.bzl", "rust_static_library", "rust_test")

objc_library(
    name = "macos_logging",
    srcs = ["src/logging/macos_logging.mm"],
    hdrs = ["src/logging/macos_logging.h"],
)

rust_static_library(
    name = "rednose_static",
    srcs = glob(["src/**/*.rs"]),
    edition = "2024",
    deps = [
        ":macos_logging",
    ],
)

rust_test(
    name = "rednose_test",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":macos_logging",
    ]
)
