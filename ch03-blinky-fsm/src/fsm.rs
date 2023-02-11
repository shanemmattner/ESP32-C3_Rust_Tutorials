#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused)]

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio;
use statig::prelude::*;

#[derive(Debug, Default)]
pub struct Blinky {
    // pub led: gpio::Gpio8<gpio::Output>,
    pub led: bool,
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

impl StateMachine for Blinky {

    type State = State;
    type Superstate<'a> = ();
    type Event<'a> = Event;
    const INITIAL: State = State::Off;
    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |_, source, target| {
        println!("Transitioned from {source:?} to {target:?}");
    };
 
}

impl statig::State<Blinky> for State{
    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event) -> Response<Self>{
        match self{
            State::On => blinky.on(event),
            State::Off => blinky.off(event),
        }
    }
}

impl Blinky{

    fn on(&mut self, event: &Event) -> Response<State> {
        self.led = false;
        // Transition to the `off` state.
        Transition(State::Off)
    }

    fn off(&mut self, event: &Event) -> Response<State> {
        self.led = true;
        // Transition to the `on` state.
        Transition(State::On)
    }

}

/*
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
    */
impl Blinky {
    // The `on_transition` callback that will be called after every transition.
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }
}
