#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio;
use statig::prelude::*;
use std::sync::mpsc;
use std::{thread, time::Duration};

// #[derive(Debug, Default)]
pub struct Button {
    pub btn: gpio::Gpio6<gpio::Input>,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    Tick,
}

#[state_machine(
    initial = "State::button_off()",
    state(derive(Debug)),
    on_transition = "Self::on_transition"
)]
impl Button {
    #[action]
    fn enter_on(&mut self) {}

    #[state(entry_action = "enter_on")]
    fn button_on(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Tick => Transition(State::button_off()),
        }
    }

    #[action]
    fn enter_off(&mut self) {}

    #[state(entry_action = "enter_off")]
    fn button_off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Tick => Transition(State::button_on()),
        }
    }
}

impl Button {
    // The `on_transition` callback that will be called after every transition.
    fn on_transition(&mut self, source: &State, target: &State) {
        // println!("transitioned from `{:?}` to `{:?}`", source, target);
    }
}
