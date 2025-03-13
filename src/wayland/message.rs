struct Header {
    object_id: u32,
    opcode: u16
}

struct Body {
    content: Vec<u8>
}

pub struct Message {
    header: Header,
    body: Body,
}

pub impl Message {
    fn new(object_id: u32, opcode: u16, content: Vec<u8>) -> Self {
        Self {
            Header {
                object_id,
                opcode
            },
            Body {
                content
            },
        }
    }

    fn empty() -> Self {
        new(0, 0, Vec::new())
    }

    fn serialize(&self) {
    }

    fn deserialize() -> Self {
    }
}
