use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

#[cfg(feature = "with-pr47")] use pr47::builtins::closure::Closure;
#[cfg(feature = "with-pr47")] use pr47::data::exception::UncheckedException;
#[cfg(feature = "with-pr47")] use pr47::data::wrapper::{OWN_INFO_READ_MASK, OwnershipInfo};
#[cfg(feature = "with-pr47")] use pr47::ffi::FFIException;
#[cfg(feature = "with-pr47")] use crate::pr47_libs::dangerous_clone_closure;

#[cfg(feature= "with-rhai")] use std::sync::{Arc, Mutex, MutexGuard};
#[cfg(feature = "with-rhai")] use rhai::{Dynamic, FnPtr};

pub enum ServerRequestHandler {
    #[cfg(feature = "with-pr47")]
    Pr47Function(Closure),
    #[cfg(feature = "with-rhai")]
    RhaiFunction(FnPtr),
    StaticFile { file_path: String, content_type: Option<String> }
}

impl Debug for ServerRequestHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "with-pr47")]
            ServerRequestHandler::Pr47Function(closure) => {
                write!(f, "Pr47Function(func_id = {}", closure.func_id)
            },
            #[cfg(feature = "with-rhai")]
            ServerRequestHandler::RhaiFunction(fn_ptr) => {
                f.debug_tuple("RhaiFunction").field(&fn_ptr).finish()
            },
            ServerRequestHandler::StaticFile { file_path, content_type } => {
                write!(f, "StaticFile({:?}, {:?})", file_path, content_type)
            }
        }
    }
}

#[cfg(feature = "with-pr47")] unsafe impl Send for ServerRequestHandler {}
#[cfg(feature = "with-pr47")] unsafe impl Sync for ServerRequestHandler {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub port: u16,

    #[cfg(feature = "with-pr47")]
    pub pr47_folder: String,

    #[cfg(feature = "with-rhai")]
    pub rhai_folder: String,

    pub static_folder: String,

    #[cfg_attr(
        any(feature = "with-pr47", feature = "with-rhai"),
        serde(skip_deserializing, skip_serializing)
    )]
    pub handlers: Vec<(String, ServerRequestHandler)>
}

impl ServerConfig {
    #[cfg(feature = "with-pr47")]
    pub fn add_pr47_handler(&mut self, handler_path: &str, input_closure: &Closure)
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
        self.handlers.push((handler_path.to_string(), ServerRequestHandler::Pr47Function(closure)));
        Ok(())
    }

    #[cfg(feature = "with-pr47")]
    pub fn add_static_file_handler_pr47(
        &mut self,
        handler_path: &str,
        file_path: &str,
        content_type: Option<&str>
    ) {
        self.handlers.push((
            handler_path.to_string(),
            ServerRequestHandler::StaticFile {
                file_path: file_path.to_string(),
                content_type: content_type.or_else(|| {
                    mime_guess::from_path(file_path).first_raw()
                }).map(ToString::to_string)
            }
        ));
    }

    #[cfg(feature = "with-rhai")]
    pub fn add_rhai_handler(&mut self, handler_path: String, fn_ptr: FnPtr) {
        self.handlers.push((handler_path, ServerRequestHandler::RhaiFunction(fn_ptr)));
    }

    #[cfg(feature = "with-rhai")]
    pub fn add_static_file_handler_rhai(
        &mut self,
        handler_path: String,
        file_path: String,
        content_type: Dynamic
    ) {
        match content_type.into_immutable_string() {
            Ok(immutable) => {
                self.handlers.push((handler_path, ServerRequestHandler::StaticFile {
                    file_path,
                    content_type: Some(immutable.to_string())
                }));
            },
            Err(_) => {
                self.handlers.push((handler_path, ServerRequestHandler::StaticFile {
                    file_path,
                    content_type: None
                }));
            }
        }
    }
}

#[cfg(feature = "with-rhai")]
pub fn add_rhai_handler(this: Arc<Mutex<ServerConfig>>, handler_path: String, fn_ptr: FnPtr) {
    let mut this: MutexGuard<ServerConfig> = this.lock().unwrap();
    this.add_rhai_handler(handler_path, fn_ptr);
}
