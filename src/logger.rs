use chrono::Local;
use colored::Colorize;

pub struct Logger;

impl Logger {
    #[inline]
    pub fn info(msg: String) {
        let lable = "INFO".bright_blue();
        if cfg!(debug_assertions) {
            println!("[{}] [{lable}] {msg}", Self::time_str());
        } else {
            println!("[{lable}] {msg}");
        }
    }

    #[inline]
    pub fn warning(msg: String) {
        let lable = "WARNING".yellow();
        if cfg!(debug_assertions) {
            println!("[{}] [{lable}] {msg}", Self::time_str());
        } else {
            println!("[{lable}] {msg}");
        }
    }

    #[inline]
    pub fn error(msg: String) {
        let lable = "ERROR".red();
        if cfg!(debug_assertions) {
            println!("[{}] [{lable}] {msg}", Self::time_str());
        } else {
            println!("[{lable}] {msg}");
        }
    }

    #[inline]
    fn time_str() -> String {
        Local::now().format("%H:%M:%S%.3f").to_string()
    }
}
