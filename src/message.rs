use crate::node::Node;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
static UNIQUE_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct InitBody {
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InitOkBody {
    pub in_reply_to: u64,
    pub msg_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadBody {
    pub msg_id: u64,
    pub key: i32,
}
#[derive(Serialize, Deserialize)]
pub struct ReadOkBody {
    pub msg_id: u64,
    pub in_reply_to: u64,
    pub value: i32,
}

#[derive(Serialize, Deserialize)]
pub struct EchoBody {
    pub echo: String,
    pub msg_id: u64,
}
#[derive(Serialize, Deserialize)]
pub struct EchoOkBody {
    pub echo: String,
    pub msg_id: u64,
    pub in_reply_to: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateBody {
    pub msg_id: u64,
}
#[derive(Serialize, Deserialize)]
pub struct GenerateOkBody {
    pub msg_id: u64,
    pub in_reply_to: u64,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorBody {
    pub in_reply_to: u64,
    pub code: i32,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Body {
    #[serde(rename = "init")]
    Init(InitBody),
    #[serde(rename = "init_ok")]
    InitOk(InitOkBody),
    #[serde(rename = "echo")]
    Echo(EchoBody),
    #[serde(rename = "echo_ok")]
    EchoOk(EchoOkBody),
    #[serde(rename = "read")]
    Read(ReadBody),
    #[serde(rename = "read_ok")]
    ReadOk(ReadOkBody),
    #[serde(rename = "generate")]
    Generate(GenerateBody),
    #[serde(rename = "generate_ok")]
    GenerateOk(GenerateOkBody),
    #[serde(rename = "error")]
    Error(ErrorBody),
}

fn generate_msg_id() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst).into()
}

fn generate_unique_id(node: &Node) -> String {
    if let Some(node_id) = node.id.clone() {
        node_id + &format!("-{:0>10}", UNIQUE_ID.fetch_add(1, Ordering::SeqCst))
    } else {
        panic!("Node is not initialised.")
    }
}

impl Message {
    pub fn reply(self, node: &mut Node) -> Message {
        Message {
            src: self.dest,
            dest: self.src,
            body: self.body.reply(node),
        }
    }
}

impl Body {
    fn reply(self, node: &mut Node) -> Body {
        let msg_id = generate_msg_id();
        match self {
            Body::Init(body) => {
                node.init(body.node_id);
                Body::InitOk(InitOkBody {
                    msg_id,
                    in_reply_to: body.msg_id,
                })
            }
            Body::Echo(body) => Body::EchoOk(EchoOkBody {
                msg_id,
                in_reply_to: body.msg_id,
                echo: body.echo,
            }),
            Body::Generate(body) => Body::GenerateOk(GenerateOkBody {
                msg_id,
                in_reply_to: body.msg_id,
                id: generate_unique_id(node),
            }),
            _ => unreachable!(),
        }
    }
}
