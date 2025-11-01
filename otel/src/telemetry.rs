use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry::{metrics, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_prometheus::PrometheusExporter;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::{self, RandomIdGenerator, Sampler};
use opentelemetry_sdk::{runtime, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_telemetry(quickwit_endpoint: &str) -> PrometheusExporter {
    let export_config = ExportConfig {
        endpoint: quickwit_endpoint.to_string(),
        ..ExportConfig::default()
    };

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", "todo-app"),
                    KeyValue::new("service.version", "0.1.0"),
                ])),
        )
        .install_batch(runtime::Tokio)
        .expect("Failed to install tracer");

    let meter_provider = SdkMeterProvider::builder().build();
    global::set_meter_provider(meter_provider.clone());

    let prometheus_exporter = opentelemetry_prometheus::exporter()
        .with_registry(prometheus::default_registry())
        .build()
        .expect("Failed to create Prometheus exporter");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(telemetry)
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    prometheus_exporter
}