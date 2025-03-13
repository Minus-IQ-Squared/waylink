use crate::wayland::Message;

pub struct MessageReader {
    object_id: u32,
    length: u16,
    opcode: u16,
    content: Vec<u8>,
    flushed: bool,
    bytes_read: u16,
    temp: Some<Message>
}

impl MessageReader {
    pub fn new() -> Self {
        Self {
            object_id: 0,
            length: 0,
            opcode: 0,
            content: Vec::new(),
            flushed: true
        }
    }

    fn extract_once(&mut self, bytes: &[u8], start: u16, message: &Message) -> Result<(u16, bool), Err> {
        if self.flushed {
            self.object_id = 0;
            self.length = 0;
            self.opcode = 0;
            self.content.clear();
            self.flushed = false;
            self.bytes_read = 0;
        }

        let mut i = 0;
        let mut remaining_bytes = bytes.len();

        if self.bytes_read == 0 {
            self.object_id = bytes[0..4];
            self.bytes_read = 4;
            i = 4;
        }

        if self.bytes_read == 4 {
            self.length = bytes[i..2];
            self.bytes_read += 2;
            i += 2;
        }

        if self.bytes_read == 6 {
            self.opcode = bytes[i..2];
            self.bytes_read += 2;
            i += 2;
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<Vec<Message>, Err> {
        let mut messages = Vec<Message>::new();
        let mut start = 0;

        while start != bytes.len() {
            if let Some(message) = self.temp {
            } else {
                Message::empty()
            }

            let consumed_bytes, done = extract_once(bytes, start, message)?;

            start += consumed_bytes;

            if done {
                messages.append(message);
                self.temp = None;
            }
        }

        Ok(messages)
    }
}
