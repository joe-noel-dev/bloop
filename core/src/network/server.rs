use super::client;
use crate::bloop::{Request, Response};
use get_if_addrs::{get_if_addrs, Interface};
use log::{debug, info, warn};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::net::IpAddr;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

const PORT: u16 = 14072;

pub async fn run(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    let ips = get_ips_for_responder();

    for ip in ips.iter() {
        tokio::spawn(listen_on_ip(*ip, request_tx.clone(), response_tx.clone()));
    }
}

pub async fn listen_on_ip(ip: IpAddr, request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    let address = format!("{ip}:{PORT}");
    info!("Binding to: {address}");
    let listener = match TcpListener::bind(address.clone()).await {
        Ok(listener) => listener,
        Err(error) => {
            warn!("Failed to bind to {address}: {error}");
            warn!("Trying port 0 instead");
            TcpListener::bind(format!("{ip}:0"))
                .await
                .expect("Failed to bind to any port")
        }
    };

    let local_address = listener.local_addr().expect("Unable to get address from port");

    let local_port = local_address.port();
    let instance_name = service_instance_name(ip);

    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_bloop._tcp.local.";
    let host_name = format!("{ip}.local.");
    let version = env!("CARGO_PKG_VERSION");
    let properties = [("version", version)];

    let service_info = ServiceInfo::new(
        service_type,
        instance_name.as_str(),
        &host_name,
        ip,
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

fn service_instance_name(ip: IpAddr) -> String {
<<<<<<< HEAD
<<<<<<< HEAD
    let fallback = safe_fallback_name(ip);
=======
    let fallback = android_safe_fallback_name(ip);
>>>>>>> d395b60 (Require BLOOP_HOME from app)
=======
    let fallback = safe_fallback_name(ip);
>>>>>>> 99faabd (Don't need a separate fallback for Android)

    match hostname::get() {
        Ok(hostname) => match hostname.into_string() {
            Ok(raw_hostname) => {
                let clean_hostname = raw_hostname.replace(".local", "").replace(".lan", "");
                if clean_hostname.trim().is_empty() {
                    fallback
                } else {
                    clean_hostname
                }
            }
            Err(_) => {
                warn!("Hostname is not valid UTF-8; using fallback mDNS instance name");
                fallback
            }
        },
        Err(error) => {
            warn!("Failed to get hostname ({error}); using fallback mDNS instance name");
            fallback
        }
    }
}

<<<<<<< HEAD
<<<<<<< HEAD
fn safe_fallback_name(ip: IpAddr) -> String {
    format!("bloop-{}", ip.to_string().replace('.', "-"))
=======
fn android_safe_fallback_name(ip: IpAddr) -> String {
    if cfg!(target_os = "android") {
        format!("bloop-android-{}", ip.to_string().replace('.', "-"))
    } else {
        format!("bloop-{}", ip.to_string().replace('.', "-"))
    }
>>>>>>> d395b60 (Require BLOOP_HOME from app)
=======
fn safe_fallback_name(ip: IpAddr) -> String {
    format!("bloop-{}", ip.to_string().replace('.', "-"))
>>>>>>> 99faabd (Don't need a separate fallback for Android)
}

fn should_respond_on_interface(iface: &Interface) -> bool {
    let ignored_interfaces = [
        "utun", "tun", "tap", "wg", "docker", "veth", "br-", "bridge", "virbr", "pdp_ip", "ipsec",
    ];

    iface.addr.ip().is_ipv4() && !ignored_interfaces.iter().any(|ignored| iface.name.contains(ignored))
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
