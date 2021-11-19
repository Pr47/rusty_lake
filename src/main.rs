pub mod config;

#[cfg(feature = "with-pr47")] pub mod pr47_libs;
#[cfg(feature = "with-rhai")] pub mod rhai_libs;

use std::fs::read_to_string;

use config::ServerConfig;
use xjbutil::std_ext::ExpectSilentExt;

#[cfg(feature = "with-rhai")] use rhai::module_resolvers::FileModuleResolver;
#[cfg(feature = "with-rhai")] use rhai::Engine;
#[cfg(feature = "with-rhai")] use rhai::AST;
#[cfg(feature = "with-rhai")] use rhai::Scope;
#[cfg(feature = "with-rhai")] use crate::rhai_libs::config::{
    RHAI_BIND_add_rhai_handler,
    RHAI_BIND_add_static_file_handler
};

fn main() {
    let config: String =
        read_to_string("./config.json").expect_silent("error reading config.json");
    let config0: ServerConfig =
        serde_json::from_str(&config).unwrap();

    #[cfg(feature = "with-rhai")]
    let config1: ServerConfig = {
        use std::sync::{Arc, Mutex};

        let index_file: String = format!("{}/{}", config0.rhai_folder, "index.rhai");

        let mut resolver: FileModuleResolver = FileModuleResolver::new();
        resolver.set_base_path(&config0.rhai_folder);
        resolver.set_extension("rhai");

        let mut engine: Engine = Engine::new();
        engine.set_module_resolver(resolver);
        engine.register_type::<std::sync::Arc<std::sync::Mutex<ServerConfig>>>()
            .register_fn("add_rhai_handler", RHAI_BIND_add_rhai_handler)
            .register_fn("add_static_handler", RHAI_BIND_add_static_file_handler);

        let ast: AST = match engine.compile_file(index_file.into()) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("error compiling file index.rhai:\n{}", e);
                std::process::exit(-1)
            }
        };

        let mut scope = Scope::new();
        let config0: Arc<Mutex<ServerConfig>> = Arc::new(Mutex::new(config0));
        match engine.call_fn(&mut scope, &ast, "build_config", (config0.clone(),)) {
            Ok(()) => {},
            Err(e) => {
                eprintln!("error initializing with index.rhai:\n{}", e);
                std::process::exit(-1);
            }
        }

        Arc::try_unwrap(config0).unwrap().into_inner().unwrap()
    };
    #[cfg(not(feature = "with-rhai"))]
    let config1: ServerConfig = config0;

    // TODO: add Pr47 side initialization afterwards

    dbg!(config1);
}
