#![doc = include_str!("../README.md")]

use autocxx::subclass::prelude::*;
use ffi::ToCppString;

autocxx::include_cpp! {
    #include "breakpad_exception_handler_delegate.h"
    safety!(unsafe)
    generate!("breakpad::ExceptionHandlerDelegate")
}

#[cxx::bridge(namespace = "breakpad")]
mod breakpad {
    unsafe extern "C++" {
        include!("breakpad.h");
        type ExceptionHandler;
        type ExceptionHandlerDelegate = crate::ffi::breakpad::ExceptionHandlerDelegate;
        fn CreateExceptionHandler(
            delegate: &ExceptionHandlerDelegate,
        ) -> UniquePtr<ExceptionHandler>;
    }
}

// TODO: This struct is for internal use only, so its visibility should be private or pub(crate).
// The reason for making this struct public is due to the limitation of autocxx.
#[subclass(superclass("breakpad::ExceptionHandlerDelegate"))]
pub struct ExceptionHandlerDelegateBridge {
    delegate: Box<dyn ExceptionHandlerDelegate>,
}

impl ExceptionHandlerDelegateBridge {
    fn new(
        delegate: Option<Box<dyn ExceptionHandlerDelegate>>,
    ) -> std::rc::Rc<std::cell::RefCell<Self>> {
        Self::new_rust_owned(Self {
            cpp_peer: Default::default(),
            delegate: match delegate {
                Some(delegate) => delegate,
                None => Box::new(DefaultExceptionHandlerDelegate),
            },
        })
    }
}

#[allow(non_snake_case)]
impl ffi::breakpad::ExceptionHandlerDelegate_methods for ExceptionHandlerDelegateBridge {
    fn DidWriteMinidump(&self, working_path: &cxx::CxxString, minidump_id: &cxx::CxxString) {
        self.delegate.did_write_minidump(working_path.to_string(), minidump_id.to_string());
    }

    fn GetWorkingPath(&self) -> cxx::UniquePtr<cxx::CxxString> {
        self.delegate.get_working_path().into_cpp()
    }

    fn ShouldWriteMinidump(&self) -> bool {
        self.delegate.should_write_minidump()
    }
}

pub trait ExceptionHandlerDelegate {
    fn did_write_minidump(&self, working_path: String, minidump_id: String) {
        log::debug!("[did_write_minidump] working_path={working_path} minidump_id={minidump_id}");
    }

    fn get_working_path(&self) -> String {
        log::debug!("[get_working_path] return=empty string");
        String::new()
    }

    fn should_write_minidump(&self) -> bool {
        log::debug!("[should_write_minidump] return=true");
        true
    }
}

struct DefaultExceptionHandlerDelegate;
impl ExceptionHandlerDelegate for DefaultExceptionHandlerDelegate {}

#[allow(dead_code)]
pub struct Breakpad {
    bridge: std::rc::Rc<std::cell::RefCell<ExceptionHandlerDelegateBridge>>,
    exception_handler: cxx::UniquePtr<breakpad::ExceptionHandler>,
}

impl Breakpad {
    pub fn new(delegate: Option<Box<dyn ExceptionHandlerDelegate>>) -> Self {
        let bridge = ExceptionHandlerDelegateBridge::new(delegate);
        let breakpad = Self {
            bridge: bridge.clone(),
            exception_handler: breakpad::CreateExceptionHandler(bridge.as_ref().borrow().as_ref()),
        };
        breakpad
    }
}
