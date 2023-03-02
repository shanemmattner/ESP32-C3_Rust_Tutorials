use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    uart,
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

static ASCII_DEL_CODE: u8 = 8;
static ASCII_CR_CODE: u8 = 13;
static MAX_UART_BUFFER: usize = 100;

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

pub fn uart_thread(uart: uart::UartDriver) {
    let mut uart_buf: Vec<u8> = Vec::new();

    loop {
        let mut buf: [u8; 100] = [0; 100];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(x) => {
                if x > 0 {
                    // Logic for deleting a character in the uart buffer vector
                    if buf[0] == ASCII_DEL_CODE {
                        match uart_buf.pop() {
                            Some(_) => {}
                            None => println!("Error: No characters to pop"),
                        }
                    } else {
                        uart_buf.push(buf[0]);
                    }
                }
            }
            Err(_) => {}
        }

        if uart_buf.len() > 0 {
            if uart_buf[uart_buf.len() - 1] == ASCII_CR_CODE {
                println!("{:?}", uart_buf);
                match uart_write(&uart, &uart_buf) {
                    CliError::Success => uart_buf.clear(),
                    CliError::Fail => {}
                }
            }
        }

        if uart_buf.len() > MAX_UART_BUFFER {
            uart_buf.clear();
        }

        FreeRtos::delay_ms(50);
    }
}

fn uart_write(uart: &uart::UartDriver, msg: &Vec<u8>) -> CliError {
    match uart.write(&msg) {
        Ok(_) => CliError::Success,
        Err(_) => CliError::Fail,
    }
}
