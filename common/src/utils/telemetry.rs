use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() {
    let _ = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")),
        )
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .try_init();
}
