use std::fmt::{Debug, Formatter};

use pr47::builtins::closure::Closure;
use pr47::data::exception::UncheckedException;
use pr47::data::wrapper::{OWN_INFO_READ_MASK, OwnershipInfo};
use pr47::ffi::FFIException;
use rhai::FnPtr;

use crate::pr47::dangerous_clone_closure;

pub enum ServerRequestHandler {
    Pr47Function(Closure),
    RhaiFunction(FnPtr),
    StaticFile { file_path: String, content_type: Option<String> }
}

impl Debug for ServerRequestHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerRequestHandler::Pr47Function(closure) => {
                write!(f, "Pr47Function(func_id = {}", closure.func_id)
            },
            ServerRequestHandler::RhaiFunction(fn_ptr) => {
                f.debug_tuple("RhaiFunction").field(&fn_ptr).finish()
            },
            ServerRequestHandler::StaticFile { file_path, content_type } => {
                write!(f, "StaticFile({:?}, {:?})", file_path, content_type)
            }
        }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    listen_addr: String,
    port: u16,

    handlers: Vec<(String, ServerRequestHandler)>
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            listen_addr: "127.0.0.1".into(),
            port: 8080,
            handlers: vec![]
        }
    }

    pub fn add_pr47_handler(&mut self, handler_path: String, input_closure: &mut Closure)
        -> Result<(), FFIException>
    {
        let mut closure = unsafe { dangerous_clone_closure(input_closure) };

        for captured_value in closure.capture.iter_mut() {
            unsafe {
                if !captured_value.ownership_info_norm().is_readable() {
                    return Err(FFIException::Right(UncheckedException::OwnershipCheckFailure {
                        object: captured_value.clone(),
                        expected_mask: OWN_INFO_READ_MASK
                    }))
                }
                captured_value.set_ownership_info(OwnershipInfo::SharedToRust);
            }
        }
        self.handlers.push((handler_path, ServerRequestHandler::Pr47Function(closure)));
        Ok(())
    }

    pub fn add_rhai_handler(&mut self, handler_path: String, fn_ptr: FnPtr) {
        self.handlers.push((handler_path, ServerRequestHandler::RhaiFunction(fn_ptr)));
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}
