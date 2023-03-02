use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    uart,
};
use lazy_static::lazy_static; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use shell_words::*;
use std::{collections::HashMap, str};

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

fn uart_write(uart: &uart::UartDriver, msg: &Vec<u8>) -> CliError {
    match uart.write(&msg) {
        Ok(_) => CliError::Success,
        Err(_) => CliError::Fail,
    }
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
                        // Prevent buffer from growing too large, no command should be more than 100 characters
                        if uart_buf.len() > MAX_UART_BUFFER {
                            uart_buf.clear();
                        }
                    }
                }
            }
            Err(_) => {}
        }

        if uart_buf.len() > 0 {
            // If the last character was a carriage return then process command
            if uart_buf[uart_buf.len() - 1] == ASCII_CR_CODE {
                // remove the carriage return character from the end of the line
                uart_buf.pop();
                // Make sure there wasn't just a carriage return
                if uart_buf.len() > 0 {
                    // convert vec<u8> to utf-8 &str
                    match str::from_utf8(&uart_buf) {
                        Ok(s) => {
                            // split str slice to get command and arguments
                            let buf_split = split(s).unwrap();
                            // Make sure there's more than just the command present
                            if buf_split.len() > 1 {}
                            let cmd: &str = &buf_split[0][..];

                            // find the appropriate command
                            match HASHES.get(cmd) {
                                Some(x) => {
                                    x.handler();
                                    println!("key found");
                                }
                                None => println!("key not found"),
                            }

                            println!("{:?}", buf_split);
                            match uart_write(&uart, &uart_buf) {
                                CliError::Success => uart_buf.clear(), // clear buffer on write success
                                CliError::Fail => {}
                            }
                        }
                        Err(_) => uart_buf.clear(),
                    }
                }
            }
        }

        FreeRtos::delay_ms(50);
    }
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
