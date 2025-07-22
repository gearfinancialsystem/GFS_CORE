use serde_json::{self, Value as JsonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use crate::risk_factors::risk_factor_model_1::composantes::DataSerie::DataSerie;
use crate::risk_factors::risk_factor_model_1::composantes::ObservedDataPoint::ObservedDataPoint;

use lib_actus_core::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct DataObserved {
    pub data_series: HashMap<String, DataSerie>,
}

impl DataObserved {
    pub fn new() -> Self {
        Self {
            data_series: HashMap::new(),
        }
    }

    // should use DataSerie & ObservedDataPoint
    pub fn new_from(
        file_path: &str,
        test_case_id: &str,
    ) -> Result<DataObserved, Box<dyn std::error::Error>> { // HashMap<String, DataObserver1>
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
                    let mut data_serie = DataSerie::new();

                    // Set identifier
                    if let Some(JsonValue::String(identifier)) = dataset_obj.get("identifier") {
                        data_serie.set_identifier(identifier.clone());
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

                        data_serie.set_data_serie(points);
                    } else {
                        return Err("Missing data array in observed dataset".into());
                    }

                    result.insert(market_object_code.clone(), data_serie);
                } else {
                    return Err("Invalid dataset format".into());
                }
            }

            Ok(Self {
                data_series : result
            })
        } else {
            Err("dataObserved should be an object".into())
        }
    }
}




