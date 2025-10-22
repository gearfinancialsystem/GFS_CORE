use crate::contracts::Swppv::SWPPV;
use crate::events::ContractEvent::ContractEvent;

pub struct SWPPVEventIterator<'a> {
    pub swppv: &'a SWPPV,
    pub index: usize,
}

impl<'a> Iterator for SWPPVEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.swppv.event_timeline.len() {
            let event = &self.swppv.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}