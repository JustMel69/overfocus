use std::{sync::{Mutex, OnceLock}, time::Instant};

// · · ·  Macro Definitions  · · · //

#[macro_export]
macro_rules! notify_short {
    ($expr:expr) => {
        $crate::logger::Logger::notify(format!("{}", $expr), $crate::logger::Duration::Short);
    };
}

#[macro_export]
macro_rules! notify_long {
    ($expr:expr) => {
        $crate::logger::Logger::notify(format!("{}", $expr), $crate::logger::Duration::Long);
    };
}

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
    ($expr:expr, else => $else:expr) => {
        match $expr {
            Result::Ok(x) => x,
            Result::Err(e) => {
                $crate::log_err!(e);
                $else
            },
        }
    };
    
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

#[derive(Clone)]
pub struct NotificationData(pub String, pub Duration);

#[derive(Clone)]
pub enum Duration { Short, Long }

pub struct Logger {
    logs: Vec<LogData>,
    notification: Option<NotificationData>,
    start: Instant,
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();


impl Logger {
    fn new() -> Mutex<Self> {
        Mutex::new(Self { logs: Vec::new(), start: Instant::now(), notification: None })
    }
    
    pub fn init() {
        _ = LOGGER.set(Self::new());
    }
    
    pub fn log(text: String, kind: LogKind) {
        let mut logger = LOGGER.get_or_init(Self::new).lock().unwrap();
        let elapsed = logger.start.elapsed();

        logger.logs.push(LogData(text, kind, elapsed.as_secs()));
    }

    pub fn notify(text: String, duration: Duration) {
        { // Scope here so there's no lock
            let mut logger = LOGGER.get_or_init(Self::new).lock().unwrap();
            logger.notification = Some(NotificationData(text.clone(), duration))
        }

        Logger::log(text, LogKind::Info);
    }

    pub fn last() -> Option<LogData> {
        LOGGER.get_or_init(Self::new).lock().unwrap().logs.last().map(|x| x.clone())
    }

    pub fn consume_notification() -> Option<NotificationData> {
        let mut logger = LOGGER.get_or_init(Self::new).lock().unwrap();

        let tmp = logger.notification.clone();
        logger.notification = None;
        return tmp;
    }
}