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

#include "rednose/src/logging/macos_logging.h"

#include <os/log.h>

static inline os_log_type_t LevelToLogType(uint8_t level) {
    switch (level) {
        case 0:
            return OS_LOG_TYPE_DEBUG;
        case 1:
            return OS_LOG_TYPE_INFO;
        case 2:
            return OS_LOG_TYPE_DEFAULT;
        case 3:
            return OS_LOG_TYPE_ERROR;
        default:
            return OS_LOG_TYPE_DEFAULT;
    }
}

void macos_log(uint8_t level, const char *message) {
  os_log_with_type(OS_LOG_DEFAULT, LevelToLogType(level), "%{public}s", message);
}
