# Prometheus Network Metrics Exporter

A simple network metrics exporter written in Rust. This tool collects network-related metrics from the system and exposes them via an HTTP endpoint in a format that Prometheus can scrape. The exported metrics include network interface statistics like bytes received and transmitted, packet loss, and more.

## Features

- Collects network metrics such as:
  - Total bytes received and transmitted by each network interface.
  - Network interface statistics (e.g., packets sent/received, errors).
- Exposes metrics via HTTP endpoint for Prometheus to scrape.
- Provides real-time network statistics.
- Built with Rust for performance and efficiency.

## Prerequisites

Before running the exporter, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/learn/get-started) (version 1.60.0 or later)
- [Prometheus](https://prometheus.io/docs/prometheus/latest/installation/) (for scraping the metrics)

## Installation

### 1. Clone the repository

First, clone the repository to your local machine:

```bash
[https://github.com/theraghav12/Prometheus_Network_Metrics_Exporter]
cd prometheus-network-metrics-exporter
```

### 2. Build the project

To build the project, run the following command:

```bash
cargo build --release
```

This will compile the code into an optimized binary that is ready for production use.

### 3. Run the exporter

Once the build is successful, you can run the exporter by executing:

```bash
cargo run
```

By default, the exporter will start an HTTP server on http://127.0.0.1:9000/metrics. If you'd like to change the port or other configuration settings, you can modify the code or use environment variables.

## Usage

The exporter exposes network metrics in a format that Prometheus can scrape. To enable Prometheus to collect the metrics from the exporter, you need to add it to your Prometheus configuration.

### 1. Configure Prometheus

In your prometheus.yml file, add a new scrape job like this:

```yaml
scrape_configs:
  - job_name: 'network-metrics'
    static_configs:
      - targets: ['localhost:9000']
```

### 2. Access the Metrics

Once the exporter is running, you can access the exposed metrics by navigating to:

```
http://127.0.0.1:9000/metrics
```

Prometheus will scrape this endpoint at the configured interval to collect the network statistics.

## Metrics Exposed

The exporter exposes several network-related metrics. These metrics represent data collected for each network interface available on the system. Some example metrics are:

- `network_received_bytes{interface="eth0"}` – Total bytes received on the eth0 interface.
- `network_transmitted_bytes{interface="eth0"}` – Total bytes transmitted on the eth0 interface.
- `network_received_packets{interface="eth0"}` – Total packets received on the eth0 interface.
- `network_transmitted_packets{interface="eth0"}` – Total packets transmitted on the eth0 interface.
- `network_received_errors{interface="eth0"}` – Total errors during received packets on eth0.
- `network_transmitted_errors{interface="eth0"}` – Total errors during transmitted packets on eth0.

Each metric includes the network interface as a label, allowing you to track statistics per interface.

## Building and Running in Production

For production deployment, use the following steps:

1. Build a release version of the binary:

```bash
cargo build --release
```

2. Run the release binary:

Navigate to the target/release directory, and run the compiled binary:

```bash
./prometheus-network-metrics-exporter
```

This will start the exporter on the default port 9000. If you'd like to change the port, update the environment variables or modify the configuration.

## Dependencies

This project uses the following dependencies:

- **sysinfo**: For gathering system and network statistics. More information can be found at: [sysinfo crate](https://crates.io/crates/sysinfo).
- **prometheus**: For exposing metrics in a format that Prometheus can scrape. Learn more about this crate at: [prometheus crate](https://crates.io/crates/prometheus).
- **warp**: A lightweight and efficient web framework for serving HTTP requests. You can read more at: [warp crate](https://crates.io/crates/warp).

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

This project leverages several great open-source libraries:

- **sysinfo**: A library that provides system information such as CPU usage, memory usage, and network statistics.
- **Prometheus**: An open-source system monitoring and alerting toolkit.
