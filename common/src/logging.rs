use config::Config;
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace as sdktrace};
use std::env;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing(service_name: &str) {
    // for prod, change the file name on line 13 to point to live config e.g. settings_prod.yaml
    let logging_config = Config::builder()
        .add_source(config::File::with_name("settings_dev.toml").required(false))
        .build()
        .expect("Unable to read tracing endpoint.");

    let otlp_endpoint: Option<String> = match env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        Ok(v) => Some(v),
        Err(_) => logging_config
            .get_string("tracing.OTEL_EXPORTER_OTLP_ENDPOINT")
            .ok(),
    };

    if let Some(endpoint) = otlp_endpoint {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()
            .expect("Failed to build exporter");

        let provider = sdktrace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(
                Resource::builder_empty()
                    .with_attributes(vec![KeyValue::new(
                        "service.name",
                        service_name.to_string(),
                    )])
                    .build(),
            )
            .build();

        let tracer = provider.tracer("clinic_booking");
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
            .with(telemetry)
            .with(tracing_subscriber::fmt::layer())
            .try_init()
            .expect("Failed to init tracing subsystem.");
    } else {
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
            .with(tracing_subscriber::fmt::layer())
            .try_init()
            .expect("Failed to init tracing subsystem.");
    }
}
