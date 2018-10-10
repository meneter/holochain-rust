//! This logger is the logger that's attached to each Holochain application
//! which is separate from standard logging via the log crate warn! info! debug! logging that
//! gets emitted globaly from the container.

use chrono::Local;

/// trait that defines the logging functionality that holochain_core requires
pub trait Logger: Send {
    fn log(&mut self, msg: String);
}

#[derive(Clone)]
pub struct SimpleLogger {
    // log: Vec<String>,
}

#[cfg_attr(tarpaulin, skip)]
impl Logger for SimpleLogger {
    fn log(&mut self, msg: String) {
        let date = Local::now();
        println!("{}:{}", date.format("%Y-%m-%d %H:%M:%S"), msg);
    }
    // fn new() -> SimpleLogger {
    //      SimpleLogger {}
    // }
}
