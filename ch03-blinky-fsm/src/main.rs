use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::{gpio, prelude::*};
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio8.into_output().unwrap();
    let state_machine = Blinky { led }.state_machine().init();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(state_machine))
        .unwrap();
}

fn blinky_fsm_thread(mut fsm: InitializedStatemachine<Blinky>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&Event::TimerElapsed);
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&Event::TimerElapsed);
    }
}

// #[derive(Debug, Default)]
pub struct Blinky {
    led: gpio::Gpio8<gpio::Output>,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
}

#[state_machine(
    initial = "State::led_on()",
    state(derive(Debug)),
    on_transition = "Self::on_transition"
)]
impl Blinky {
    #[action]
    fn enter_on(&mut self) {
        self.led.set_high().unwrap();
    }

    #[state(entry_action = "enter_on")]
    fn led_on(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_off()),
            _ => Super,
        }
    }

    #[action]
    fn enter_off(&mut self) {
        self.led.set_low().unwrap();
    }

    #[state(entry_action = "enter_off")]
    fn led_off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super,
        }
    }
}

impl Blinky {
    // The `on_transition` callback that will be called after every transition.
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }
}
