//#![macro_use]
extern crate log;
use  log::{LogMetadata,LogLevel,LogRecord,SetLoggerError,LogLevelFilter};

pub struct Logger;

impl log::Log for Logger  {
    fn enabled(&self,metadata:&LogMetadata) -> bool {
       metadata.level() <= LogLevel::Debug
    }
    fn log(&self,record:&LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}",record.level(),record.args())
        }
    }
}

pub fn init() -> Result<(),SetLoggerError> {
    log::set_logger(|max_log_level|{
        max_log_level.set(LogLevelFilter::Debug);
        Box::new(Logger)
    })
}
