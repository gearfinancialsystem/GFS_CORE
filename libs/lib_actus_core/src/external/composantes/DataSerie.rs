use crate::external::composantes::ObservedDataPoint::ObservedDataPoint;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct DataSerie {
    pub identifier: String,
    pub data_serie: Vec<ObservedDataPoint>
}