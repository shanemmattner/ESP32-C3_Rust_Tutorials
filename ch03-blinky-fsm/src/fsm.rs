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
