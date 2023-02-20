use esp_idf_hal::gpio::{AnyOutputPin, Output, PinDriver};
use statig::prelude::*;

// #[derive(Debug, Default)]
pub struct Blinky<'a> {
    pub led: PinDriver<'a, AnyOutputPin, Output>,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
}

#[derive(Debug)]
pub enum State {
    LedOn,
    LedOff,
}

impl StateMachine for Blinky<'_> {
    type State = State;
    type Superstate<'a> = ();
    type Event<'a> = Event;
    const INITIAL: State = State::LedOff;
    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |_, source, target| {
        println!("Transitioned from {source:?} to {target:?}");
    };
}

impl statig::State<Blinky<'_>> for State {
    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event) -> Response<Self> {
        match self {
            State::LedOn => blinky.led_on(event),
            State::LedOff => blinky.led_off(event),
        }
    }

    fn call_entry_action(&mut self, blinky: &mut Blinky) {
        match self {
            State::LedOn => blinky.enter_led_on(),
            State::LedOff => blinky.enter_led_off(),
        }
    }
}

impl Blinky<'_> {
    fn enter_led_on(&mut self) {
        // Setting the pin high turns the LED off on my dev board
        self.led.set_low().unwrap();
    }
    fn enter_led_off(&mut self) {
        // Setting the pin low turns the LED on for my dev board
        self.led.set_high().unwrap();
    }

    fn led_on(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::LedOff),
        }
    }

    fn led_off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::LedOn),
        }
    }
}
