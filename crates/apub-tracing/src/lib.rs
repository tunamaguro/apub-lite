use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub struct TracingGuard;

/// `tracing`を初期化する
pub fn init() -> TracingGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        };
        level.into()
    });

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true);

    #[cfg(debug_assertions)]
    let fmt_layer = fmt_layer.with_ansi(true).pretty();
    #[cfg(not(debug_assertions))]
    let fmt_layer = fmt_layer.json();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    TracingGuard
}
