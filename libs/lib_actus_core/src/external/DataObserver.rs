use crate::external::data_observers::DataObserver1::{DataObserver1,
                                                     ObservedDataPoint};


#[derive(PartialEq, Debug, Clone)]
pub enum DataObserver {
    DataObserver1(DataObserver1)
}

impl DataObserver {

    pub fn get_identifier(&self) -> String {
        match self {
            Self::DataObserver1(dobs) => dobs.get_identifier()
        }
    }
    
    pub fn set_identifier(&mut self, identifier: String) {
        match self {
            Self::DataObserver1(dobs) => {
                dobs.set_identifier(identifier);
            }
        }
    }
    
    pub fn get_data(&self) -> Vec<ObservedDataPoint> {
        match self {
            Self::DataObserver1(dobs) => dobs.get_data()
        }
    }
    
    pub fn set_data(&mut self, data: Vec<ObservedDataPoint>) {
        match self {
            Self::DataObserver1(dobs) => {
                dobs.set_data(data);
            }
        }
    }
}



