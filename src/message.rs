use crate::database::DatabaseKey;

pub const KEY_LENGTH: usize = 32;
pub const MAX_VALUE_LENGTH: usize = 4096;
pub const MAX_MESSAGE_LENGTH: usize = 3 + KEY_LENGTH + MAX_VALUE_LENGTH;
pub const MIN_MESSAGE_LENGTH: usize = 3 + KEY_LENGTH;

#[derive(Debug)]
pub enum Command {
    Get,
    Put,
    Keep,
    Ok,
    NOk,
}

impl Command {
    pub fn to_byte(&self) -> u8 {
        match self {
            Command::Get => "G".as_bytes()[0],
            Command::Put => "P".as_bytes()[0],
            Command::Keep => "K".as_bytes()[0],
            Command::Ok => "O".as_bytes()[0],
            Command::NOk => "N".as_bytes()[0],
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub command: Command,
    pub key: DatabaseKey,
    pub value: Vec<u8>,
}

impl Message {
    pub fn pack(&self) -> Result<Vec<u8>, String> {
        if self.value.len() > MAX_VALUE_LENGTH {
            return Err("value too long".to_string());
        }

        let mut buf = Vec::with_capacity(MAX_MESSAGE_LENGTH);
        let length = ((3 + KEY_LENGTH + self.value.len()) as u16).to_be_bytes();
        buf.push(length[1]);
        buf.push(length[0]);
        buf.push(self.command.to_byte());
        buf.extend_from_slice(&self.key);
        buf.extend_from_slice(&self.value);
        Ok(buf)
    }

    pub fn unpack(buf: &[u8]) -> Result<Message, String> {
        if buf.len() < MIN_MESSAGE_LENGTH {
            return Err("message too short".to_string());
        }
        if buf.len() > MAX_MESSAGE_LENGTH {
            return Err("message too long".to_string());
        }

        let length = u16::from_be_bytes([buf[0], buf[1]]) as usize;

        if buf.len() != length {
            return Err(format!(
                "message length mismatch: expected {}, got {}",
                length,
                buf.len()
            ));
        }

        let command = match buf[2] as char {
            'G' => Command::Get,
            'P' => Command::Put,
            'K' => Command::Keep,
            'O' => Command::Ok,
            'N' => Command::NOk,
            _ => return Err("Invalid command".to_string()),
        };

        let mut key: DatabaseKey = [0; KEY_LENGTH];
        key.copy_from_slice(&buf[3..3 + KEY_LENGTH]);
        let value = buf[3 + KEY_LENGTH..].to_vec();

        Ok(Message {
            command,
            key,
            value,
        })
    }
}
