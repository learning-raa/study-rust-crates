use anyhow::Result;

pub fn dive_in() -> anyhow::Result<()> {
    println!("!!!!");
    log::info!("314\n15 {}", 44);
    println!("!8888888!!!");

    Ok(())
}

//  //  //  //  //  //  //  //
use log::{Level, Metadata, Record};
use std::sync::RwLock;

enum RaaLoggerMode {
    Empty,
    File(String),
}

static RAA_LOGGER: RaaLogger = RaaLogger {
    mode: RwLock::new(RaaLoggerMode::Empty),
};
struct RaaLogger {
    mode: RwLock<RaaLoggerMode>,
}

#[allow(dead_code)]
impl RaaLogger {
    fn init() -> Result<&'static Self> {
        if let Err(ee) = log::set_logger(&RAA_LOGGER) {
            return Err(anyhow::anyhow!(ee.to_string()));
        }
        log::set_max_level(log::LevelFilter::Info);

        Ok(&RAA_LOGGER)
    }
    fn set_pts(&self, name: &str) -> &Self {
        let mode: &mut RaaLoggerMode = &mut self.mode.write().expect("RaaLog::set_pts: RwLock");
        *mode = RaaLoggerMode::File(name.to_string());

        self
    }
}

impl log::Log for RaaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mode: &RaaLoggerMode = &self.mode.read().expect("RaaLog.enabled: RwLock");
        match mode {
            RaaLoggerMode::Empty => return false,
            RaaLoggerMode::File(_) => metadata.level() <= Level::Info,
        }
    }

    fn log(&self, record: &Record) {
        let mode: &RaaLoggerMode = &self.mode.read().expect("RaaLog.log: RwLock");
        match mode {
            RaaLoggerMode::Empty => return,
            RaaLoggerMode::File(pts) => {
                if self.enabled(record.metadata()) {
                    println!(
                        "{}: {}:{} - {}",
                        pts,
                        record.level(),
                        record.target(),
                        record.args()
                    );
                }
            }
        }
    }

    fn flush(&self) {}
}

//  //  //  //  //  //  //  //
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_init() -> Result<()> {
        let _ = RaaLogger::init();
        Ok(())
    }

    #[test]
    #[should_panic]
    fn double_init() {
        let _ = RaaLogger::init();
        RaaLogger::init().expect("double inited Logger");
    }

    #[test]
    fn set_pts() -> Result<()> {
        let _ = RaaLogger::init();
        RAA_LOGGER.set_pts("uni-pts");
        log::info!("test info_msg");

        Ok(())
    }
}
