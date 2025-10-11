

#[derive(PartialEq, Debug, Clone)]
pub struct ObservedDataPoint {
    pub timestamp: IsoDatetime,
    pub value: f64
}

impl ObservedDataPoint {
    pub fn new(timestamp: IsoDatetime, value: f64) -> ObservedDataPoint {
        ObservedDataPoint {timestamp, value}
    }
    pub fn get_timestamp(&self) -> IsoDatetime {
        self.timestamp.clone()
    }
    pub fn set_timestamp(&mut self, timestamp: IsoDatetime) {
        self.timestamp = timestamp;
    }
    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    

}
