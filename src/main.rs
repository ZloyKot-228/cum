use core::Core;
use std::env;

pub mod core;
pub mod diagnostics;
pub mod errors;
pub mod logger;
pub mod parsing;

fn main() {
    let mut core = Core::default();

    core.parse_config();
    core.verify_diagnostics();

    core.parse_args(env::args().collect());
    core.verify_diagnostics();

    core.print_all_diagnostics();
}
