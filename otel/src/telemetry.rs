use opentelemetry_prometheus::PrometheusExporter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_telemetry(_quickwit_endpoint: &str) -> PrometheusExporter {
    let prometheus_exporter = opentelemetry_prometheus::exporter()
        .with_registry(prometheus::default_registry().clone())
        .build()
        .expect("Failed to create Prometheus exporter");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    prometheus_exporter
}