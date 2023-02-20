#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
extern crate crossbeam;
extern crate crossbeam_channel;

use anyhow::{bail, Result};
use crossbeam_channel::bounded;
use embedded_hal_0_2::PwmPin;
use embedded_svc::{
    mqtt::client::{Connection, Event, MessageImpl, QoS},
    utils::mqtt::client::ConnState,
    wifi::*,
};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::{
    adc::{self, config::Config, AdcDriver, Atten11dB, *},
    gpio::{
        self, AnyInputPin, AnyOutputPin, Input, InputMode, InputPin, Output, OutputPin, PinDriver,
        *,
    },
    ledc::*,
    peripheral,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use serde::Serialize;
use statig::{prelude::*, InitializedStatemachine};
use std::{
    env,
    sync::{mpsc::*, Arc},
    thread,
    time::Duration,
};

use esp_idf_svc::{eventloop::*, mqtt::client, netif::*, wifi::*};

mod led_fsm;
mod tasks;

const SSID: &str = env!("WIFI_SSID");
const PASS: &str = env!("WIFI_PASS");
const MQTT_URL: &str = env!("MQTT_URL");

#[derive(Serialize, Debug)]
struct MqttData {
    distance: u16,
    temperature: f32,
    tds: f32,
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Config GPIO for input and output
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();
    // TODO: how to make the `btn` pin pull-up or pull-down.
    // LED controller config
    let config = TimerConfig::new().frequency(25.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config).unwrap());
    let channel0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio7,
    )
    .unwrap();

    let max_duty = channel0.get_max_duty();

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let (tx1, rx1) = bounded(1);
    let (tx2, rx2) = (tx1.clone(), rx1.clone());

    let a1_ch4 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    // start wifi
    let sysloop = EspSystemEventLoop::take().unwrap();
    let _wifi = wifi(peripherals.modem, sysloop).unwrap();
    //let mut client = get_client(MQTT_URL).unwrap();

    // initialize the tasks
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(tasks::BLINKY_STACK_SIZE)
        .spawn(move || tasks::blinky_fsm_thread(led_fsm, rx1))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(tasks::BUTTON_STACK_SIZE)
        .spawn(move || tasks::button_thread(btn, tx1))
        .unwrap();

    let _adc_thread = std::thread::Builder::new()
        .stack_size(tasks::ADC_STACK_SIZE)
        .spawn(move || tasks::adc_thread(adc1, a1_ch4, max_duty, channel0, tx2))
        .unwrap();
}

#[cfg(not(feature = "qemu"))]
fn wifi(
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
    use std::net::Ipv4Addr;

    let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);

    println!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        password: PASS.into(),
        channel,
        ..Default::default()
    }))?;

    wifi.start()?;

    println!("Starting wifi...");

    if !WifiWait::new(&sysloop)?
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        bail!("Wifi did not start");
    }

    println!("Connecting wifi...");

    wifi.connect()?;

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        bail!("Wifi did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    println!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
