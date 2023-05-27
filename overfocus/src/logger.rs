use std::{sync::{Mutex, OnceLock}, time::Instant};

// · · ·  Macro Definitions  · · · //

#[macro_export]
macro_rules! log_info {
    ($expr:expr) => {
        $crate::logger::Logger::log(format!("{}", $expr), $crate::logger::LogKind::Info);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($expr:expr) => {
        $crate::logger::Logger::log(format!("{}", $expr), $crate::logger::LogKind::Warn);
    };
}

#[macro_export]
macro_rules! log_err {
    ($expr:expr) => {
        $crate::logger::Logger::log(format!("{}", $expr), $crate::logger::LogKind::Err);
    };
}

#[macro_export]
macro_rules! unwrap_err {
    ($expr:expr) => {
        if let Result::Err(e) = $expr {
            $crate::log_err!(e);
        }
    };
}



// · · ·  Logger Definition  · · · //

#[derive(Clone)]
pub struct LogData(pub String, pub LogKind, pub u64);

#[derive(Clone)]
pub enum LogKind { Info, Warn, Err }

pub struct Logger {
    logs: Vec<LogData>,
    start: Instant,
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();


impl Logger {
    fn new() -> Mutex<Self> {
        Mutex::new(Self { logs: Vec::new(), start: Instant::now() })
    }
    
    pub fn init() {
        _ = LOGGER.set(Self::new());
    }
    
    pub fn log(text: String, kind: LogKind) {
        let mut logger = LOGGER.get_or_init(Self::new).lock().unwrap();
        let elapsed = logger.start.elapsed();

        logger.logs.push(LogData(text, kind, elapsed.as_secs()));
    }

    pub fn last() -> Option<LogData> {
        LOGGER.get_or_init(Self::new).lock().unwrap().logs.last().map(|x| x.clone())
    }
}