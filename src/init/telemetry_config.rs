use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use opentelemetry::sdk::trace::TracerProvider as TProvider;
use opentelemetry::trace::TracerProvider;
use opentelemetry_stackdriver::{GcpAuthorizer, StackDriverExporter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};

pub async fn init(app_name: String) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let authorizer = GcpAuthorizer::new().await.unwrap();
    let (exporter, tasks) = StackDriverExporter::builder()
        .build(authorizer)
        .await
        .unwrap();

    let _spawned = tokio::spawn(tasks);

    let tracer = TProvider::builder()
        .with_batch_exporter(exporter, TokioCurrentThread)
        .build()
        .tracer(app_name.clone());
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(telemetry)
        ;
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}
