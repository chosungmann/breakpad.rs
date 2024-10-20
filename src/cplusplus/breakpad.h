#pragma once

#include <memory>

#include "breakpad_exception_handler_delegate.h"
#include "platform.h"

#if defined(PLATFORM_ANDROID)
#include "third_party/breakpad/src/client/linux/handler/exception_handler.h"
#elif defined(PLATFORM_IOS)
#include "third_party/breakpad/src/client/ios/exception_handler_no_mach.h"
#elif defined(PLATFORM_LINUX)
#include "third_party/breakpad/src/client/linux/handler/exception_handler.h"
#elif defined(PLATFORM_MACOS)
#include "third_party/breakpad/src/client/mac/handler/exception_handler.h"
#elif defined(PLATFORM_WINDOWS)
#include "third_party/breakpad/src/client/windows/handler/exception_handler.h"
#else
#error "Unsupported Platform!"
#endif

namespace breakpad {

using ExceptionHandler = google_breakpad::ExceptionHandler;

std::unique_ptr<ExceptionHandler> CreateExceptionHandler(
    const ExceptionHandlerDelegate& delegate);

}  // namespace breakpad
