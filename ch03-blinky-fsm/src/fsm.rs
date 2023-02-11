#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused)]

use esp_idf_hal::{gpio::{Output, PinDriver}, prelude::*};
use esp_idf_hal::gpio;
use statig::prelude::*;

// #[derive(Debug, Default)]
pub struct Blinky<'a> {
    pub led: PinDriver<'a, gpio::Gpio8, Output>,
}

// The event that will be handled by the state machine.
#[derive(Debug)]
pub enum Event {
    TimerElapsed,
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

impl StateMachine for Blinky<'_> {

    type State = State;
    type Superstate<'a> = ();
    type Event<'a> = Event;
    const INITIAL: State = State::Off;
    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |_, source, target| {
        println!("Transitioned from {source:?} to {target:?}");
    };
 
}

impl statig::State<Blinky<'_>> for State{
    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event) -> Response<Self>{
        match self{
            State::On => blinky.on(event),
            State::Off => blinky.off(event),
        }
    }
}

impl Blinky<'_>{
    fn on(&mut self, event: &Event) -> Response<State> {
        self.led.set_low().unwrap();
        Transition(State::Off)
    }

    fn off(&mut self, event: &Event) -> Response<State> {
        self.led.set_high().unwrap();
        Transition(State::On)
    }

    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }

}
