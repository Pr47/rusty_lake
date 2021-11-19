use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

#[cfg(feature = "with-pr47")] use pr47::builtins::closure::Closure;
#[cfg(feature = "with-rhai")] use rhai::FnPtr;

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
#[serde(rename_all = "camelCase")]
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
