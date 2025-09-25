use std::path::Path;
use tracing::{Level, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

/// Initialize the logging system
pub fn init_logging(log_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = log_dir.as_ref();
    
    // Create log directory if it doesn't exist
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }
    
    // Set up file appender for logs
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        "agent-friend.log",
    );
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Store the guard in a static variable to keep it alive
    // This prevents the file handle from being closed
    static mut GUARD: Option<tracing_appender::non_blocking::WorkerGuard> = None;
    unsafe {
        GUARD = Some(_guard);
    }
    
    // Set up the subscriber with both terminal and file output
    tracing_subscriber::registry()
        .with(
            fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        )
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
        )
        .init();
    
    info!("Logging initialized");
    Ok(())
}

/// Log an error and return it
pub fn log_error<E: std::fmt::Display>(error: E) -> E {
    tracing::error!("{}", error);
    error
}
