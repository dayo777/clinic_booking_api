// MongoDB init for testing here. Telemetry is optional for tests.
use std::time::{SystemTime, UNIX_EPOCH};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};

#[allow(dead_code)]
pub struct TestEnv {
    pub mongodb_uri: String,                         // local uri instance
    pub mongodb_database: String,                    // database name
    pub otlp_endpoint: String,                       // endpoint for Jaeger
    mongodb_container: ContainerAsync<GenericImage>, // name of the container image for testing
}

static TEST_ENV: tokio::sync::OnceCell<TestEnv> = tokio::sync::OnceCell::const_new();

pub async fn setup_test_env() -> &'static TestEnv {
    TEST_ENV
        .get_or_init(|| async {
            let mongodb_container = GenericImage::new("mongo", "8.3-rc-noble")
                .with_exposed_port(27017.tcp())
                .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
                .with_env_var("GLIBC_TUNABLES", "glibc.pthread.rseq=1")
                .start()
                .await
                .expect("failed to start mongodb container.");

            let mongodb_port = mongodb_container
                .get_host_port_ipv4(27017.tcp())
                .await
                .expect("Failed to read mongodb mapped port");

            let mongodb_uri = format!("mongodb://127.0.0.1:{mongodb_port}");
            let otlp_endpoint = String::new();

            let test_run_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Issue getting the system time")
                .as_millis();

            let mongodb_database = format!("clinic_booking_api_test_{test_run_suffix}");

            unsafe {
                std::env::set_var("MONGODB_URI", &mongodb_uri);
                std::env::set_var("MONGODB_DATABASE", &mongodb_database);
                std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
            }

            TestEnv {
                mongodb_uri,
                mongodb_database,
                otlp_endpoint,
                mongodb_container,
            }
        })
        .await
}
