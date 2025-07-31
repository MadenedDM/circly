pub enum MessageType {
    Connect, // CONC
    Join, // JOIN
    Deny, // DENY
    Quit, // QUIT
    Close, // CLSO
    Kick, // KICK
    Drop, // DROP
    GetEntity, // GETE
    SendEntity, // RESE
    GetPlayer, // GETP
    SendPlayer, // RESP
    GetComponents, // GETC
    SendComponents, // RESC
    GetRegisty, // GREG
    SendRegistry, // SREG
    GetRenderData, // REND
    SendRenderData, // SHOW
}

pub struct MessageHeader {
    code: MessageType,
    id_tag: u8,
    id: [u8; 6],
}

impl MessageHeader {
    pub fn to_bytes(self) -> [u8; 8] {
        [
            message_to_u8(self.code),
            self.id_tag,
            self.id[0],
            self.id[1],
            self.id[2],
            self.id[3],
            self.id[4],
            self.id[5],
        ]
    }
}

pub fn create_id_tag(server: bool) -> u8 {
    let mut buf: u8 = 0u8;

    if server {
        buf += 1;
    }
    buf
}

/// Converts a byte into a message type
pub fn u8_to_message(val: u8) -> Option<MessageType> {
    match val {
        0x00 => Some(MessageType::Connect),
        0x01 => Some(MessageType::Join),
        0x02 => Some(MessageType::Deny),
        0x03 => Some(MessageType::Quit),
        0x04 => Some(MessageType::Close),
        0x05 => Some(MessageType::Kick),
        0x06 => Some(MessageType::Drop),
        0x07 => Some(MessageType::GetEntity),
        0x08 => Some(MessageType::SendEntity),
        0x09 => Some(MessageType::GetPlayer),
        0x0a => Some(MessageType::SendPlayer),
        0x0b => Some(MessageType::GetComponents),
        0x0c => Some(MessageType::SendComponents),
        0x0d => Some(MessageType::GetRegisty),
        0x0e => Some(MessageType::SendRegistry),
        0x0f => Some(MessageType::GetRenderData),
        0x10 => Some(MessageType::SendRenderData),
        _ => None,
    }
}

/// Converts a message type into a byte
pub fn message_to_u8(msg: MessageType) -> u8 {
    match msg {
        MessageType::Connect => 0x00,
        MessageType::Join => 0x01,
        MessageType::Deny => 0x02,
        MessageType::Quit => 0x03,
        MessageType::Close => 0x04,
        MessageType::Kick => 0x05,
        MessageType::Drop => 0x06,
        MessageType::GetEntity => 0x07,
        MessageType::SendEntity => 0x08,
        MessageType::GetPlayer => 0x09,
        MessageType::SendPlayer => 0x0a,
        MessageType::GetComponents => 0x0b,
        MessageType::SendComponents => 0x0c,
        MessageType::GetRegisty => 0x0d,
        MessageType::SendRegistry => 0x0e,
        MessageType::GetRenderData => 0x0f,
        MessageType::SendRenderData => 0x10,
    }
}
