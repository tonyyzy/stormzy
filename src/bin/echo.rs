use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use stormzy::message::{Body, EchoOkBody, InitBody, InitOkBody, Message};

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct Node {
    id: Option<String>,
}

impl Node {
    fn new() -> Self {
        Self { id: None }
    }

    fn init(&mut self, src: String, dst: String, body: InitBody) -> Message {
        self.id = Some(body.node_id);
        Message {
            src: dst,
            dest: src,
            body: Body::InitOk(InitOkBody {
                msg_id: COUNTER.fetch_add(1, Ordering::SeqCst).into(),
                in_reply_to: body.msg_id,
            }),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut buf = String::new();
    let mut node = Node::new();
    loop {
        buf.clear();
        if let Ok(_) = stdin.read_line(&mut buf) {
            if let Ok(message) = serde_json::from_str::<Message>(&buf) {
                let reply = match message.body {
                    Body::Init(init_body) => {
                        if node.id.is_some() {
                            panic!("Node is already initialised.")
                        } else {
                            node.init(message.src, message.dest, init_body)
                        }
                    }
                    Body::Echo(echo_body) => Message {
                        src: message.dest,
                        dest: message.src,
                        body: Body::EchoOk(EchoOkBody {
                            echo: echo_body.echo,
                            msg_id: COUNTER.fetch_add(1, Ordering::SeqCst).into(),
                            in_reply_to: echo_body.msg_id,
                        }),
                    },
                    _ => unreachable!(),
                };
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
