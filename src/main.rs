use futures::future::TryJoinAll;
use futures::StreamExt;
use futures::{Future, Sink, Stream};
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

trait MarkovState {
    fn set_state(&self, state: u8) -> Result<(), Error>;
    fn state(&self) -> &u8;
    fn transition(&self) -> Result<(), Error>;
}

pub struct State {
    state: [u8; 2],
    producer: Option<Sender<StateMsg>>,
}

impl State {
    fn new(state: [u8; 2]) -> Self {
        Self {
            state: state,
            producer: None,
        }
    }
    fn add_producer(&mut self, producer: Sender<StateMsg>) {
        self.producer = Some(producer)
    }
}

struct StateMsg {
    state: usize,
    x: f32,
    y: f32,
}
/*
impl Stream for State {
    type Item = &'static str;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&'static str>> {
        if self.state == 1 {
            Poll::Ready("ready");
        }
        Poll::Pending
    }
}
*/

pub struct MarkovMachine<'a> {
    futures: Option<Vec<&'a mut State>>,
    transition_matrix: [[f32; 3]; 3],
    buffer: [u8; 9],
    buffer_position: usize,
    receiver: Receiver<StateMsg>,
}

impl<'a> MarkovMachine<'a> {
    fn new() -> (Self, Sender<StateMsg>) {
        let transition_matrix: [[f32; 3]; 3] = [[0.2, 0.3, 0.5], [0.6, 0.2, 0.2], [0.1, 0.4, 0.5]];
        let (sender, receiver) = channel(1);
        let markov_machine = MarkovMachine {
            transition_matrix: transition_matrix,
            futures: None,
            buffer: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            buffer_position: 0,
            receiver,
        };

        (markov_machine, sender)
    }
    fn states(&self) -> usize {
        let length_width = self.transition_matrix[0].len();
        let length_height = self.transition_matrix.len();
        return length_width * length_height;
    }
    fn add_states(&mut self, states: Vec<&'a mut State>) {
        self.futures = Some(states)
    }
    async fn update_transition_probabilities(&mut self) {
        while let Some(parameters) = self.receiver.recv().await {
            let result = task::spawn_blocking(move || compute(parameters.x, parameters.y)).await;
        }
    }
}

fn compute(x: f32, y: f32) -> f32 {
    x / y
}

/*
impl Sink<u8> for MarkovMachine<'_> {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        if this.buffer_position < this.buffer.len() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn start_send(self: Pin<&mut Self>, item: u8) -> Result<(), Self::Error> {
        let this = self.get_mut();
        if this.buffer_position < this.buffer.len() {
            this.buffer[this.buffer_position] = item;
            this.buffer_position += 1;
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::WouldBlock, "Buffer is full"))
        }
    }

    fn poll_flush(self: Pin<&mut Self:>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        if this.buffer_position == this.buffer.len() {
            println!("Flushing buffer: {:?}", this.buffer);
            this.buffer_position = 0;
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
*/

fn main() {
    let (markov_machine, producer) = MarkovMachine::new();
    let states: Vec<State> = Vec::new();
    for i in 0..markov_machine.states() {
        let state = State::new([0, 0]).add_producer(producer.clone());
    }
}
