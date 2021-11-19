fn handle_simple_request(req, res) {
    print("hello, world!");
}

fn build_config(config) {
    config.add_rhai_handler("/api/rhai/simple", Fn("handle_simple_request"));
}
