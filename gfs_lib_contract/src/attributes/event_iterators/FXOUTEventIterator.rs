use crate::contracts::Fxout::FXOUT;
use crate::events::ContractEvent::ContractEvent;

pub struct FXOUTEventIterator<'a> {
    pub fxout: &'a FXOUT,
    pub index: usize,
}

impl<'a> Iterator for FXOUTEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.fxout.event_timeline.len() {
            let event = &self.fxout.event_timeline[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}