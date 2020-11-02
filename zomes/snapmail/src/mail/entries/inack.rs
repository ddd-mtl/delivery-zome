use hdk3::prelude::*;

/// Entry for a received Acknowledgement Receipt
#[hdk_entry(id = "inack")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct InAck {
    // n/a
}

impl InAck {
    pub fn new() -> Self {
        Self {
        }
    }
}
