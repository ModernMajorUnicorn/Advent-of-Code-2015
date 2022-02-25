use std::collections::HashMap;
use crate::signal_source::SignalSource;

pub trait LogicalInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool;
    fn set_destination(&mut self, destination: String);
    fn get_destination(&self) -> String;
}

pub struct MovInstruction {
    pub source: SignalSource,
    pub destination: String
}

impl LogicalInstruction for MovInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand = self.source.get_signal(wires);

        match operand {
            Some(operand) => wires.insert(self.destination.to_string(), operand),
            None => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}

pub struct AndInstruction {
    pub source_1: SignalSource,
    pub source_2: SignalSource,
    pub destination: String
}

impl LogicalInstruction for AndInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand_1 = self.source_1.get_signal(wires);
        let operand_2 = self.source_2.get_signal(wires);

        match (operand_1, operand_2) {
            (Some(operand_1), Some(operand_2)) => wires.insert(self.destination.to_string(), operand_1 & operand_2),
            _ => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}

pub struct OrInstruction {
    pub source_1: SignalSource,
    pub source_2: SignalSource,
    pub destination: String
}

impl LogicalInstruction for OrInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand_1 = self.source_1.get_signal(wires);
        let operand_2 = self.source_2.get_signal(wires);

        match (operand_1, operand_2) {
            (Some(operand_1), Some(operand_2)) => wires.insert(self.destination.to_string(), operand_1 | operand_2),
            _ => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}

pub struct NotInstruction {
    pub source: SignalSource,
    pub destination: String
}

impl LogicalInstruction for NotInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand = self.source.get_signal(wires);

        match operand {
            Some(operand) => wires.insert(self.destination.to_string(), !operand),
            None => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}

pub struct LShiftInstruction {
    pub source: SignalSource,
    pub shift_count: i64,
    pub destination: String
}

impl LogicalInstruction for LShiftInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand = self.source.get_signal(wires);

        match operand {
            Some(operand) => wires.insert(self.destination.to_string(), operand << self.shift_count),
            None => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}

pub struct RShiftInstruction {
    pub source: SignalSource,
    pub shift_count: i64,
    pub destination: String
}

impl LogicalInstruction for RShiftInstruction {
    fn perform(&self, wires: &mut HashMap<String, u16>) -> bool {
        let operand = self.source.get_signal(wires);

        match operand {
            Some(operand) => wires.insert(self.destination.to_string(), operand >> self.shift_count),
            None => return false,
        };

        true
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn get_destination(&self) -> String { self.destination.to_string() }
}