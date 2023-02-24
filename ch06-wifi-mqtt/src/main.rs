mod tasks;
mod wifi;

use anyhow::{bail, Result};
use crossbeam_channel::bounded;
use crossbeam_utils::atomic::AtomicCell;
use embedded_svc::mqtt::client::{Connection, Event, MessageImpl, QoS};
use embedded_svc::utils::mqtt::client::ConnState;
use esp_idf_hal::{
    adc::{self, *},
    gpio::{IOPin, PinDriver, Pull},
    ledc::{config::TimerConfig, *},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_svc::{
    mqtt::client::{EspMqttClient, LwtConfiguration, MqttClientConfiguration},
    wifi::EspWifi,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::esp_efuse_mac_get_default;
use esp_idf_sys::EspError;
use esp_println::println;
use mqtt::control::ConnectReturnCode;
use mqtt::packet::{ConnackPacket, ConnectPacket, PublishPacketRef, QoSWithPacketIdentifier};
use mqtt::{Decodable, Encodable, TopicName};
use serde::Serialize;
use std::{env, io::Write, net::TcpStream, sync::Arc, thread};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;
static ADC_STACK_SIZE: usize = 2000;

const SSID: &str = env!("WIFI_SSID");
const PASS: &str = env!("WIFI_PASS");
//const _MQTT_URL: &str = env!("MQTT_URL");

const MQTT_ADDR: &str = "mqtt://192.168.68.115:1883"; // host:port
const MQTT_CLIENT_ID: &str = "clientId-SvLEFMBFY3";
const MQTT_TOPIC_NAME: &str = "test_topic";

#[derive(Serialize, Debug)]
struct MqttData {
    distance: u16,
    temperature: f32,
    tds: f32,
}

pub struct Config {
    mqtt_host: &'static str,
    mqtt_user: &'static str,
    mqtt_pass: &'static str,
    wifi_ssid: &'static str,
    wifi_psk: &'static str,
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // MQTT Client configuration:
    let app_config = Config {
        mqtt_host: "clientId-sycsXsDMCw",
        mqtt_user: "shane123",
        mqtt_pass: "pass",
        wifi_ssid: SSID,
        wifi_psk: PASS,
    };

    match wifi::connect(app_config.wifi_ssid, app_config.wifi_psk) {
        Ok(_) => {
            println!("Connected to WiFi succesfully")
        }
        Err(e) => println!("Error connecting to wifi: {e}"),
    };

    //let broker_url = if app_config.mqtt_user != "" {
    //    format!(
    //        "mqtt://{}:{}@{}",
    //        app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
    //    )
    //} else {
    //    format!("mqtt://{}", app_config.mqtt_host)
    //};

    //let _client = get_client(&MQTT_URL);
    //match mqtt_connect(MQTT_ADDR, MQTT_CLIENT_ID) {
    //    Ok(_) => println!("MQTT connect"),
    //    Err(_) => println!("MQTT ERROR"),
    //}

    match test_mqtt_client() {
        Ok(_) => println!("MQTT connect"),
        Err(_) => println!("MQTT ERROR"),
    }

    //    let mqtt_config = MqttClientConfiguration::default();
    //    // 1. Create a client with default configuration and empty handler
    //let mut client = EspMqttClient::new(broker_url, &mqtt_config, move |message_event| {
    //    // ... your handler code here - leave this empty for now
    //    // we'll add functionality later in this chapter
    //})
    //.unwrap();
    //
    //    // 2. publish an empty hello message
    //    let payload: &[u8] = &[];
    //    client
    //        .publish(&hello_topic(UUID), QoS::AtLeastOnce, true, payload)
    //        .unwrap();
    //
    // Initialize the button pin
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    // Crossbeam channel
    let (tx, rx) = bounded(1);

    // ADC init
    // Create atomic to store adc readings
    let atomic_value: AtomicCell<u16> = AtomicCell::new(0);
    let arc_data = Arc::new(atomic_value);
    // Create ADC channel driver
    let a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    let timer_config = TimerConfig::new().frequency(25.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config).unwrap());
    let channel_0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio8,
    )
    .unwrap();
    let a1 = arc_data.clone();
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || tasks::blinky_thread(rx, a1, channel_0))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || tasks::button_thread(btn_pin, tx))
        .unwrap();

    let a2 = arc_data.clone();
    let _adc_thread = std::thread::Builder::new()
        .stack_size(ADC_STACK_SIZE)
        .spawn(move || tasks::adc_thread(a2, adc1, a1_ch0))
        .unwrap();
}

pub fn get_unique_id() -> String {
    let mut mac: [u8; 6] = [0; 6];
    unsafe {
        let ptr = &mut mac as *mut u8;
        esp_efuse_mac_get_default(ptr);
    }
    hex::encode(mac)
}

fn test_mqtt_client() -> Result<EspMqttClient<ConnState<MessageImpl, EspError>>> {
    println!("About to start MQTT client");

    let conf = MqttClientConfiguration {
        client_id: Some("rust-esp32-std-demo"),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    };

    let (mut client, mut connection) =
        EspMqttClient::new_with_conn("mqtts://broker.emqx.io:8083", &conf)?;

    println!("MQTT client started");

    // Need to immediately start pumping the connection for messages, or else subscribe() and publish() below will not work
    // Note that when using the alternative constructor - `EspMqttClient::new` - you don't need to
    // spawn a new thread, as the messages will be pumped with a backpressure into the callback you provide.
    // Yet, you still need to efficiently process each message in the callback without blocking for too long.
    //
    // Note also that if you go to http://tools.emqx.io/ and then connect and send a message to topic
    // "rust-esp32-std-demo", the client configured here should receive it.
    thread::spawn(move || {
        println!("MQTT Listening for messages");

        while let Some(msg) = connection.next() {
            match msg {
                Err(e) => println!("MQTT Message ERROR: {}", e),
                Ok(msg) => println!("MQTT Message: {:?}", msg),
            }
        }

        println!("MQTT connection loop exit");
    });

    client.subscribe("rust-esp32-std-demo", QoS::AtMostOnce)?;

    println!("Subscribed to all topics (rust-esp32-std-demo)");

    client.publish(
        "rust-esp32-std-demo",
        QoS::AtMostOnce,
        false,
        "Hello from Shane's rust-esp32-std-demo! #1".as_bytes(),
    )?;

    println!("Published a hello message to topic \"rust-esp32-std-demo\"");

    Ok(client)
}
