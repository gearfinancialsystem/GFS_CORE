use std::collections::HashMap;
use serde_json::error::Category::Data;
use crate::external::data_observers::DataObserver1::DataObserver1;


#[derive(PartialEq, Debug, Clone)]
pub enum DataObserved {
    DataObserved1(DataObserver1)
}

impl DataObserved {
    pub fn new_from(&self,
                              file_path: &str,
                              test_case_id: &str) -> DataObserved {
        match self {
            DataObserved::DataObserved1(v) => {
                let a = DataObserver1::new_from(file_path, test_case_id).expect("Error loading data");
                DataObserved::DataObserved1(a)
            },
        }
    }
    
}



