#![allow(unused_imports, dead_code)]
use anyhow::bail;
use anyhow::Result;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sntp;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

const WIFI_SSID: &str = "TheHolyGrail";
const WIFI_PASS: &str = "DevelopersDevelopers";

pub fn test_wifi() -> Result<String> {
    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_look_stack = Arc::new(EspSysLoopStack::new()?);
    let nvs = Arc::new(EspDefaultNvs::new()?);

    let mut wifi = EspWifi::new(netif_stack, sys_look_stack, nvs)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.into(),
        password: WIFI_PASS.into(),
        ..Default::default()
    }))?;

    wifi.wait_status_with_timeout(Duration::from_secs(30), |s| !s.is_transitional())
        .map_err(|e| anyhow::anyhow!("Wait timeout: {:?}", e))?;

    let status = wifi.get_status();

    println!("Status: {:?}", status);

    if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
        client_settings,
    ))) = status.0
    {
        Ok(format!("{:?}", client_settings.ip))
    } else {
        Err(anyhow::anyhow!("Failed to connect in time."))
    }
}
