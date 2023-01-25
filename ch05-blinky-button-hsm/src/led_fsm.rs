#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio;
use statig::prelude::*;

// #[derive(Debug, Default)]
pub struct Blinky {
    pub led: gpio::Gpio8<gpio::Output>,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
    ButtonPressed,
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

    #[state(entry_action = "enter_on", superstate = "blinking")]
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

    #[state(entry_action = "enter_off", superstate = "blinking")]
    fn led_off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super,
        }
    }

    #[superstate]
    fn blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::not_blinking()),
            _ => Super,
        }
    }

    #[state]
    fn not_blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::led_on()),
            // Altough this state has no superstate, we can still defer the event which
            // will cause the event to be handled by an implicit `top` superstate.
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
