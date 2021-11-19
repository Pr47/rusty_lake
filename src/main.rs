pub mod config;

#[cfg(feature = "with-pr47")] pub mod pr47_libs;
#[cfg(feature = "with-rhai")] pub mod rhai_libs;

use std::fs::read_to_string;

use crate::config::{add_rhai_handler, ServerConfig};

#[cfg(feature = "with-rhai")] use rhai::module_resolvers::FileModuleResolver;
#[cfg(feature = "with-rhai")] use rhai::Engine;
#[cfg(feature = "with-rhai")] use rhai::AST;
#[cfg(feature = "with-rhai")] use rhai::Scope;

fn main() {
    let config: String = read_to_string("./config.json").expect("error reading config.json");
    let config0: ServerConfig = serde_json::from_str(&config).expect("error parsing config.json");

    #[cfg(feature = "with-rhai")]
    let _config1: ServerConfig = {
        use std::sync::{Arc, Mutex};

        let index_file: String = format!("{}/{}", config0.rhai_folder, "index.rhai");

        let mut resolver: FileModuleResolver = FileModuleResolver::new();
        resolver.set_base_path(&config0.rhai_folder);
        resolver.set_extension("rhai");

        let mut engine: Engine = Engine::new();
        engine.set_module_resolver(resolver);
        engine.register_type::<std::sync::Arc<std::sync::Mutex<ServerConfig>>>()
            .register_fn("add_rhai_handler", add_rhai_handler);


        let ast: AST = engine.compile_file(index_file.into()).expect("failed compiling index.rhai");

        let mut scope = Scope::new();
        let config0: Arc<Mutex<ServerConfig>> = Arc::new(Mutex::new(config0));
        let _: () = engine.call_fn(&mut scope, &ast, "build_config", (config0.clone(),))
            .expect("failed initializing");

        Arc::try_unwrap(config0).unwrap().into_inner().unwrap()
    };
}
