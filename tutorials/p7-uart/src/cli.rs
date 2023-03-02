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
