use blog_application::interfaces::ILogger;
use log::{debug, error, info, warn};

pub struct SimpleLogger;

impl ILogger for SimpleLogger {
    fn debug(&self, message: &str) {
        debug!("{}", message);
    }

    fn info(&self, message: &str) {
        info!("{}", message);
    }

    fn warn(&self, message: &str) {
        warn!("{}", message);
    }

    fn error(&self, message: &str) {
        error!("{}", message);
    }
}
