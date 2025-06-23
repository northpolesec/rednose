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

#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <sys/fcntl.h>

#include "rednose/src/export/bridge.rs.h"

TEST(ExportBridgeFFI, ExportBasicGCP) {
    // int fd = open("/tmp/rand.foo.bin", O_RDONLY);
    // ASSERT_GE(fd, 0) << "Failed to open test file...\n";

    // rednose::ExportStatus status =
    //     rednose::export_file_gcp(
    //       fd,
    //       "<token>",
    //       "<bucket>",
    //       "",
    //       "rand_foo.bin");

    // printf("Got GCP status: %d: %s\n", status.code, status.error.c_str());
    // EXPECT_EQ(status.code, rednose::ExportCode::Success);

    // close(fd);
}

TEST(ExportBridgeFFI, ExportBasicAWS) {
    // int fd = open("/tmp/rand.foo.bin", O_RDONLY);
    // ASSERT_GE(fd, 0) << "Failed to open test file...\n";

    // rednose::ExportStatus status =
    //     rednose::export_file_aws(
    //       fd,
    //       "<access_token>",
    //       "<secret_token>",
    //       "<session_token>",
    //       "<bucket>",
    //       "",
    //       "rand_foo.bin");

    // printf("Got AWS status: %d: %s\n", status.code, status.error.c_str());
    // EXPECT_EQ(status.code, rednose::ExportCode::Success);

    // close(fd);
}
