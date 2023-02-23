use anyhow::{bail, Result};
use embedded_svc::mqtt::client::{Connection, Event, MessageImpl, QoS};
use embedded_svc::{utils::mqtt::client::ConnState, wifi::*};
use esp_idf_hal::peripheral;
use esp_idf_svc::mqtt::client::{EspMqttClient, LwtConfiguration, MqttClientConfiguration};
use esp_idf_svc::{eventloop::*, netif::*, wifi::*};
use esp_idf_sys::esp_efuse_mac_get_default;
use esp_idf_sys::EspError;
use esp_println::println;
use std::{env, thread, time::Duration};

const SSID: &str = env!("WIFI_SSID");
const PASS: &str = env!("WIFI_PASS");
const _MQTT_URL: &str = env!("MQTT_URL");

#[cfg(not(feature = "qemu"))]
pub fn wifi(
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

pub fn get_client(url: &str) -> Result<EspMqttClient<ConnState<MessageImpl, EspError>>, EspError> {
    let client_id = format!("fishtank-rust_{}", get_unique_id());
    let conf = MqttClientConfiguration {
        client_id: Some(&client_id),
        keep_alive_interval: Some(std::time::Duration::new(60, 0)),
        lwt: Some(LwtConfiguration {
            topic: "fishtank/status",
            payload: b"offline",
            qos: QoS::AtLeastOnce,
            retain: true,
        }),
        ..Default::default()
    };

    let (mut client, mut connection) = EspMqttClient::new_with_conn(url, &conf).unwrap();

    thread::spawn(move || {
        while let Some(msg) = connection.next() {
            let event = msg.as_ref().unwrap();
            match event {
                Event::Received(_msg) => {}
                Event::Connected(_) => {}
                Event::Disconnected => {}
                Event::Subscribed(_x) => {
                    // Do nothing
                }
                _event => println!("Got unknown MQTT event"),
            }
        }
    });
    client
        .publish("fishtank/status", QoS::AtLeastOnce, true, b"online")
        .unwrap();
    Ok(client)
}

pub fn get_unique_id() -> String {
    let mut mac: [u8; 6] = [0; 6];
    unsafe {
        let ptr = &mut mac as *mut u8;
        esp_efuse_mac_get_default(ptr);
    }
    hex::encode(mac)
}

fn publish_data(data: MqttData, client: &mut EspMqttClient<ConnState<MessageImpl, EspError>>) {
    let data = serde_json::to_string(&data).unwrap();
    client
        .publish("fishtank/sensors", QoS::AtLeastOnce, false, data.as_bytes())
        .unwrap();
}
