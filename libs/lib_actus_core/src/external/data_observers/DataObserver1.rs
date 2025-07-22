use serde_json::{self, Value as JsonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use crate::external::composantes::DataSerie::DataSerie;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct DataObserver1 {
    pub data_series: HashMap<IsoDatetime, DataSerie>,
}
  
impl DataObserver1 {
    pub fn new() -> Self {
        Self {identifier: String::new(), data: Vec::new()}
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
    
    pub fn load_data_observed(
        file_path: &str,
        test_case_id: &str,
    ) -> Result<HashMap<String, DataObserver1>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let json: JsonValue = serde_json::from_reader(reader)?;

        let test_case = json.get(test_case_id)
            .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

        let data_observed = test_case.get("dataObserved")
            .ok_or_else(|| format!("'dataObserved' section not found in {}", test_case_id))?;

        if let JsonValue::Object(data_observed_map) = data_observed {
            let mut result = HashMap::new();

            for (market_object_code, dataset) in data_observed_map {
                if let JsonValue::Object(dataset_obj) = dataset {
                    let mut observed_dataset = DataObserver1::new();

                    // Set identifier
                    if let Some(JsonValue::String(identifier)) = dataset_obj.get("identifier") {
                        observed_dataset.set_identifier(identifier.clone());
                    } else {
                        return Err("Missing identifier in observed dataset".into());
                    }

                    // Parse data points
                    if let Some(JsonValue::Array(data_points)) = dataset_obj.get("data") {
                        let mut points = Vec::new();

                        for point in data_points {
                            if let JsonValue::Object(point_obj) = point {
                                let timestamp = point_obj.get("timestamp")
                                    .and_then(|v| IsoDatetime::from_str(v.as_str().unwrap()).ok() )
                                    .ok_or("Missing timestamp in data point")?;

                                let value_str = point_obj.get("value")
                                    .and_then(|v| v.as_str())
                                    .ok_or("Missing value in data point")?;

                                let value = value_str.parse::<f64>()
                                    .map_err(|_| format!("Invalid float value: {}", value_str))?;

                                points.push(ObservedDataPoint::new(timestamp, value));
                            } else {
                                return Err("Invalid data point format".into());
                            }
                        }

                        observed_dataset.set_data(points);
                    } else {
                        return Err("Missing data array in observed dataset".into());
                    }

                    result.insert(market_object_code.clone(), observed_dataset);
                } else {
                    return Err("Invalid dataset format".into());
                }
            }

            Ok(result)
        } else {
            Err("dataObserved should be an object".into())
        }
    }
}




