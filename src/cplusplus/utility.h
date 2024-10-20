#pragma once

#include <string>

#include "platform.h"

std::string GetValidWorkingPath(const std::string& path);

#if defined(PLATFORM_ANDROID) || defined(PLATFORM_LINUX)
#include <tuple>
std::tuple<std::string, std::string> GetMinidumpPathComponents(
    const std::string& path);
#endif  // defined(PLATFORM_ANDROID) || defined(PLATFORM_LINUX)

#if defined(PLATFORM_WINDOWS)
std::string ToStdString(const std::wstring& from);
std::wstring ToStdWstring(const std::string& from);
#endif  // defined(PLATFORM_WINDOWS)
