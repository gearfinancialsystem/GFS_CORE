// use std::fs::File;
// use std::io::Read;
// use std::error::Error;
// use std::collections::HashMap;
// use serde_json::{Value as SerdeValue};
// use serde_json::from_str;
// use crate::utils::Value::Value;
// use std::io::BufReader;
//
// pub fn read_file(path: &str) ->Result<String, Box<dyn Error>> {
//     let mut file = File::open(path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
// pub fn test_read_and_parse_json(path: &str) -> Result<SerdeValue, Box<dyn Error>> {
//     let contents = read_file(path)?;
//     let json: SerdeValue = from_str(&contents)?;
//     Ok(json)
// }
//
// pub fn convert_value_map_to_string_map_ref(
//     value_maps: &Vec<HashMap<String, Value>>
// ) -> Vec<HashMap<String, String>> {
//     value_maps.iter()
//         .map(|map| {
//             map.iter()
//                 .map(|(key, value)| (key.clone(), custom_value_to_types(value.clone())))
//                 .collect()
//         })
//         .collect()
// }
// pub fn custom_value_to_types(value: Value) -> String {
//     if let Value::Vstring(s) = value {
//         return s;
//     }
//     if let Value::Vf64(f) = value {
//         return format!("{:.2}", f);
//     }
//     "".to_string()
// }
//
// pub fn serde_value_to_custom_value(value: SerdeValue) -> Value {
//     match value {
//         SerdeValue::Object(map) => {
//             if map.is_empty() {
//                 // Cas spécial pour les objets vides comme dataObserved
//                 Value::VhashMap(HashMap::new())
//             } else {
//                 let mut hm = HashMap::new();
//                 for (k, v) in map {
//                     // Traitement spécial pour eventsObserved si c'est un tableau vide
//                     if k == "eventsObserved" {
//                         if let SerdeValue::Array(vec) = v.clone() {
//                             if vec.is_empty() {
//                                 hm.insert(k, Value::VvecVal(Vec::new()));
//                                 continue;
//                             }
//                         }
//                     }
//                     hm.insert(k, serde_value_to_custom_value(v));
//                 }
//                 Value::VhashMap(hm)
//             }
//         }
//         SerdeValue::Array(vec) => {
//             if vec.is_empty() {
//                 // Traitement uniforme pour tous les tableaux vides
//                 Value::VvecVal(Vec::new())
//             } else {
//                 let first_element = &vec[0];
//                 if first_element.is_object() {
//                     let v: Vec<Value> = vec.into_iter()
//                         .map(|v| {
//                             if let SerdeValue::Object(map) = v {
//                                 let mut hm = HashMap::new();
//                                 for (k, v) in map {
//                                     hm.insert(k, serde_value_to_custom_value(v));
//                                 }
//                                 Value::VhashMap(hm)
//                             } else {
//                                 let mut hm = HashMap::new();
//                                 hm.insert("error".to_string(), Value::Vstring("Expected an object".to_string()));
//                                 Value::VhashMap(hm)
//                             }
//                         })
//                         .collect();
//                     Value::VvecVal(v)
//                 } else if first_element.is_string() {
//                     let v: Vec<Value> = vec.into_iter()
//                         .map(|v| {
//                             if let SerdeValue::String(s) = v {
//                                 Value::Vstring(s)
//                             } else {
//                                 Value::Vstring("".to_string())
//                             }
//                         })
//                         .collect();
//                     Value::VvecVal(v)
//                 } else if first_element.is_number() {
//                     let v: Vec<Value> = vec.into_iter()
//                         .map(|v| {
//                             if let SerdeValue::Number(n) = v {
//                                 Value::Vf64(n.as_f64().unwrap_or(0.0))
//                             } else {
//                                 Value::Vf64(0.0)
//                             }
//                         })
//                         .collect();
//                     Value::VvecVal(v)
//                 } else {
//                     // Par défaut, traiter comme un VecVal avec des valeurs converties
//                     let v: Vec<Value> = vec.into_iter()
//                         .map(serde_value_to_custom_value)
//                         .collect();
//                     Value::VvecVal(v)
//                 }
//             }
//         }
//         SerdeValue::String(s) => Value::Vstring(s),
//         SerdeValue::Number(n) => Value::Vf64(n.as_f64().unwrap()),
//         SerdeValue::Bool(b) => Value::Vstring(b.to_string()),
//         SerdeValue::Null => Value::None,
//     }
// }
//
// pub fn json_to_dico(value: SerdeValue) -> Vec<Value> {
//     if let SerdeValue::Object(map) = value {
//         //let mut dico = HashMap::new();
//         let mut vec: Vec<Value> = Vec::new();
//         for (k, v) in map {
//             
//             vec.push(serde_value_to_custom_value(v));
//             //dico.insert(k, serde_value_to_custom_value(v));
//         }
//         vec
//     } else {
//         panic!("Expected a JSON object at the root");
//     }
// }
//
//
//
//
