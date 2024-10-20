#include "utility.h"

std::tuple<std::string, std::string> GetMinidumpPathComponents(
    const std::string& path) {
  // A minidump path consists of two components: a working path and a minidump
  // id. A working path is a directory where a minidump is located, and a
  // minidump id is a unique identifier of a minidump.
  //
  //   /minidump/path/F79622A0-AD12-4C91-A0DB-904B2B167317.dmp
  //   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  //   |              |
  //   |              minidump id: F79622A0-AD12-4C91-A0DB-904B2B167317
  //   |
  //   working path: /minidump/path/
  //
  size_t slash = path.rfind("/");
  size_t dot_dmp = path.rfind(".dmp");
  if (slash == std::string::npos || dot_dmp == std::string::npos) {
    return std::make_tuple(std::string(), std::string());
  }
  std::string working_path = slash == 0 ? "/" : path.substr(0, slash);
  std::string minidump_id = path.substr(slash + 1, dot_dmp - slash - 1);
  return std::make_tuple(working_path, minidump_id);
}
