use std::process::{Child, Command};

use network_manager::Device;

use errors::*;
use config::Config;

pub fn start_dnsmasq(config: &Config, device: &Device) -> Result<Child> {
    let args = [
        // the following addresses were taken from
        // https://github.com/tretos53/Captive-Portal/blob/master/dnsmasq.conf
        // they are required because otherwise the WiFi portal will not
        // be shown automatically when connecting to the WiFi on Android devices
        "--address=/connectivitycheck.gstatic.com/216.58.206.131",
        "--address=/www.gstatic.com/216.58.206.99",
        "--address=/clients3.google.com/216.58.204.46",
        "--address=/www.msftconnecttest.com/13.107.4.52",
        &format!("--address=/#/{}", config.gateway),
        &format!("--dhcp-range={}", config.dhcp_range),
        &format!("--dhcp-option=option:router,{}", config.gateway),
        &format!("--interface={}", device.interface()),
        "--keep-in-foreground",
        "--bind-interfaces",
        "--except-interface=lo",
        "--conf-file",
        "--no-hosts",
    ];

    Command::new("dnsmasq")
        .args(&args)
        .spawn()
        .chain_err(|| ErrorKind::Dnsmasq)
}
