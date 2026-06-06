#[derive(Debug, Clone)]
pub struct MultiField {
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct PublicField {
    pub message: String,
    pub from_id: u16,
}

#[derive(Debug, Clone)]
pub struct PrivateField {
    pub receiver: u16,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum TcpMessage {
    PingMessage,
    SensorValueIntMessage(i64),
    SensorValueFloatMessage(f64),
    MultiFieldMessage(MultiField),
    PublicMessage(PublicField),
    PrivateMessage(PrivateField),
    TuiStatusMessage(String),
}

pub type TcpLength = u16;

impl TcpMessage {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            TcpMessage::PingMessage => vec![0x01],
            TcpMessage::SensorValueIntMessage(n) => {
                let mut msg: Vec<u8> = vec![0x02];
                msg.append(&mut n.to_be_bytes().to_vec());
                msg
            },
            TcpMessage::SensorValueFloatMessage(n) => {
                let mut msg: Vec<u8> = vec![0x03];
                msg.append(&mut n.to_be_bytes().to_vec());
                msg
            },
            TcpMessage::MultiFieldMessage(m) => {
                // |TcpCmd(u8)|NameLength(u16)|Name(var)|value(u32)|
                let mut msg: Vec<u8> = vec![0x04];

                let name_bytes = m.name.as_bytes();
                msg.extend((name_bytes.len() as TcpLength).to_be_bytes());
                msg.extend(name_bytes);
                msg.extend(m.value.to_be_bytes());
                msg
            },
            TcpMessage::PublicMessage(public_field) => {
                // |TcpCmd(u8)|MessageLength(u16)|Message(var)|from_id(u16)|
                let mut msg: Vec<u8> = vec![0x05];
                let message_bytes= public_field.message.as_bytes();
                msg.extend((message_bytes.len() as TcpLength).to_be_bytes());
                msg.extend(message_bytes);
                msg.extend(public_field.from_id.to_be_bytes());
                msg
            },
            TcpMessage::PrivateMessage(private_field) => {
                // |TcpCmd(u8)|RxId(u16)|MessageLength(u16)|Message(var)|
                let mut msg: Vec<u8> = vec![0x06];
                let msg_bytes= private_field.message.as_bytes();

                msg.extend(private_field.receiver.to_be_bytes());
                msg.extend((msg_bytes.len() as TcpLength).to_be_bytes());
                msg.extend(msg_bytes);
                msg
            }
            TcpMessage::TuiStatusMessage(status_message) => {
                // |TcpCmd(u8)|MessageLength(u16)|Message(var)|

                let mut msg: Vec<u8> = vec![0x07];
                let msg_bytes= status_message.as_bytes();
                msg.extend((msg_bytes.len() as TcpLength).to_be_bytes());
                msg.extend(msg_bytes);
                msg
            }
        }
    }

    pub fn decode(data: &[u8]) -> Option<TcpMessage> {
        match data.first()? {
            // PingMessage
            0x01 => Some(TcpMessage::PingMessage),

            // SensorValueIntMessage
            0x02 => {
                let value_bytes: [u8; 8] = data.get(1..9)?.try_into().ok()?;
                let value = i64::from_be_bytes(value_bytes);
                Some(TcpMessage::SensorValueIntMessage(value))
            },

            // SensorValueFloatMessage
            0x03 => {
                let value_bytes: [u8; 8] = data.get(1..9)?.try_into().ok()?;
                let value = f64::from_be_bytes(value_bytes);
                Some(TcpMessage::SensorValueFloatMessage(value))
            },

            // MultiFieldMessage
            0x04 => {
                let name_length = TcpLength::from_be_bytes(data.get(1..3)?.try_into().ok()?) as usize;

                let name_start = 3;
                let name_end = name_start + name_length;

                let name = String::from_utf8(
                    data.get(name_start..name_end)?.to_vec()
                ).ok()?;

                let value = u32::from_be_bytes(data.get(name_end..name_end + 4)?.try_into().ok()?);

                Some(TcpMessage::MultiFieldMessage(
                    MultiField {
                        name,
                        value,
                    }
                ))
            },

            // PublicMessage
            0x05 => {
                let message_length = TcpLength::from_be_bytes(data.get(1..3)?.try_into().ok()?) as usize;

                let message_start = 3;
                let message_end = message_start + message_length;

                let message = String::from_utf8(
                    data.get(message_start..message_end)?.to_vec()
                ).ok()?;

                let id = u16::from_be_bytes(data.get(message_end..message_end + 2)?.try_into().ok()?);

                Some(TcpMessage::PublicMessage(PublicField { message, from_id: id }))
            }

            // PrivateMessage
            0x06 => {
                let message_length = TcpLength::from_be_bytes(data.get(3..5)?.try_into().ok()?) as usize;

                let message_start = 5;
                let message_end = message_start + message_length;

                let receiver = u16::from_be_bytes(data.get(1..3)?.try_into().ok()?);
                let message = String::from_utf8(
                    data.get(message_start..message_end)?.to_vec()
                ).ok()?;

                Some(TcpMessage::PrivateMessage( PrivateField { receiver, message }))
            }

            // TuiStatusMessage
            0x07 => {
                let message_length = TcpLength::from_be_bytes(data.get(1..3)?.try_into().ok()?) as usize;
                let message_start = 3;
                let message_end = message_start + message_length;

                let message = String::from_utf8(
                    data.get(message_start..message_end)?.to_vec()
                ).ok()?;

                Some(TcpMessage::TuiStatusMessage(message))

            }
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TcpMessage::PingMessage => "Ping".to_string(),
            TcpMessage::SensorValueIntMessage(n) => "SensorValueInt coming soon...".to_string(),
            TcpMessage::SensorValueFloatMessage(n) => "SensorValueFloat coming soon...".to_string(),
            TcpMessage::MultiFieldMessage(m) => format!("Name: {}, Value: {}", m.value, m.name).to_string(),
            TcpMessage::PublicMessage(p) => format!("Message: {} from: {}", p.message, p.from_id).to_string(),
            TcpMessage::PrivateMessage(p) => "PrivateMessage coming soon...".to_string(),
            TcpMessage::TuiStatusMessage(status_message) => format!("{}", status_message).to_string(),
        }
    }
}