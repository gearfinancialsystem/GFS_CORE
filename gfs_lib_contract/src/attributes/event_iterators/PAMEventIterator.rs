use crate::contracts::Pam::PAM;
use crate::events::ContractEvent::ContractEvent;
use crate::traits::TraitContractModel::TraitContractModel;

pub struct PAMEventIterator<'a> {
    pub pam: &'a PAM,
    pub index: usize,
}

impl<'a> Iterator for PAMEventIterator<'a> {
    type Item = &'a ContractEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.pam.event_timeline.len() {
            let event = &self.pam.event_timeline[self.index];

            self.index += 1;
            Some(event)
        } else {
            None
        }
    }
}
