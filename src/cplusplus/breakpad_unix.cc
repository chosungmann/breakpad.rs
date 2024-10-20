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

bool MinidumpCallback(const google_breakpad::MinidumpDescriptor& descriptor,
                      void* context,
                      bool succeeded) {
  if (const auto* delegate = static_cast<ExceptionHandlerDelegate*>(context)) {
    const auto& [working_path, minidump_id] =
        GetMinidumpPathComponents(descriptor.path());
    delegate->DidWriteMinidump(working_path, minidump_id);
  }
  return succeeded;
}

}  // namespace

std::unique_ptr<ExceptionHandler> CreateExceptionHandler(
    const ExceptionHandlerDelegate& delegate) {
  return std::make_unique<ExceptionHandler>(
      google_breakpad::MinidumpDescriptor(
          GetValidWorkingPath(delegate.GetWorkingPath())),
      FilterCallback, MinidumpCallback,
      static_cast<void*>(const_cast<ExceptionHandlerDelegate*>(&delegate)),
      true, -1);
}

}  // namespace breakpad
