#[macro_export]
macro_rules! define_struct_f64 {
    // Case with validation conditions and a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {$default_value:expr}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(f64);

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    Ok($struct_name($value))
                }
            }

            pub fn value(&self) -> f64 {
                self.0
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    self.0 = $value;
                    Ok(())
                }
            }

            // Convert to String with rounding to a specific number of decimal places
            pub fn to_string_rounded(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let rounded = (self.0 * factor).round() / factor;
                format!("{:.1$}", rounded, decimals)
            }

            // Convert to String with truncating to a specific number of decimal places
            pub fn to_string_truncated(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let truncated = (self.0 * factor).trunc() / factor;
                format!("{:.1$}", truncated, decimals)
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => Some(Self::default()), // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }

        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name($default_value)
            }
        }

        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.parse::<f64>() {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as f64", s)),
                }
            }
        }
    };

    // Case with validation conditions but without a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(f64);

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    Ok($struct_name($value))
                }
            }

            pub fn value(&self) -> f64 {
                self.0
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    self.0 = $value;
                    Ok(())
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let rounded = (self.0 * factor).round() / factor;
                format!("{:.1$}", rounded, decimals)
            }

            pub fn to_string_truncated(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let truncated = (self.0 * factor).trunc() / factor;
                format!("{:.1$}", truncated, decimals)
            }
            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }
        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.parse::<f64>() {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as f64", s)),
                }
            }
        }
    };

    // Case without validation conditions but with a default value
    ($struct_name:ident, |$value:ident| {}, {$default_value:expr}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(f64);

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    Ok($struct_name($value))
                }
            }

            pub fn value(&self) -> f64 {
                self.0
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    self.0 = $value;
                    Ok(())
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let rounded = (self.0 * factor).round() / factor;
                format!("{:.1$}", rounded, decimals)
            }

            pub fn to_string_truncated(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let truncated = (self.0 * factor).trunc() / factor;
                format!("{:.1$}", truncated, decimals)
            }
            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => Some(Self::default()), // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name($default_value)
            }
        }
        
        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.parse::<f64>() {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as f64", s)),
                }
            }
        }
    };

    // Case without validation conditions and without a default value
    ($struct_name:ident, |$value:ident| {}, {}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(f64);

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    Ok($struct_name($value))
                }
            }

            pub fn value(&self) -> f64 {
                self.0
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    self.0 = $value;
                    Ok(())
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let rounded = (self.0 * factor).round() / factor;
                format!("{:.1$}", rounded, decimals)
            }

            pub fn to_string_truncated(&self, decimals: usize) -> String {
                let factor = 10f64.powi(decimals as i32);
                let truncated = (self.0 * factor).trunc() / factor;
                format!("{:.1$}", truncated, decimals)
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }
        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.parse::<f64>() {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as f64", s)),
                }
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*; // Importe tout de la portée parente dans la portée du module de test

    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::util::Value::Value;

    #[test]
    fn test_new_valid_value() {
        define_struct_f64!(TestStruct, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
        let result = TestStruct::new(5.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 5.0);
    }

    #[test]
    fn test_new_invalid_value() {
        define_struct_f64!(TestStruct, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
        let result = TestStruct::new(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_value() {
        define_struct_f64!(TestStruct, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
        let mut test_struct = TestStruct::new(5.0).unwrap();
        let result = test_struct.set_value(10.0);
        assert!(result.is_ok());
        assert_eq!(test_struct.value(), 10.0);
    }

    #[test]
    fn test_set_invalid_value() {
        define_struct_f64!(TestStruct, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
        let mut test_struct = TestStruct::new(5.0).unwrap();
        let result = test_struct.set_value(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string_rounded() {
        define_struct_f64!(TestStruct, |value| {}, {0.0});
        let test_struct = TestStruct::new(5.678).unwrap();
        assert_eq!(test_struct.to_string_rounded(2), "5.68");
    }

    #[test]
    fn test_to_string_truncated() {
        define_struct_f64!(TestStruct, |value| {}, {0.0});
        let test_struct = TestStruct::new(5.678).unwrap();
        assert_eq!(test_struct.to_string_truncated(2), "5.67");
    }

    #[test]
    fn test_provide_from_input_dict() {
        use serde_json::Value;
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::from("5.0"));

        define_struct_f64!(TestStruct, |value| {}, {0.0});
        let result = TestStruct::provide_from_input_dict(&map, "key");
        assert!(result.is_some());
        assert_eq!(result.unwrap().value(), 5.0);
    }

}