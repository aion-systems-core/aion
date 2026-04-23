#pragma once

#include "aion.h"
#include <stdexcept>
#include <string>

namespace aion {
inline void check(int32_t code) {
    if (code != AION_OK) {
        const char* e = aion_last_error();
        throw std::runtime_error(e ? e : "aion error");
    }
}
} // namespace aion
