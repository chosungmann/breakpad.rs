# breakpad.rs

[![https://github.com/chosungmann/breakpad.rs/actions/workflows/continuous_integration.yaml/badge.svg](https://github.com/chosungmann/breakpad.rs/actions/workflows/continuous_integration.yaml/badge.svg)](https://github.com/chosungmann/breakpad.rs/actions/workflows/continuous_integration.yaml)

`breakpad.rs` is a Rust crate that enables the use of [Breakpad](https://chromium.googlesource.com/breakpad/breakpad) in Rust applications. When a crash occurs, it generates a minidump and notifies related information through a designated delegate.

## Example

```rust
struct Delegate;

impl breakpad_rs::ExceptionHandlerDelegate for Delegate {
    fn did_write_minidump(&self, working_path: String, minidump_id: String) {
        println!("[did_write_minidump] working_path={working_path} minidump_id={minidump_id}");
    }

    fn get_working_path(&self) -> String {
        println!("[get_working_path] return=.");
        String::from(".")
    }

    fn should_write_minidump(&self) -> bool {
        println!("[should_write_minidump] return=true");
        true
    }
}

fn main() {
    let _breakpad = breakpad_rs::Breakpad::new(Some(Box::new(Delegate)));
    unsafe {
        let null: *mut u8 = std::ptr::null_mut();
        *null = 42;
    }
}
```

## References

* https://chromium.googlesource.com/breakpad/breakpad/
* https://chromium.googlesource.com/breakpad/breakpad/+/refs/heads/main/docs/
