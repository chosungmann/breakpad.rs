#pragma once

// PLATFORM_ANDROID: Android
// PLATFORM_IOS: iOS
// PLATFORM_LINUX: Linux (non-Android)
// PLATFORM_MACOS: macOS
// PLATFORM_WINDOWS: Windows

#if defined(_WIN32)
#define PLATFORM_WINDOWS 1

#elif defined(__ANDROID__)
#define PLATFORM_ANDROID 1

#elif defined(__APPLE__)
#include <TargetConditionals.h>
#if defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
#define PLATFORM_IOS 1
#elif defined(TARGET_OS_OSX) && TARGET_OS_OSX
#define PLATFORM_MACOS 1
#else
#error "Unsupported Apple Platform!"
#endif

#elif defined(__linux__) && !defined(__ANDROID__)
#define PLATFORM_LINUX 1

#else
#error "Unsupported Platform!"

#endif
