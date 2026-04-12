// MongoDB & Jaeger instances for testing is initialized here
use std::time::{SystemTime, UNIX_EPOCH};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::{GenericImage, runners::AsyncRunner};

#[allow(dead_code)]
pub struct TestEnv {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub otlp_endpoint: String,
}

static TEST_ENV: tokio::sync::OnceCell<TestEnv> = tokio::sync::OnceCell::const_new();

pub async fn setup_test_env() -> &'static TestEnv {
    TEST_ENV
        .get_or_init(|| async {
            let mongodb_container = GenericImage::new("mongo", "8.3-rc-noble")
                .with_exposed_port(27017.tcp())
                .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
                .pull_image()
                .await
                .expect("failed to pull mongodb image")
                .start()
                .await
                .expect("failed to start mongodb container");

            let mongodb_port = mongodb_container
                .get_host_port_ipv4(27017.tcp())
                .await
                .expect("failed to read mongodb mapped port");
            let mongodb_uri = format!("mongodb://127.0.0.1:{mongodb_port}");

            let jaeger_container = GenericImage::new("jaegertracing/jaeger", "2.14.1")
                .with_exposed_port(4317.tcp())
                .with_wait_for(WaitFor::seconds(10))
                .pull_image()
                .await
                .expect("failed to pull jaeger image")
                .start()
                .await
                .expect("failed to start jaeger container");

            let jaeger_port = jaeger_container
                .get_host_port_ipv4(4317.tcp())
                .await
                .expect("failed to read jaeger mapped port");
            let otlp_endpoint = format!("http://127.0.0.1:{jaeger_port}");

            // Keep containers alive for the entire test process lifetime.
            std::mem::forget(mongodb_container);
            std::mem::forget(jaeger_container);

            // retrieve the system time, & append to DB name so that each DB instance name is different
            let test_run_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_millis();
            let mongodb_database = format!("clinic_booking_api_test_{test_run_suffix}");

            // Safe in this test harness because environment is initialized once, before test app init.
            unsafe {
                std::env::set_var("MONGODB_URI", &mongodb_uri);
                std::env::set_var("MONGODB_DATABASE", &mongodb_database);
                std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", &otlp_endpoint);
            }

            TestEnv {
                mongodb_uri,
                mongodb_database,
                otlp_endpoint,
            }
        })
        .await
}
