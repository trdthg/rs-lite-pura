use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

use log::{Level, Log, Metadata, Record};
use serde::Serialize;

use crate::error::{Error, Result};

lazy_static::lazy_static! {
    static ref CONTAINERLOGGER: ContainerLogger = ContainerLogger {
        inner: Mutex::new(None),
    };
}
pub struct ContainerLogger {
    inner: Mutex<Option<Logger>>,
}

pub struct Logger {
    path: String,
    max_level: Level,
}

impl ContainerLogger {
    pub fn init(path: &String, max_level: Level) -> Result<()> {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)?;
        *CONTAINERLOGGER.inner.lock().unwrap() = Some(Logger {
            path: path.clone(),
            max_level,
        });

        log::set_logger(&*CONTAINERLOGGER).map_err(|e| Error::LogError(e.to_string()))?;
        log::set_max_level(max_level.to_level_filter());
        Ok(())
    }
}

#[derive(Serialize)]
pub struct LogEntity {
    pub level: String,
    pub msg: String,
    pub time: String,
}

impl Log for ContainerLogger {
    // 判断
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.inner.lock().unwrap().as_ref().unwrap().max_level
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
        if !self.enabled(record.metadata()) {
            return;
        }
        let log_entity = LogEntity {
            level: record.level().to_string(),
            msg: record.args().to_string(),
            time: chrono::Local::now().to_rfc3339(),
        };

        if let Some(logger) = self.inner.lock().unwrap().as_ref() {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&logger.path)
                .unwrap();
            let log_json = serde_json::to_string(&log_entity).unwrap();
            let _ = file.write(format!("{}\n", log_json).as_bytes()).unwrap();
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use log::{debug, error, info, trace, warn, Level};

    use super::ContainerLogger;

    fn read_file(path: &str) -> String {
        let mut log_file = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open(path)
            .unwrap();
        let mut logs = String::new();
        log_file.read_to_string(&mut logs).unwrap();
        logs
    }

    #[test]
    fn log() {
        let _ = ContainerLogger::init(&"log.txt".to_string(), Level::Info).unwrap();
        warn!("warn");
        error!("error");
        info!("info");
        debug!("debug");
        trace!("trace");

        let logs = read_file("log.txt");

        assert!(logs.contains("warn"));
        assert!(logs.contains("error"));
        assert!(logs.contains("info"));
        assert!(!logs.contains("debug"));
        assert!(!logs.contains("trace"));

        std::fs::remove_file("log.txt").unwrap();
    }
}
