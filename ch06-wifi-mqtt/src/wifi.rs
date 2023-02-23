use anyhow::{bail, Context};
use crossbeam_channel::bounded;
use crossbeam_utils::atomic::AtomicCell;
use embedded_svc::wifi::{
    AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration,
};
use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    gpio::{ADCPin, AnyIOPin, IOPin, Input, PinDriver, Pull},
    ledc::{config::TimerConfig, *},
    modem::{Modem, WifiModem},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_svc::{
    eventloop::{EspEventLoop, EspSystemEventLoop, System},
    mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration},
    netif::{EspNetif, EspNetifWait},
    nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault},
    timer::EspTaskTimerService,
    wifi::{EspWifi, WifiWait},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use serde::Serialize;
use std::{env, net::Ipv4Addr, sync::atomic::*, sync::Arc, thread, time::Duration};

pub fn connect(wifi_ssid: &str, wifi_pass: &str) -> anyhow::Result<EspWifi<'static>> {
    let sys_loop = EspEventLoop::take().unwrap();
    let modem = unsafe { WifiModem::new() };
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = EspWifi::new(modem, sys_loop.clone(), Some(nvs))?;

    println!("Wifi created, scanning available networks...");

    let available_networks = wifi.scan()?;
    let target_network = available_networks
        .iter()
        .find(|network| network.ssid == wifi_ssid)
        .with_context(|| format!("Failed to detect the target network ({wifi_ssid})"))?;

    println!("Scan successfull, found '{wifi_ssid}', with config: {target_network:#?}");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_ssid.into(),
        password: wifi_pass.into(),
        auth_method: target_network.auth_method,
        bssid: Some(target_network.bssid),
        channel: Some(target_network.channel),
    }))?;

    wifi.start()?;
    if !WifiWait::new(&sys_loop)?
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        bail!("Wifi did not start");
    }

    wifi.connect()?;

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sys_loop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.driver().is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        bail!("Wifi did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    println!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
