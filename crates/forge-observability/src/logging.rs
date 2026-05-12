use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging(log_level: &str, json_format: bool) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    if json_format {
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .json()
            .flatten_event(true)
            .with_target(true)
            .with_thread_ids(true)
            .finish();
        tracing::subscriber::set_global_default(subscriber).ok();
    } else {
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(false)
            .finish();
        tracing::subscriber::set_global_default(subscriber).ok();
    }
}
