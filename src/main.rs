pub mod config;

#[cfg(feature = "with-pr47")] pub mod pr47_libs;
#[cfg(feature = "with-rhai")] pub mod rhai_libs;

use std::fs::read_to_string;

use crate::config::ServerConfig;

#[cfg(feature = "with-rhai")] use rhai::module_resolvers::FileModuleResolver;
#[cfg(feature = "with-rhai")] use rhai::Engine;
#[cfg(feature = "with-rhai")] use rhai::AST;
#[cfg(feature = "with-rhai")] use rhai::Scope;

fn main() {
    let config: String = read_to_string("./config.json").expect("error reading config.json");
    let config: ServerConfig = serde_json::from_str(&config).expect("error parsing config.json");

    #[cfg(feature = "with-rhai")] {
        let index_file: String = format!("{}/{}", config.rhai_folder, "index.rhai");

        let mut resolver: FileModuleResolver = FileModuleResolver::new();
        resolver.set_base_path(config.rhai_folder);
        resolver.set_extension("rhai");

        let mut engine: Engine = Engine::new();
        engine.set_module_resolver(resolver);

        let ast: AST = engine.compile_file(index_file.into()).expect("failed compiling index.rhai");

        let mut scope = Scope::new();
        engine.call_fn(&mut scope, &ast, "build_config", (config,))
    }
}
