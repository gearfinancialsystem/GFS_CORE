use crate::contracts::Stk::STK;
use crate::events::ContractEvent::ContractEvent;

pub struct STKEventIterator<'a> {
    pub stk: &'a STK,
    pub index: usize,
}

impl<'a> Iterator for STKEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stk.event_timeline.len() {
            let event = &self.stk.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}