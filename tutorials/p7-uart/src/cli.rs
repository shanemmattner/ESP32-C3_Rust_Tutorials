use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    gpio,
    peripherals::Peripherals,
    prelude::*,
    uart::{self, *},
};
use lazy_static::lazy_static; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::collections::HashMap;

pub enum CliError {
    Success,
    Fail,
}

pub struct SShellCommand<'a> {
    handler: fn(&str) -> CliError,
    help: &'a str,
}

lazy_static! {
    pub static ref HASHES: HashMap<&'static str, SShellCommand<'static>> = {
        let mut m = HashMap::new();
        // Add each command handler and help message
        m.insert(
            "hello",
            SShellCommand {
                handler: cli_hello,
                help: "Say hello",
            },
        );
        m
    };
}

pub fn cli_hello(subcommand: &str) -> CliError {
    let err = CliError::Fail;

    let msg = "Hello World";
    println!("{msg}");
    //    match uart.write(&msg) {
    //        Ok(_) => println!("msg sent successful"),
    //        Err(_) => {}
    //    }
    err
}

pub fn uart_thread(uart: uart::UartDriver) -> CliError {
    let err = CliError::Fail;
    let mut uart_buf: Vec<u8> = Vec::new();

    loop {
        let mut buf: [u8; 100] = [0; 100];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(x) => {
                if x > 0 {
                    uart_buf.push(buf[0]);
                }
            }
            Err(_) => {}
        }

        if uart_buf.len() > 0 {
            if uart_buf[uart_buf.len() - 1] == 13 {
                println!("{:?}", uart_buf);
                match uart.write(&uart_buf) {
                    Ok(_) => uart_buf.clear(),
                    Err(_) => {}
                }
            }
        }
        FreeRtos::delay_ms(100);
    }
    err
}
