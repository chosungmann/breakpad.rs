#include "breakpad.h"

#include "utility.h"

namespace breakpad {

namespace {

bool FilterCallback(void* context,
                    [[maybe_unused]] EXCEPTION_POINTERS* exinfo,
                    [[maybe_unused]] MDRawAssertionInfo* assertion) {
  if (const auto* delegate = static_cast<ExceptionHandlerDelegate*>(context)) {
    return delegate->ShouldWriteMinidump();
  }
  return true;
}

bool MinidumpCallback(const wchar_t* working_path,
                      const wchar_t* minidump_id,
                      void* context,
                      [[maybe_unused]] EXCEPTION_POINTERS* exinfo,
                      [[maybe_unused]] MDRawAssertionInfo* assertion,
                      bool succeeded) {
  if (const auto* delegate = static_cast<ExceptionHandlerDelegate*>(context)) {
    delegate->DidWriteMinidump(ToStdString(working_path),
                               ToStdString(minidump_id));
  }
  return succeeded;
}

}  // namespace

std::unique_ptr<ExceptionHandler> CreateExceptionHandler(
    const ExceptionHandlerDelegate& delegate) {
  return std::make_unique<ExceptionHandler>(
      ToStdWstring(GetValidWorkingPath(delegate.GetWorkingPath())),
      FilterCallback, MinidumpCallback,
      static_cast<void*>(const_cast<ExceptionHandlerDelegate*>(&delegate)),
      ExceptionHandler::HANDLER_ALL);
}

}  // namespace breakpad
