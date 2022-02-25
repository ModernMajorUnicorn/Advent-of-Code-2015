use std::collections::HashMap;

pub struct SignalSource {
    pub is_static: bool,
    pub value: u16,
    pub wire_name: String
}

impl SignalSource {
    pub fn get_signal(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        if self.is_static {
            Some(self.value)
        } else {
            match wires.get(&self.wire_name) {
                Some(x) => Some(*x),
                None => None,
            }
        }
    }
}