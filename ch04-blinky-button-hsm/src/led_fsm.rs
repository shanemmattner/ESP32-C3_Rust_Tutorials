#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]

use esp_idf_hal::{gpio::{Output, PinDriver}, prelude::*};
use esp_idf_hal::gpio;
use statig::prelude::*;

// #[derive(Debug, Default)]
pub struct Blinky<'a> {
    pub led: PinDriver<'a, gpio::Gpio8, Output>,
}

// The event that will be handled by the state machine.
pub enum Event {
    TimerElapsed,
    ButtonPressed,
}
#[derive(Debug)]
pub enum State {
    LedOn,
    LedOff,
    NotBlinking,
}

pub enum Superstate {
    Blinking,
}


impl StateMachine for Blinky<'_> {
    type State = State;
    type Superstate<'a> = Superstate;
    type Event<'a> = Event;
    const INITIAL: State = State::LedOff;
    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |_, source, target| {
        println!("Transitioned from {source:?} to {target:?}");
    };
 
}

impl statig::State<Blinky<'_>> for State{

    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event) -> Response<Self> {
        match self {
            State::LedOn => Blinky::led_on(event),
            State::LedOff => Blinky::led_off(event),
            State::NotBlinking => Blinky::not_blinking(event),
        }
    }

    fn superstate(&mut self) -> Option<Superstate> {
        match self {
            State::LedOn => Some(Superstate::Blinking),
            State::LedOff => Some(Superstate::Blinking),
            State::NotBlinking => None,
        }
    }

    fn call_entry_action(&mut self, blinky: &mut Blinky) {
        match self {
            State::LedOn  => blinky.enter_led_on(),
            State::LedOff  => blinky.enter_led_off(),
            _ => (),
        }
    }
}

impl statig::Superstate<Blinky<'_>> for Superstate {
    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event) -> Response<State> {
        match self {
            Superstate::Blinking => Blinky::blinking(event),
        }
    }
}

impl Blinky<'_>{

    fn enter_led_on(&mut self){
        self.led.set_high().unwrap();
    }

    fn enter_led_off(&mut self){
        self.led.set_low().unwrap();
    }

    fn led_on(event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::LedOff),
            _ => Super,
        }
    }

    fn led_off(event: &Event) -> Response<State> {
        match event {
            Event::TimerElapsed => Transition(State::LedOn),
            _ => Super,
        }
    }

    fn blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::NotBlinking),
            _ => Super,
        }
    }

    fn not_blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::LedOn),
            _ => Super,
        }
    }
}
