use crate::util_tests::ObservedDataPoint::ObservedDataPoint;

#[derive(PartialEq, Debug, Clone)]
pub struct ObservedDataSet {
    identifier: String,
    data: Vec<ObservedDataPoint>
}

impl ObservedDataSet {
    pub fn new() -> ObservedDataSet {
        ObservedDataSet {identifier: String::new(), data: Vec::new()}
    }
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }
    pub fn get_data(&self) -> Vec<ObservedDataPoint> {
        self.data.clone()
    }
    pub fn set_data(&mut self, data: Vec<ObservedDataPoint>) {
        self.data = data;
    }
}