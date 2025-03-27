use super::client;
use crate::bloop::{Request, Response};
use get_if_addrs::{get_if_addrs, Interface};
use log::{debug, info, warn};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::net::IpAddr;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

const PORT: u16 = 0;
const BIND_ADDRESS: &str = "0.0.0.0";

pub async fn run(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    let address = format!("{BIND_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(address).await.expect("Failed to bind");

    let local_address = listener.local_addr().expect("Unable to get address from port");

    let local_port = local_address.port();
    let raw_hostname = hostname::get()
        .expect("Failed to get hostname")
        .into_string()
        .expect("Failed to convert hostname to String");
    let clean_hostname = raw_hostname.replace(".local", "").replace(".lan", "");

    let ips = get_ips_for_responder();

    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    for ip in ips.iter() {
        info!("Responding on IP: {ip}");
    }

    let service_type = "_bloop._tcp.local.";
    let instance_name = clean_hostname.as_str();
    let host_name = format!("{}.local.", ips.first().expect("No valid IPs found"));
    let version = env!("CARGO_PKG_VERSION");
    let properties = [("version", version)];

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        &host_name,
        &ips[..],
        local_port,
        &properties[..],
    )
    .expect("Unable to create service info");

    mdns.register(service_info).expect("Failed to register service");

    let local_ip = local_address.ip().to_string();
    info!("Server listening on {local_ip}:{local_port}");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = request_tx.clone();
        let rx = response_tx.subscribe();
        tokio::spawn(async move {
            client::run(stream, tx, rx).await;
        });
    }

    if let Err(error) = mdns.shutdown() {
        warn!("Failed to shutdown mDNS daemon: {error}");
    }
}

fn should_respond_on_interface(iface: &Interface) -> bool {
    let ignored_interfaces = [
        "utun", "tun", "lo", "tap", "wg", "docker", "veth", "br-", "bridge", "virbr", "pdp_ip", "ipsec",
    ];

    !iface.is_loopback()
        && iface.addr.ip().is_ipv4()
        && !ignored_interfaces.iter().any(|ignored| iface.name.contains(ignored))
}

fn get_ips_for_responder() -> Vec<IpAddr> {
    let ifaces = get_if_addrs().expect("Unable to get network interfaces");

    ifaces.iter().for_each(|iface| {
        let name = &iface.name;
        let ip = iface.addr.ip();
        debug!("Available interface: {name} ({ip})");
    });

    let interfaces = ifaces
        .iter()
        .filter(|iface| should_respond_on_interface(iface))
        .map(|iface| iface.addr.ip())
        .collect();

    interfaces
}
