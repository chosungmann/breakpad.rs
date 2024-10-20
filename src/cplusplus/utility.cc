#include "utility.h"

namespace {
inline constexpr char kDefaultWorkingPath[] = ".";
}  // namespace

std::string GetValidWorkingPath(const std::string& path) {
  return path.empty() ? kDefaultWorkingPath : path;
}
