use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::collections::HashMap;
use serde_json::{Value as SerdeValue};
use serde_json::from_str;
use crate::util::CommonUtils::Value;

pub fn read_file(path: &str) ->Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn test_read_and_parse_json(path: &str) -> Result<SerdeValue, Box<dyn Error>> {
    let contents = read_file(path)?;
    let json: SerdeValue = from_str(&contents)?;
    Ok(json)
}

pub fn convert_value_map_to_string_map_ref(
    value_maps: &Vec<HashMap<String, Value>>
) -> Vec<HashMap<String, String>> {
    value_maps.iter()
        .map(|map| {
            map.iter()
                .map(|(key, value)| (key.clone(), custom_value_to_types(value.clone())))
                .collect()
        })
        .collect()
}
pub fn custom_value_to_types(value: Value) -> String {
    if let Value::String(s) = value {
        return s;
    }
    if let Value::F64(f) = value {
        return format!("{:.2}", f);
    }
    "".to_string()
}

pub fn serde_value_to_custom_value(value: SerdeValue) -> Value {
    match value {
        SerdeValue::Object(map) => {
            if map.is_empty() {
                // Cas spécial pour les objets vides comme dataObserved
                Value::HashMap(HashMap::new())
            } else {
                let mut hm = HashMap::new();
                for (k, v) in map {
                    // Traitement spécial pour eventsObserved si c'est un tableau vide
                    if k == "eventsObserved" {
                        if let SerdeValue::Array(vec) = v.clone() {
                            if vec.is_empty() {
                                hm.insert(k, Value::VecVal(Vec::new()));
                                continue;
                            }
                        }
                    }
                    hm.insert(k, serde_value_to_custom_value(v));
                }
                Value::HashMap(hm)
            }
        }
        SerdeValue::Array(vec) => {
            if vec.is_empty() {
                // Traitement uniforme pour tous les tableaux vides
                Value::VecVal(Vec::new())
            } else {
                let first_element = &vec[0];
                if first_element.is_object() {
                    let v: Vec<HashMap<String, Value>> = vec.into_iter()
                        .map(|v| {
                            if let SerdeValue::Object(map) = v {
                                let mut hm = HashMap::new();
                                for (k, v) in map {
                                    hm.insert(k, serde_value_to_custom_value(v));
                                }
                                hm
                            } else {
                                let mut hm = HashMap::new();
                                hm.insert("error".to_string(), Value::String("Expected an object".to_string()));
                                hm
                            }
                        })
                        .collect();
                    Value::Vec(v)
                } else if first_element.is_string() {
                    let v: Vec<String> = vec.into_iter()
                        .map(|v| {
                            if let SerdeValue::String(s) = v {
                                s
                            } else {
                                "".to_string()
                            }
                        })
                        .collect();
                    Value::VecStr(v)
                } else if first_element.is_number() {
                    let v: Vec<f64> = vec.into_iter()
                        .map(|v| {
                            if let SerdeValue::Number(n) = v {
                                n.as_f64().unwrap_or(0.0)
                            } else {
                                0.0
                            }
                        })
                        .collect();
                    Value::VecReal(v)
                } else {
                    // Par défaut, traiter comme un VecVal avec des valeurs converties
                    let v: Vec<Value> = vec.into_iter()
                        .map(serde_value_to_custom_value)
                        .collect();
                    Value::VecVal(v)
                }
            }
        }
        SerdeValue::String(s) => Value::String(s),
        SerdeValue::Number(n) => Value::F64(n.as_f64().unwrap()),
        SerdeValue::Bool(b) => Value::String(b.to_string()),
        SerdeValue::Null => Value::None,
    }
}
pub fn json_to_dico(value: SerdeValue) -> Vec<Value> {
    if let SerdeValue::Object(map) = value {
        //let mut dico = HashMap::new();
        let mut vec: Vec<Value> = Vec::new();
        for (k, v) in map {
            println!("{}", k);
            vec.push(serde_value_to_custom_value(v));
            //dico.insert(k, serde_value_to_custom_value(v));
        }
        vec
    } else {
        panic!("Expected a JSON object at the root");
    }
}
