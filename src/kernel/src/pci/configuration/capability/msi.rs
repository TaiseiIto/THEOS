#[derive(Clone, Debug)]
pub struct MessageControl(u16);

impl From<u16> for MessageControl {
    fn from(message_control: u16) -> Self {
        Self(message_control)
    }
}

