use log::{error, info, warn};

// Function to initialize the logger
pub fn init_logger() {
    env_logger::init();
}

// Function to log an informational message
pub fn log_info(message: &str) {
    info!("{}", message);
}

// Function to log a warning message
pub fn log_warning(message: &str) {
    warn!("{}", message);
}

// Function to log an error message
pub fn log_error(message: &str) {
    error!("{}", message);
}


