use blog_application::interfaces::ILogger;

pub struct SimpleLogger;

impl ILogger for SimpleLogger {
    fn debug(&self, message: &str) {
        println!("[DEBUG] {}", message);
    }

    fn info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    fn warn(&self, message: &str) {
        println!("[WARN] {}", message);
    }

    fn error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }
}
