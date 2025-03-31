use prometheus::{Encoder, TextEncoder, IntGauge, register_int_gauge};
use lazy_static::lazy_static;
use std::sync::Mutex;
use warp::Filter; // Ensure warp is imported

lazy_static! {
    pub static ref LATENCY_METRIC: IntGauge = register_int_gauge!(
        "network_latency",
        "Latency of network connections"
    )
    .unwrap();

    pub static ref PACKET_LOSS_METRIC: IntGauge = register_int_gauge!(
        "packet_loss",
        "Percentage of packet loss in the network"
    )
    .unwrap();

    pub static ref THROUGHPUT_METRIC: IntGauge = register_int_gauge!(
        "network_throughput",
        "Throughput of the network in bytes per second"
    )
    .unwrap();
}

pub fn collect_metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub async fn start_metrics_server() {
    let metrics_route = warp::path("metrics").map(|| {
        let metrics = collect_metrics();
        warp::reply::with_header(metrics, "Content-Type", "text/plain")
    });

    warp::serve(metrics_route).run(([0, 0, 0, 0], 9000)).await;
}
