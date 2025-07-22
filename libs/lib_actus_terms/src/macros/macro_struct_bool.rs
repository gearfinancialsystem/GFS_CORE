#[macro_export]
macro_rules! define_struct_bool {
    ($struct_name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $struct_name(bool);

        impl $struct_name {
            // Create a new instance of the struct with a boolean value
            pub fn new(value: bool) -> Result<Self, String> {
                Ok($struct_name(value))
            }

            // Get the boolean value
            pub fn value(&self) -> bool {
                self.0
            }

            // Set the boolean value
            pub fn set_value(&mut self, value: bool) {
                self.0 = value;
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None, // If key is absent, return None
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valid value
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }

        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    "true" => Ok(Self(true)),
                    "false" => Ok(Self(false)),
                    _ => Err(format!("Unable to parse '{}' as a boolean", s)),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::util::Value::Value;
    // Define the TestStructs
    define_struct_bool!(TestStructBool);




    // Test for new method
    #[test]
    fn test_teststructbool_new() {
        let valid_true = TestStructBool::new(true);
        assert!(valid_true.is_ok());
        assert_eq!(valid_true.unwrap().value(), true);

        let valid_false = TestStructBool::new(false);
        assert!(valid_false.is_ok());
        assert_eq!(valid_false.unwrap().value(), false);
    }

    // Test for value method
    #[test]
    fn test_teststructbool_value() {
        let ts_true = TestStructBool::new(true).unwrap();
        assert_eq!(ts_true.value(), true);

        let ts_false = TestStructBool::new(false).unwrap();
        assert_eq!(ts_false.value(), false);
    }

    // Test for set_value method
    #[test]
    fn test_teststructbool_set_value() {
        let mut ts = TestStructBool::new(true).unwrap();
        ts.set_value(false);
        assert_eq!(ts.value(), false);

        ts.set_value(true);
        assert_eq!(ts.value(), true);
    }

    // Test for provide_from_input_dict method
    #[test]
    fn test_teststructbool_provide_from_input_dict() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::from_string("true".to_string()));
        let ts = TestStructBool::provide_from_input_dict(&map, "key").unwrap();
        assert_eq!(ts.value(), true);

        let mut map_false = HashMap::new();
        map_false.insert("key".to_string(), Value::from_string("false".to_string()));
        let ts_false = TestStructBool::provide_from_input_dict(&map_false, "key").unwrap();
        assert_eq!(ts_false.value(), false);

        let mut map_empty = HashMap::new();
        let ts_none = TestStructBool::provide_from_input_dict(&map_empty, "key");
        assert!(ts_none.is_none());
    }

    // Test for FromStr implementation
    #[test]
    fn test_teststructbool_from_str() {
        let valid_str_true = "true";
        let parsed_ts_true = TestStructBool::from_str(valid_str_true);
        assert!(parsed_ts_true.is_ok());
        assert_eq!(parsed_ts_true.unwrap().value(), true);

        let valid_str_false = "false";
        let parsed_ts_false = TestStructBool::from_str(valid_str_false);
        assert!(parsed_ts_false.is_ok());
        assert_eq!(parsed_ts_false.unwrap().value(), false);

        let invalid_str = "not_a_bool";
        let parsed_ts_invalid = TestStructBool::from_str(invalid_str);
        assert!(parsed_ts_invalid.is_err());
    }
}
