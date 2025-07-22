use crate::external::composantes::ObservedDataPoint::ObservedDataPoint;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct DataSerie {
    pub identifier: String,
    pub data_serie: Vec<ObservedDataPoint>
}

impl DataSerie {
    
    pub fn new() -> DataSerie {
        DataSerie {
            identifier: String::new(),
            data_serie: Vec::<ObservedDataPoint>::new()
        }
    }
    
    pub fn new_with(identifier: &str, data_serie: Vec<ObservedDataPoint>) -> Self {
        Self {
            identifier: identifier.to_string(),
            data_serie: data_serie
        }
    }
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }


    pub fn get_data_serie(&self) -> Vec<ObservedDataPoint> {
        self.data_serie.clone()
    }

    pub fn set_data_serie(&mut self, data: Vec<ObservedDataPoint>) {
        self.data_serie = data;
    }
}