use crate::attributes::ContractModel::ContractModel;
use crate::attributes::event_iterators::ANNEventIterator::ANNEventIterator;
use crate::attributes::event_iterators::FXOUTEventIterator::FXOUTEventIterator;
use crate::attributes::event_iterators::LAMEventIterator::LAMEventIterator;
use crate::attributes::event_iterators::NAMEventIterator::NAMEventIterator;
use crate::attributes::event_iterators::PAMEventIterator::PAMEventIterator;
use crate::attributes::event_iterators::STKEventIterator::STKEventIterator;
use crate::attributes::event_iterators::SWPPVEventIterator::SWPPVEventIterator;
use crate::events::ContractEvent::ContractEvent;

pub enum EventIterator<'a> {
    ANNEventIterator(ANNEventIterator<'a>),
    FXOUTEventIterator(FXOUTEventIterator<'a>),
    LAMEventIterator(LAMEventIterator<'a>),
    NAMEventIterator(NAMEventIterator<'a>),
    PAMEventIterator(PAMEventIterator<'a>),
    STKEventIterator(STKEventIterator<'a>),
    SWPPVEventIterator(SWPPVEventIterator<'a>),
}

impl<'a> EventIterator<'a> {
    pub fn next(&mut self) -> Option<&ContractEvent> {
        match self {
            EventIterator::LAMEventIterator(c) => { c.next() },
            EventIterator::STKEventIterator(c) => { c.next() },
            EventIterator::PAMEventIterator(c) => { c.next() },
            EventIterator::ANNEventIterator(c) => { c.next() },
            EventIterator::FXOUTEventIterator(c) => { c.next() },
            EventIterator::NAMEventIterator(c) => { c.next() },
            EventIterator::SWPPVEventIterator(c) => { c.next() },
        }
    }
    pub fn get_index(&self) -> usize {
        match self {
            EventIterator::LAMEventIterator(c) => { c.index },
            EventIterator::STKEventIterator(c) => { c.index },
            EventIterator::PAMEventIterator(c) => { c.index },
            EventIterator::ANNEventIterator(c) => { c.index },
            EventIterator::FXOUTEventIterator(c) => { c.index },
            EventIterator::NAMEventIterator(c) => { c.index },
            EventIterator::SWPPVEventIterator(c) => { c.index },
        }
    }

}