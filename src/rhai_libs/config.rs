use std::sync::{Arc, Mutex, MutexGuard};
use rhai::{Dynamic, FnPtr};

use crate::ServerConfig;
use crate::config::ServerRequestHandler;

#[cfg(feature = "with-rhai")]
#[allow(non_snake_case)]
pub fn RHAI_BIND_add_rhai_handler(
    this: Arc<Mutex<ServerConfig>>,
    handler_path: String,
    fn_ptr: FnPtr
) {
    let mut this: MutexGuard<ServerConfig> = this.lock().unwrap();
    this.handlers.push((handler_path, ServerRequestHandler::RhaiFunction(fn_ptr)));
}

#[cfg(feature = "with-rhai")]
#[allow(non_snake_case)]
pub fn RHAI_BIND_add_static_file_handler(
    this: Arc<Mutex<ServerConfig>>,
    handler_path: String,
    file_path: String,
    content_type: Dynamic
) {
    let mut this: MutexGuard<ServerConfig> = this.lock().unwrap();
    match content_type.into_immutable_string() {
        Ok(immutable) => {
            this.handlers.push((handler_path, ServerRequestHandler::StaticFile {
                file_path,
                content_type: Some(immutable.to_string())
            }));
        },
        Err(_) => {
            this.handlers.push((handler_path, ServerRequestHandler::StaticFile {
                file_path,
                content_type: None
            }));
        }
    }
}
