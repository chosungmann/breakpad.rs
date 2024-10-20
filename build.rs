use miette::IntoDiagnostic;

const THIRD_PARTY_BREAKPAD_APPLE: &[&str] = &[
    "third_party/breakpad/src/client/mac/handler/breakpad_nlist_64.cc",
    "third_party/breakpad/src/client/mac/handler/dynamic_images.cc",
    "third_party/breakpad/src/client/mac/handler/minidump_generator.cc",
    "third_party/breakpad/src/client/minidump_file_writer.cc",
    "third_party/breakpad/src/common/convert_UTF.cc",
    "third_party/breakpad/src/common/mac/arch_utilities.cc",
    "third_party/breakpad/src/common/mac/file_id.cc",
    "third_party/breakpad/src/common/mac/macho_id.cc",
    "third_party/breakpad/src/common/mac/macho_utilities.cc",
    "third_party/breakpad/src/common/mac/macho_walker.cc",
    "third_party/breakpad/src/common/mac/string_utilities.cc",
    "third_party/breakpad/src/common/md5.cc",
    "third_party/breakpad/src/common/string_conversion.cc",
];

const THIRD_PARTY_BREAKPAD_UNIX: &[&str] = &[
    "third_party/breakpad/src/client/linux/crash_generation/crash_generation_client.cc",
    "third_party/breakpad/src/client/linux/dump_writer_common/thread_info.cc",
    "third_party/breakpad/src/client/linux/dump_writer_common/ucontext_reader.cc",
    "third_party/breakpad/src/client/linux/handler/exception_handler.cc",
    "third_party/breakpad/src/client/linux/handler/minidump_descriptor.cc",
    "third_party/breakpad/src/client/linux/log/log.cc",
    "third_party/breakpad/src/client/linux/microdump_writer/microdump_writer.cc",
    "third_party/breakpad/src/client/linux/minidump_writer/linux_dumper.cc",
    "third_party/breakpad/src/client/linux/minidump_writer/linux_ptrace_dumper.cc",
    "third_party/breakpad/src/client/linux/minidump_writer/minidump_writer.cc",
    "third_party/breakpad/src/client/linux/minidump_writer/pe_file.cc",
    "third_party/breakpad/src/client/minidump_file_writer.cc",
    "third_party/breakpad/src/common/convert_UTF.cc",
    "third_party/breakpad/src/common/linux/elfutils.cc",
    "third_party/breakpad/src/common/linux/file_id.cc",
    "third_party/breakpad/src/common/linux/guid_creator.cc",
    "third_party/breakpad/src/common/linux/linux_libc_support.cc",
    "third_party/breakpad/src/common/linux/memory_mapped_file.cc",
    "third_party/breakpad/src/common/linux/safe_readlink.cc",
    "third_party/breakpad/src/common/string_conversion.cc",
];

fn autocxx_include_paths() -> Vec<String> {
    // The order of these paths is important to prevent the following error:
    //
    //   <cstdint> tried including <stdint.h> but didn't find libc++'s <stdint.h> header. This
    //   usually means that your header search paths are not configured properly. The header search
    //   paths should contain the C++ Standard Library headers before any C Standard Library, and
    //   you are probably using compiler flags that make that not be the case.
    //
    let target = std::env::var("TARGET").unwrap_or_default();
    let sysroot = std::env::var(format!("SYSROOT_{target}")).unwrap_or_default();
    let sysroot_target = std::env::var(format!("SYSROOT_TARGET_{target}")).unwrap_or_default();
    vec![
        "src/cplusplus".to_string(),
        format!("{sysroot}/usr/include/c++/v1"),
        format!("{sysroot}/usr/include"),
        format!("{sysroot}/usr/include/{sysroot_target}"),
    ]
}

fn configure_build_android(build: &mut autocxx_engine::BuilderBuild) {
    build
        .files(THIRD_PARTY_BREAKPAD_UNIX)
        .files([
            "src/cplusplus/breakpad_unix.cc",
            "src/cplusplus/utility.cc",
            "src/cplusplus/utility_unix.cc",
        ])
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .pic(true);
}

fn configure_build_ios(build: &mut autocxx_engine::BuilderBuild) {
    build
        .files(THIRD_PARTY_BREAKPAD_APPLE)
        .files([
            "src/cplusplus/breakpad_apple.cc",
            "src/cplusplus/utility.cc",
            "third_party/breakpad/src/client/ios/exception_handler_no_mach.cc",
        ])
        .flag_if_supported("-Wno-mismatched-tags")
        .flag_if_supported("-Wno-unused-parameter");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

fn configure_build_linux(build: &mut autocxx_engine::BuilderBuild) {
    build
        .files(THIRD_PARTY_BREAKPAD_UNIX)
        .files([
            "src/cplusplus/breakpad_unix.cc",
            "src/cplusplus/utility.cc",
            "src/cplusplus/utility_unix.cc",
        ])
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-maybe-uninitialized")
        .flag_if_supported("-Wno-missing-field-initializers")
        .pic(true);
}

fn configure_build_macos(build: &mut autocxx_engine::BuilderBuild) {
    build
        .files(THIRD_PARTY_BREAKPAD_APPLE)
        .files([
            "src/cplusplus/breakpad_apple.cc",
            "src/cplusplus/utility.cc",
            "third_party/breakpad/src/client/mac/crash_generation/crash_generation_client.cc",
            "third_party/breakpad/src/client/mac/handler/exception_handler.cc",
            "third_party/breakpad/src/common/mac/MachIPC.mm",
        ])
        .flag_if_supported("-Wno-deprecated-copy-with-user-provided-copy")
        .flag_if_supported("-Wno-unused-parameter");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

fn configure_build_windows(build: &mut autocxx_engine::BuilderBuild) {
    build
        .define("NOMINMAX", None)
        .define("UNICODE", None)
        .define("WIN32_LEAN_AND_MEAN", None)
        .define("_UNICODE", None)
        .files([
            "src/cplusplus/breakpad_windows.cc",
            "src/cplusplus/utility.cc",
            "src/cplusplus/utility_windows.cc",
            "third_party/breakpad/src/client/windows/crash_generation/crash_generation_client.cc",
            "third_party/breakpad/src/client/windows/handler/exception_handler.cc",
            "third_party/breakpad/src/common/windows/guid_string.cc",
        ]);
}

fn configure_build(build: &mut autocxx_engine::BuilderBuild) {
    match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "android" => configure_build_android(build),
        "ios" => configure_build_ios(build),
        "linux" => configure_build_linux(build),
        "macos" => configure_build_macos(build),
        "windows" => configure_build_windows(build),
        target_os => panic!("Unsupported Platform: {target_os}"),
    }
}

fn main() -> miette::Result<()> {
    let mut build = autocxx_build::Builder::new("src/lib.rs", autocxx_include_paths())
        .auto_allowlist(true)
        .build()
        .into_diagnostic()?;
    configure_build(&mut build);

    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", None);

    build
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .includes(["./", "third_party/breakpad/src/"])
        .compile("breakpad");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=third_party/");

    Ok(())
}
