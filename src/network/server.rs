use super::client;
use crate::bloop::{Request, Response};
use get_if_addrs::{get_if_addrs, Interface};
use libmdns::Responder;
use log::{debug, info};
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
    let hostname = hostname::get()
        .expect("Failed to get hostname")
        .into_string()
        .expect("Failed to convert hostname to String")
        .replace(".local", "")
        .replace(".lan", "");

    let ips = get_ips_for_responder();

    for ip in ips.iter() {
        info!("Responding on IP: {ip}");
    }

    let responder = Responder::new_with_ip_list(ips).expect("Couldn't create an mDNS responder");
    let _service = responder.register("_bloop._tcp".into(), hostname, local_port, &[]);

    let local_ip = local_address.ip().to_string();
    info!("Server listening on {local_ip}:{local_port}");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = request_tx.clone();
        let rx = response_tx.subscribe();
        tokio::spawn(async move {
            client::run(stream, tx, rx).await;
        });
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
