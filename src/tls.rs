use super::*;

use std::net::TcpStream;
use std::io::prelude::*;
use rustls::ClientConnection;

pub(crate) struct Fetch {
}

impl Fetch {
    pub fn new(url: &str, method: Method, _payload: Vec<u8>) -> Self {
        let method = method.as_str();

        let mut stream = TcpStream::connect((url, 443u16))?;
        let mut client = ClientConnection::new(panic!(), panic!()).unwrap();

        

        client.writer().write(b"{method} / HTTP/1.0\r\n\r\n").unwrap();

        loop {
            if client.wants_read() && socket.ready_for_read() {
                client.read_tls(&mut socket).unwrap();
                client.process_new_packets().unwrap();

                let mut plaintext = Vec::new();
                client.reader().read_to_end(&mut plaintext).unwrap();
                io::stdout().write(&plaintext).unwrap();
            }

            if client.wants_write() && socket.ready_for_write() {
                client.write_tls(&mut socket).unwrap();
            }

            socket.wait_for_something_to_happen();
        }

        Self {
        }
    }
}

impl Notifier for Fetch {
    type Event = Result<Option<Vec<u8>>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
    }
}
