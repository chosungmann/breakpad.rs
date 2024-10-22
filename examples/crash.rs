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
