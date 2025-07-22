use crate::external::data_observers::DataObserver1::ObservedDataPoint;
use crate::external::DataObserved::DataObserver;
use crate::external::event_observers::EventObserver1::EventObserver1;
use crate::state_space::StateSpace::StateSpace;

pub enum EventObserved {
    EventObserved1(EventObserver1)
}

impl EventObserved {
    
    
    
    pub fn get_contract_id(&self) -> String {

        match self {
            Self::EventObserver1(dobs) => dobs.get_contract_id()
        }
    }
    pub fn set_contract_id(&mut self, contract_id: String) {
        match self {
            Self::EventObserver1(dobs) => {
                dobs.set_contract_id(contract_id);
            }
        }
    }
    pub fn get_states(&self) -> StateSpace {
        match self {
            Self::EventObserver1(dobs) => dobs.get_states()
        }
    }
    pub fn set_states(&mut self, states: StateSpace) {
        match self {
            Self::EventObserver1(dobs) => {
                dobs.set_states(states);
            }
        }
    }
    pub fn get_time(&self) -> String {
        match self {
            Self::EventObserver1(dobs) => dobs.get_time()
        }
    }
    pub fn set_time(&mut self, time: String) {
        match self {
            Self::EventObserver1(dobs) => {
                dobs.set_time(time);
            }
        }
    }
    pub fn get_typex(&self) -> String {
        match self {
            Self::EventObserver1(dobs) => dobs.get_typex()
        }
    }
    pub fn set_typex(&mut self, typex: String) {
        match self {
            Self::EventObserver1(dobs) => {
                dobs.set_typex(typex);
            }
        }
    }
    pub fn get_value(&self) -> f64 {
        match self {
            Self::EventObserver1(dobs) => dobs.get_value()
        }
    }
}