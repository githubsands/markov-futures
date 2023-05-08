use futures::future::TryJoinAll;
use futures::{Future, Stream, Sink};
use std::pin::Pin;
use std::io::Error;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{self, Sender, Receiver};

trait MarkovState {
    fn set_state(&self, state: u8) -> Result<(), Error>;
    fn state(&self) -> &u8;
    fn transition(&self) -> Result<(), Error>;
}

pub struct State {
    state: u8,
    receiver: Receiver<u8>,
}

pub struct MarkovMachine<'a> {
    futures: Vec<&'a mut State>,
    transition_matrix: [[f32; 3]; 3],
    buffer: [u8; 9],
    buffer_position: usize,
    receiver: Receiver<u8>,
}

impl<'a> MarkovMachine<'a> {
    fn new(futures: Vec<&'a mut State>) -> (Self, Sender<u8>) {
        let transition_matrix: [[f32; 3]; 3] = [
            [0.2, 0.3, 0.5],
            [0.6, 0.2, 0.2],
            [0.1, 0.4, 0.5],
        ];
        let (sender, receiver) = mpsc::channel(100);
        let markov_machine = MarkovMachine {
            transition_matrix: transition_matrix,
            futures: futures,
            buffer: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            buffer_position: 0,
            receiver,
        };

        (markov_machine, sender)
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

impl Sink<u8> for MarkovMachine<'_> {
    type Error = Error;

    fn poll_ready(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<(), Self::Error>> {
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

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        if this.buffer_position == this.buffer.len() {
            println!("Flushing buffer: {:?}", this.buffer);
            this.buffer_position = 0;
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn poll_close(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}



fn main() {
}
