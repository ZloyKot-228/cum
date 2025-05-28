use core::Core;
use std::env;

pub mod core;
pub mod diagnostics;
pub mod drivers;
pub mod errors;
pub mod logger;
pub mod meta;
pub mod parsing;
pub mod planning;
pub mod test_utils;

fn main() {
    let mut core = Core::default();

    core.parse_config();
    core.verify_diagnostics();

    core.parse_args(env::args().collect());
    core.verify_diagnostics();

    core.print_info();

    core.print_all_diagnostics();
}
