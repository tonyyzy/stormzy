mod message;
mod node;

use message::Message;
use node::Node;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = String::new();
    let mut node = Node::new();
    loop {
        buf.clear();
        if let Ok(_) = stdin.read_line(&mut buf) {
            if let Ok(message) = serde_json::from_str::<Message>(&buf) {
                let reply = message.reply(&mut node);
                if let Ok(mut msg) = serde_json::to_string(&reply) {
                    msg += "\n";
                    let _ = stdout.write(msg.as_bytes());
                } else {
                    panic!("Failed to serialise reply.")
                }
            } else {
                panic!("Failed to parse message {:?}", &buf)
            }
        } else {
            panic!("Failed to read from STDIN")
        }
    }
}
