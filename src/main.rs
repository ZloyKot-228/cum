use core::Core;
use std::env;

use logger::Logger;

pub mod concurrency;
pub mod core;
pub mod diagnostics;
pub mod drivers;
pub mod errors;
pub mod execution;
pub mod logger;
pub mod meta;
pub mod parsing;
pub mod planning;
pub mod test_utils;

fn main() {
    if cfg!(debug_assertions) {
        test_utils::set_dir_to_tests();
    }

    let mut core = Core::default();

    core.parse_args(env::args().collect());
    core.verify_diagnostics();

    core.print_info();

    core.parse_config();
    core.verify_diagnostics();

    core.make_plan();
    core.verify_diagnostics();

    if cfg!(debug_assertions) {
        Logger::info(&format!("{:#?}", core.ctx().plan));
    }

    core.execute_plan();
    core.verify_diagnostics();

    core.print_all_diagnostics();
}
