use chrono::Local;

pub struct Logger;

impl Logger {
    pub fn info(msg: String) {
        if cfg!(debug_assertions) {
            println!("[{}] [INFO] {msg}", Self::time_str());
        } else {
            println!("[INFO] {msg}");
        }
    }

    pub fn warning(msg: String) {
        if cfg!(debug_assertions) {
            println!("[{}] [WARNING] {msg}", Self::time_str());
        } else {
            println!("[WARNING] {msg}");
        }
    }

    pub fn error(msg: String) {
        if cfg!(debug_assertions) {
            println!("[{}] [ERROR] {msg}", Self::time_str());
        } else {
            println!("[ERROR] {msg}");
        }
    }

    fn time_str() -> String {
        Local::now().format("%H:%M:%S%.3f").to_string()
    }
}
