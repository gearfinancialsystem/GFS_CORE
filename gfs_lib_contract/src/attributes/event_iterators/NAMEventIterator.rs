use crate::contracts::Nam::NAM;
use crate::events::ContractEvent::ContractEvent;

pub struct NAMEventIterator<'a> {
    pub nam: &'a NAM,
    pub index: usize,
}

impl<'a> Iterator for NAMEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.nam.event_timeline.len() {
            let event = &self.nam.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}