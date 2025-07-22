
use crate::external::event_observers::EventObserver1::EventObserver1;

#[derive(Debug, Clone, PartialEq)]
pub enum EventObserved {
    EventObserved1(EventObserver1)
}

impl EventObserved {
    pub fn new() {
        
        
    }
    pub fn new_from(&self,
                    file_path: &str,
                    test_case_id: &str) -> EventObserved {
        match self {
            EventObserved::EventObserved1(v) => {
                let a = EventObserver1::new_from(file_path, test_case_id).expect("Error loading data");
                EventObserved::EventObserved1(a)
            },
        }
    }
}