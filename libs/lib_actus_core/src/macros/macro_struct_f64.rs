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
