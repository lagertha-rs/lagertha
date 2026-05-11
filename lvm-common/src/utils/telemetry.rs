use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() {
    let _ = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")),
        )
        .without_time()
        .with_target(true)
        .compact()
        .try_init();
}
