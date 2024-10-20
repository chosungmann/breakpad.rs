#include "utility.h"

#include <codecvt>
#include <locale>

namespace {
using WstringConverter =
    std::wstring_convert<std::codecvt_utf8<wchar_t>, wchar_t>;
}  // namespace

std::string ToStdString(const std::wstring& from) {
  return WstringConverter().to_bytes(from);
}

std::wstring ToStdWstring(const std::string& from) {
  return WstringConverter().from_bytes(from);
}
