use std::collections::HashMap;
use serde_json::error::Category::Data;
use crate::external::data_observers::DataObserver1::DataObserver1;


#[derive(PartialEq, Debug, Clone)]
pub enum DataObserved {
    DataObserved1(DataObserver1)
}

impl DataObserved {
    pub fn load_data_observed(&self,
                              file_path: &str,
                              test_case_id: &str) -> DataObserver {
        match self {
            DataObserver::DataObserver1(v) => {
                let a = DataObserver1::load_data_observed(file_path, test_case_id).expect("Error loading data");
                DataObserver::DataObserver1(a)
            },
        }
    }

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



