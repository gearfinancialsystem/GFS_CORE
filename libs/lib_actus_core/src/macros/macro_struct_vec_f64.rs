#[macro_export]
macro_rules! define_struct_vec_f64 {
    // Case with validation conditions and a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {$default_value:expr}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(Vec<f64>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<f64>) -> Result<Self, String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                    $(else if !($condition) {
                        return Err($error.to_string());
                    })+
                }
                Ok($struct_name(values))
            }

            pub fn values(&self) -> &Vec<f64> {
                &self.0
            }

            pub fn add_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(else if !($condition) {
                    return Err($error.to_string());
                })+
                else {
                    self.0.push($value);
                    Ok(())
                }
            }

            pub fn set_values(&mut self, values: Vec<f64>) -> Result<(), String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                    $(else if !($condition) {
                        return Err($error.to_string());
                    })+
                }
                self.0 = values;
                Ok(())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &f64) -> bool {
                self.0.contains(value)
            }

            pub fn parse_from_string(s: &str) -> Result<Self, String> {
                let values: Result<Vec<f64>, String> = s.split(',')
                    .map(|val_str| val_str.trim())
                    .map(|val_str| {
                        match val_str.parse::<f64>() {
                            Ok($value) => {
                                if !$value.is_finite() {
                                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                                } else if $value > f64::MAX {
                                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                                }
                                $(else if !($condition) {
                                    Err($error.to_string())
                                })+
                                else {
                                    Ok($value)
                                }
                            },
                            Err(_) => Err(format!("Unable to parse {} as f64", val_str)),
                        }
                    })
                    .collect();
                match values {
                    Ok(v) => Ok($struct_name(v)),
                    Err(e) => Err(e),
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let rounded = (value * factor).round() / factor;
                    format!("{:.1$}", rounded, decimals)
                }).collect()
            }

            pub fn to_string_truncated(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let truncated = (value * factor).trunc() / factor;
                    format!("{:.1$}", truncated, decimals)
                }).collect()
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name(vec![$default_value])
            }
        }

        impl FromStr for $struct_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $struct_name::parse_from_string(s)
            }
        }
    };
    // Case with validation conditions but without a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(Vec<f64>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<f64>) -> Result<Self, String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                    $(else if !($condition) {
                        return Err($error.to_string());
                    })+
                }
                Ok($struct_name(values))
            }

            pub fn values(&self) -> &Vec<f64> {
                &self.0
            }

            pub fn add_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(else if !($condition) {
                    return Err($error.to_string());
                })+
                else {
                    self.0.push($value);
                    Ok(())
                }
            }

            pub fn set_values(&mut self, values: Vec<f64>) -> Result<(), String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                    $(else if !($condition) {
                        return Err($error.to_string());
                    })+
                }
                self.0 = values;
                Ok(())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &f64) -> bool {
                self.0.contains(value)
            }

            pub fn parse_from_string(s: &str) -> Result<Self, String> {
                let values: Result<Vec<f64>, String> = s.split(',')
                    .map(|val_str| val_str.trim())
                    .map(|val_str| {
                        match val_str.parse::<f64>() {
                            Ok($value) => {
                                if !$value.is_finite() {
                                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                                } else if $value > f64::MAX {
                                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                                }
                                $(else if !($condition) {
                                    Err($error.to_string())
                                })+
                                else {
                                    Ok($value)
                                }
                            },
                            Err(_) => Err(format!("Unable to parse {} as f64", val_str)),
                        }
                    })
                    .collect();
                match values {
                    Ok(v) => Ok($struct_name(v)),
                    Err(e) => Err(e),
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let rounded = (value * factor).round() / factor;
                    format!("{:.1$}", rounded, decimals)
                }).collect()
            }

            pub fn to_string_truncated(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let truncated = (value * factor).trunc() / factor;
                    format!("{:.1$}", truncated, decimals)
                }).collect()
            }
        }

        impl FromStr for $struct_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $struct_name::parse_from_string(s)
            }
        }
    };
    // Case without validation conditions but with a default value
    ($struct_name:ident, |$value:ident| {}, {$default_value:expr}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(Vec<f64>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<f64>) -> Result<Self, String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                }
                Ok($struct_name(values))
            }

            pub fn values(&self) -> &Vec<f64> {
                &self.0
            }

            pub fn add_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    self.0.push($value);
                    Ok(())
                }
            }

            pub fn set_values(&mut self, values: Vec<f64>) -> Result<(), String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                }
                self.0 = values;
                Ok(())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &f64) -> bool {
                self.0.contains(value)
            }

            pub fn parse_from_string(s: &str) -> Result<Self, String> {
                let values: Result<Vec<f64>, String> = s.split(',')
                    .map(|val_str| val_str.trim())
                    .map(|val_str| {
                        match val_str.parse::<f64>() {
                            Ok($value) => {
                                if !$value.is_finite() {
                                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                                } else if $value > f64::MAX {
                                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                                } else {
                                    Ok($value)
                                }
                            },
                            Err(_) => Err(format!("Unable to parse {} as f64", val_str)),
                        }
                    })
                    .collect();
                match values {
                    Ok(v) => Ok($struct_name(v)),
                    Err(e) => Err(e),
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let rounded = (value * factor).round() / factor;
                    format!("{:.1$}", rounded, decimals)
                }).collect()
            }

            pub fn to_string_truncated(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let truncated = (value * factor).trunc() / factor;
                    format!("{:.1$}", truncated, decimals)
                }).collect()
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name(vec![$default_value])
            }
        }

        impl FromStr for $struct_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $struct_name::parse_from_string(s)
            }
        }
    };
    // Case without validation conditions and without a default value
    ($struct_name:ident, |$value:ident| {}, {}) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(Vec<f64>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<f64>) -> Result<Self, String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                }
                Ok($struct_name(values))
            }

            pub fn values(&self) -> &Vec<f64> {
                &self.0
            }

            pub fn add_value(&mut self, $value: f64) -> Result<(), String> {
                if !$value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                } else {
                    self.0.push($value);
                    Ok(())
                }
            }

            pub fn set_values(&mut self, values: Vec<f64>) -> Result<(), String> {
                for $value in &values {
                    if !$value.is_finite() {
                        return Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string());
                    } else if *$value > f64::MAX {
                        return Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string());
                    }
                }
                self.0 = values;
                Ok(())
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &f64) -> bool {
                self.0.contains(value)
            }

            pub fn parse_from_string(s: &str) -> Result<Self, String> {
                let values: Result<Vec<f64>, String> = s.split(',')
                    .map(|val_str| val_str.trim())
                    .map(|val_str| {
                        match val_str.parse::<f64>() {
                            Ok($value) => {
                                if !$value.is_finite() {
                                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                                } else if $value > f64::MAX {
                                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                                } else {
                                    Ok($value)
                                }
                            },
                            Err(_) => Err(format!("Unable to parse {} as f64", val_str)),
                        }
                    })
                    .collect();
                match values {
                    Ok(v) => Ok($struct_name(v)),
                    Err(e) => Err(e),
                }
            }

            pub fn to_string_rounded(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let rounded = (value * factor).round() / factor;
                    format!("{:.1$}", rounded, decimals)
                }).collect()
            }

            pub fn to_string_truncated(&self, decimals: usize) -> Vec<String> {
                let factor = 10f64.powi(decimals as i32);
                self.0.iter().map(|&value| {
                    let truncated = (value * factor).trunc() / factor;
                    format!("{:.1$}", truncated, decimals)
                }).collect()
            }
        }

        impl FromStr for $struct_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $struct_name::parse_from_string(s)
            }
        }
    };
}