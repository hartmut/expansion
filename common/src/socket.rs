//! One-way socket wrapper data structures.

use kafka::client::{KafkaClient;
use kafka::utils;
use std::convert::AsRef;
use std::io::{Read, Write};
use std::time::Duration;

/// A send-only socket.
pub struct SendSocket {
  socket: Socket,
  endpoint: Endpoint,
}

fn as_millis(duration: Duration) -> isize {
  (duration.as_secs() * 1_000) as isize + (duration.subsec_nanos() / 1_000_000) as isize
}

impl SendSocket {
  #[allow(missing_docs)]
  pub fn new(url: &str, timeout: Option<Duration>) -> SendSocket {
    let mut socket = Socket::new(Protocol::Push).unwrap();
    timeout.map(|timeout| socket.set_receive_timeout(as_millis(timeout)).unwrap());
    let endpoint = socket.connect(url).unwrap();

    SendSocket {
      socket: socket,
      endpoint: endpoint,
    }
  }

  /// Block until we can send this socket a message.
  pub fn write(&mut self, msg: &[u8]) {
    self.socket.write(msg).unwrap();
  }

  /// Terminate this connection.
  pub fn close(self) {
    // The `drop` takes care of everything.
  }
}

impl Drop for SendSocket {
  fn drop(&mut self) {
    self.endpoint.shutdown().unwrap();
  }
}

/// A receive-only socket.
pub struct ReceiveSocket {
  socket: Socket,
  endpoint: Endpoint,
}

impl ReceiveSocket {
  #[allow(missing_docs)]
  pub fn new(url: &str, timeout: Option<Duration>) -> ReceiveSocket {
    let mut socket = Socket::new(Protocol::Pull).unwrap();
    timeout.map(|timeout| socket.set_receive_timeout(as_millis(timeout)).unwrap());
    let endpoint = socket.bind(url.as_ref()).unwrap();

    ReceiveSocket {
      socket: socket,
      endpoint: endpoint,
    }
  }

  /// Block until a message can be fetched from this socket.
  pub fn read(&mut self) -> Vec<u8> {
    let mut msg = Vec::new();
    self.socket.read_to_end(&mut msg).unwrap();
    msg
  }

  /// Terminate this connection.
  pub fn close(self) {
    // The `drop` takes care of everything.
  }
}

impl Drop for ReceiveSocket {
  fn drop(&mut self) {
    self.endpoint.shutdown().unwrap();
  }
}
