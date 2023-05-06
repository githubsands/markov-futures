use futures::{Future, Stream};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::pin;
use tokio::runtime::Builder;
use rand::prelude::*;


pub struct State {
    state: u8
}

pub struct MarkovMachine<'a> {
    futures: Vec<&'a mut State>,
}

impl<'a> MarkovMachine<'a> {
    fn new(futures: Vec<&'a mut State>)-> Self {
        MarkovMachine {
            futures: futures,
        }
    }
}

impl State {
    fn new(initial_state: u8) -> Self {
        State {
            state: initial_state,
        }
    }
}

impl Future for State {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if self.state == 1 {
            Poll::Ready("ready");
        }
        Poll::Pending
    }
}

impl Stream for State {
    type Item = &'static str;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&'static str>> {
        if self.state == 1 {
            Poll::Ready("ready");
        }
        Poll::Pending
    }
}

fn main() {
    let mut state_1 = State::new(1);
    let mut state_2 = State::new(2);
    let mut state_3 = State::new(3);
    let mut states  = vec!(&mut state_1, &mut state_2, &mut state_3);
    let markov_machine = MarkovMachine::new(states);
}
