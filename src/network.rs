use sysinfo::{System, NetworkData};

pub fn update_network_metrics() {
    let mut sys = System::new_all();

    // Refresh system data to update network information
    sys.refresh_all();

    // Access the network information correctly
    let networks = sys.networks(); // Should be available in the updated API

    // Iterate over the network interfaces
    for (interface_name, network_data) in networks {
        println!(
            "Interface: {}, Received: {} bytes, Transmitted: {} bytes",
            interface_name,
            network_data.received(),
            network_data.transmitted()
        );
    }
}
