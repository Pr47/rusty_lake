pub mod config;

use pr47::builtins::closure::Closure;

pub unsafe fn dangerous_clone_closure(closure: &Closure) -> Closure {
    Closure {
        capture: closure.capture.clone(),
        func_id: closure.func_id
    }
}
