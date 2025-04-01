//! this module contains the configuration options for the application
//!

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput{
    Stdout,
    Stderr,
    File(String),
}

pub struct Logging{
    enabled:bool,
    level:LogLevel,
    destination:LogOutput,
}

impl Logging{
    pub fn new()->Self{
        Self{
            enabled:false,
            level:LogLevel::Info,
            destination:LogOutput::Stdout,
        }

    }
}