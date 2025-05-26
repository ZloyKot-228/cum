use core::Core;

use logger::Logger;

pub mod core;
pub mod diagnostics;
pub mod errors;
pub mod logger;
pub mod parsing;

fn main() {
    let mut core = Core::default();

    core.parse_config();
    core.verify_diagnostics();

    let c = core.get_ctx_ref();
    Logger::info(c.config.to_string());

    core.print_diagnostics_final();
}
