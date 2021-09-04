extern crate minion;

use core::time;
use std::{{self, prelude::*}, io, net, thread};
use minion::Cancellable;
use std::io::Write;

struct Service(net::TcpListener);

impl minion::Cancellable for Service {
    type Error = io::Error;
    fn for_each(&mut self) -> Result<minion::LoopState, Self::Error> {
        let mut stream = self.0.accept()?.0;
        write!(stream, "Hello, world!")?;
        Ok(minion::LoopState::Continue)
    }
}

impl Service {
    fn new() -> Self {
        let listener = net::TcpListener::bind("127.0.0.1:6556").unwrap();
        Service(listener)
    }
}

fn main() {
    let service = Service::new();
    eprintln!("Service running");
    let handler = service.spawn();
    let exit = handler.canceller();
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(10));
        eprintln!("Service terminating");
        exit.cancel();
    });
    handler.wait().unwrap();
    eprintln!("Service terminated");
}
