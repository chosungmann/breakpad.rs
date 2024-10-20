#pragma once

#include <string>

namespace breakpad {

class ExceptionHandlerDelegate {
 public:
  virtual void DidWriteMinidump(const std::string& working_path,
                                const std::string& minidump_id) const = 0;
  virtual std::string GetWorkingPath() const = 0;
  virtual bool ShouldWriteMinidump() const = 0;

  virtual ~ExceptionHandlerDelegate() = default;
};

}  // namespace breakpad
