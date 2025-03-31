mod metrics;
mod network;

#[tokio::main]
async fn main() {
    println!("Starting Prometheus Network Metrics Exporter...");

    // Start the metrics server
    metrics::start_metrics_server().await;
}
