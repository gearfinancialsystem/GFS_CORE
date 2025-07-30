use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::external::risk_factors::risk_factor_model_1::composantes::ObservedDataPoint::ObservedDataPoint;

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

    pub fn get_data_point_at_specific_timestamps(&self, timestamp: &IsoDatetime) -> Option<ObservedDataPoint> {
        let mut res : Option<ObservedDataPoint> = None;
        println!("timestamp : {:?}", timestamp);
        for e in &self.data_serie {
            println!("e.timestamp : {:?}", &e.timestamp);
            if &e.timestamp == timestamp {
                res = Some(e.clone());
            }
        }
        res // faudra changer ca et passer au HASH MAP car pas efficace
    }
    
    pub fn get_data_serie(&self) -> Vec<ObservedDataPoint> {
        self.data_serie.clone()
    }

    pub fn set_data_serie(&mut self, data: Vec<ObservedDataPoint>) {
        self.data_serie = data;
    }
}