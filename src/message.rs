use serde::{Deserialize, Serialize};

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
    #[serde(rename = "error")]
    Error(ErrorBody),
}
