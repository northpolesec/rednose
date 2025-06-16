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

load("@//:rust.bzl", "rust_cxx_bridge")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_cc//cc:defs.bzl", "objc_library", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_static_library", "rust_test")

package(
    default_visibility = ["//:rednose_package_group"],
)

rust_cxx_bridge(
    name = "export_bridge",
    src = "src/export/bridge.rs",
    deps = [
        ":rednose_static",
    ],
)

objc_library(
    name = "macos_logging",
    srcs = ["src/logging/macos_logging.mm"],
    hdrs = ["src/logging/macos_logging.h"],
    target_compatible_with = select({
        "@platforms//os:osx": [],
        "//conditions:default": ["@platforms//:incompatible"],
    }),
)

cc_test(
    name = "export_bridge_test",
    srcs = ["src/export/export_bridge_test.cc"],
    deps = [
        ":export_bridge",
        ":rednose_static",
        "@googletest//:gtest",
        "@googletest//:gtest_main",
        ":macos_frameworks",
    ],
)

cc_library(
    name = "macos_frameworks",
    linkopts = select({
        "@platforms//os:osx": [
            "-framework Security",
            "-framework CoreFoundation",
        ],
        "//conditions:default": [],
    }),
    target_compatible_with = ["@platforms//os:osx"],
)

rust_static_library(
    name = "rednose_static",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ) + select({
        "@platforms//os:osx": [
            ":macos_logging",
            ":macos_frameworks",
        ],
        "//conditions:default": [],
    }),
    visibility = ["//visibility:public"],
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
        ":macos_frameworks",
    ]
)
