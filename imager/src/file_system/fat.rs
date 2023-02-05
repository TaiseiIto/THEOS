use {
    std::fmt,
    super::super::binary::Binary,
};

#[derive(Debug)]
pub struct Fat {
}

impl Fat {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        Self {
        }
    }
}

impl Binary for Fat {
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

impl fmt::Display for Fat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

