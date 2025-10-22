use crate::contracts::Lam::LAM;
use crate::events::ContractEvent::ContractEvent;

pub struct LAMEventIterator<'a> {
    pub lam: &'a LAM,
    pub index: usize,
}

impl<'a> Iterator for LAMEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.lam.event_timeline.len() {
            let event = &self.lam.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}