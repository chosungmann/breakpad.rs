#include "breakpad.h"

#include "utility.h"

namespace breakpad {

namespace {

bool FilterCallback(void* context) {
  if (const auto* delegate = static_cast<ExceptionHandlerDelegate*>(context)) {
    return delegate->ShouldWriteMinidump();
  }
  return true;
}

bool MinidumpCallback(const char* working_path,
                      const char* minidump_id,
                      void* context,
                      bool succeeded) {
  if (const auto* delegate = static_cast<ExceptionHandlerDelegate*>(context)) {
    delegate->DidWriteMinidump(working_path, minidump_id);
  }
  return succeeded;
}

}  // namespace

std::unique_ptr<ExceptionHandler> CreateExceptionHandler(
    const ExceptionHandlerDelegate& delegate) {
  return std::make_unique<ExceptionHandler>(
      GetValidWorkingPath(delegate.GetWorkingPath()), FilterCallback,
      MinidumpCallback,
      static_cast<void*>(const_cast<ExceptionHandlerDelegate*>(&delegate)),
      true, nullptr);
}

}  // namespace breakpad
