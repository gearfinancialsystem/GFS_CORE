use crate::contracts::Ann::ANN;
use crate::events::ContractEvent::ContractEvent;

pub struct ANNEventIterator<'a> {
    pub ann: &'a ANN,
    pub index: usize,
}

impl<'a> Iterator for ANNEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ann.event_timeline.len() {
            let event = &self.ann.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}