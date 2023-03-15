use esp_idf_hal::{
    delay::{Ets, FreeRtos, NON_BLOCK},
    uart,
};
use lazy_static::lazy_static; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use shell_words::*;
use std::{collections::HashMap, str};
use crossbeam_utils::atomic::AtomicCell;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

pub struct SShellCommand<'a> {
    handler: fn(Vec<String>) -> &'static str,
    help: &'a str,
}

// Define the shell commands with a hashmap
lazy_static! {
    pub static ref HASHES: HashMap<&'static str, SShellCommand<'static>> = {
        let mut m = HashMap::new();
        // Add each command handler and help message
        m.insert(
            "hello",
            SShellCommand {
                handler: cli_hello,
                help: "Prints hello world",
            },
        );
        m.insert("cmd2",
                 SShellCommand{
                     handler: cli_cmd2,
                     help:"Subcommand logic example"
                 },
                 );
        m
    };
}

const ASCII_DEL_CODE: u8 = 8;
const ASCII_CR_CODE: u8 = 13;
const MAX_UART_BUFFER: usize = 100;
const HELP_KEYWORD: &str = "help";
const MENU_KEYWORD: &str = "menu";

pub fn uart_thread(uart: uart::UartDriver, 
                    adc_atomic: Arc<AtomicCell<[u16;4]>>, 
                    cli_msgs: Arc<ArrayQueue::<String>>
) {
    let mut uart_buf: Vec<u8> = Vec::new();
    let mut adc_stream_buffer:u32 = 0;

    loop {
        let mut buf: [u8; 10] = [0; 10];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(x) => {
                if x > 0 {
                    // We only read 1 byte at a time since we poll for new messages every 50ms
                    // Therefore we only buffer the 0th element
                    buffer_uart_byte(buf[0], &mut uart_buf);
                }
            }
            Err(_) => {}
        }

        // check for a valid command and process accordingly
        match process_uart_buffer(&mut uart_buf) {
            Some(ret) => {
                if ret.len() > 0 {
                    match uart.write(&ret.as_bytes().to_vec()) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
            }
            None => {}
        }

        // adc_stream_buffer += 1;
        // if adc_stream_buffer == 100{
        //     adc_stream_buffer = 0;
        //     let adc_values = adc_atomic.load();
        //     let mut to_send = adc_values[0].to_string();
        //     to_send.push('\n');
        //     match uart.write(to_send.as_bytes()){
        //         Ok(_) => {},
        //         Err(_) => {}
        //     }
        // }

        while !cli_msgs.is_empty(){
            match cli_msgs.pop(){
                Some(x) => {
                    match uart.write(x.as_bytes()){
                        Ok(_) => {},
                        Err(_) => {}
                    }   
                },
                None => {},
            }
        }



        Ets::delay_us(1000);
    }
}

fn process_uart_buffer(uart_buf: &mut Vec<u8>) -> Option<String> {
    let mut ret: Option<String> = None;

    match uart_buf.last() {
        Some(c) => {
            // If the last character was a carriage return then process command
            if *c == ASCII_CR_CODE {
                // Pop off the carriage return
                uart_buf.pop();
                // If user just pressed return then don't process command
                if uart_buf.len() > 0 {
                    // ensure input is utf-8 format
                    match str::from_utf8(&uart_buf) {
                        Ok(s) => {
                            // split uart buffer to get command and arguments
                            let buf_split = split(s).unwrap();
                            let cmd: &str = &buf_split.get(0).unwrap_or(&"".to_owned()).to_owned();
                            let sub_cmds: Vec<String> =
                                buf_split.get(1..).unwrap_or(&["".to_owned()]).to_vec();

                            if cmd == MENU_KEYWORD {
                                let mut buf: String = "Help Menu\n------------\n".to_owned();
                                for (key, value) in &*HASHES {
                                    buf.push_str(key);
                                    buf.push_str(" : ");
                                    buf.push_str(value.help);
                                    buf.push('\n');
                                }
                                ret = Some(buf);
                            } else {
                                // Try to find the key in the hashmap, if found process command
                                match HASHES.get(cmd) {
                                    Some(cmd) => {
                                        if sub_cmds.len() > 0 && sub_cmds[0] == HELP_KEYWORD {
                                            ret = Some(cmd.help.to_owned());
                                        } else {
                                            ret = Some((cmd.handler)(sub_cmds).to_owned());
                                        }
                                    }
                                    None => ret = Some("Key not found\n".to_owned()),
                                }
                            }
                        }
                        Err(_) => ret = Some("Not UTF-8 formatted\n".to_owned()),
                    }
                }
                uart_buf.clear();
            }
        }
        None => {}
    }
    ret
}

fn buffer_uart_byte(new_byte: u8, uart_buf: &mut Vec<u8>) {
    // Logic for deleting a character in the uart buffer vector
    if new_byte == ASCII_DEL_CODE {
        match uart_buf.pop() {
            Some(_) => {}
            None => {}
        }
    } else {
        uart_buf.push(new_byte);
        // Prevent buffer from growing too large, no command should be more than 100 characters
        if uart_buf.len() > MAX_UART_BUFFER {
            uart_buf.clear();
        }
    }
}

fn cli_hello(subcommand: Vec<String>) -> &'static str {
    println!("Subcommands: {:?}", subcommand);
    "Hello World\n"
}

fn cli_cmd2(subcommand: Vec<String>) -> &'static str {
    let mut ret: &str = "Incorrect number of arguments\n";

    if subcommand.len() == 2 {
        ret = "Correct number of arguments\n";
    }
    ret
}
