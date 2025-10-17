

#[macro_export]
macro_rules! define_option_ext {
    ($struct_name:ident) => {

        impl TraitOptionExt<$struct_name> for Option<$struct_name> {

            fn itself_or(&self, value: f64) -> $struct_name {
                match self {
                    Some(x) => x.clone(),
                    None => $struct_name::new(value).expect("Failed to create new instance"),
                }
            }

            fn itself_or_option(&self, value: f64) -> Option<$struct_name> {
                match self {
                    Some(x) => Some(x.clone()).clone(),
                    None => $struct_name::new(value).ok().clone(),
                }
            }

            // Implémentation de add_assign pour Option<$struct_name>
            fn add_assign(&mut self, other: f64) {
                if let &mut Some(ref mut some) = self {
                    some.0 += other;
                }
            }

            // Implémentation de sub_assign pour Option<$struct_name>
            fn sub_assign(&mut self, other: f64) {
                if let &mut Some(ref mut some) = self {
                    some.0 -= other;
                }
            }

        }


    }
}


#[macro_export]
macro_rules! define_struct_f64 {

    // Case with validation conditions and a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {$default_value:expr}) => {
        use std::str::FromStr;
        use std::collections::HashMap;
        use gfs_lib_types::types::Value::Value;
        use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
        use std::fmt;
        use crate::traits::TraitOptionExt::TraitOptionExt;
        use crate::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
        use crate::define_option_ext;
        //define_phantom_imports_f64!($struct_name);

        #[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
        pub struct $struct_name(f64);

        impl TraitMarkerF64 for $struct_name {
            fn value(&self) -> f64 {
                self.0
            }
            
            //define_to_phantom_type_f64!($struct_name);
        }

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if $value.is_finite() && !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    Ok($struct_name($value))
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

            pub fn signum(&self) -> f64 {
                self.0.signum()
            }
        }

        // Implémentation des opérations arithmétiques avec des types génériques
        impl<OtherType: TraitMarkerF64> Add<OtherType> for $struct_name {
            type Output = $struct_name;

            fn add(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 + other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> AddAssign<OtherType> for $struct_name {
            fn add_assign(&mut self, other: OtherType) {
                self.0 += other.value();
            }
        }

        // impl AddAssign<f64> for $struct_name {
        //     fn add_assign(&mut self, other: f64) {
        //         self.0 += other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Sub<OtherType> for $struct_name {
            type Output = $struct_name;

            fn sub(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 - other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> SubAssign<OtherType> for $struct_name {
            fn sub_assign(&mut self, other: OtherType) {
                self.0 -= other.value();
            }
        }

        // impl SubAssign<f64> for $struct_name {
        //     fn sub_assign(&mut self, other: f64) {
        //         self.0 -= other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Mul<OtherType> for $struct_name {
            type Output = $struct_name;

            fn mul(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 * other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> Div<OtherType> for $struct_name {
            type Output = $struct_name;

            fn div(self, other: OtherType) -> Self::Output {
                if other.value() == 0.0 {
                    panic!("Division by zero!");
                }
                $struct_name(self.0 / other.value())
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

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:.10}", self.0)
            }
        }

        // Implémentation du trait pour Option<$struct_name>
        define_option_ext!($struct_name);

    };

    // Case with validation conditions but without a default value
    ($struct_name:ident, |$value:ident| {$($condition:expr => $error:expr),+ $(,)?}, {}) => {
        use std::fmt;
        use std::str::FromStr;
        use std::collections::HashMap;
        use gfs_lib_types::types::Value::Value;
        use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
        use crate::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
        use crate::traits::TraitOptionExt::TraitOptionExt;
        use crate::define_option_ext;

        #[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
        pub struct $struct_name(f64);

        impl TraitMarkerF64 for $struct_name {
            fn value(&self) -> f64 {
                self.0
            }

            //define_to_phantom_type_f64!($struct_name);
        }

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if $value.is_finite() && !($condition) {
                        Err($error.to_string())
                    }
                )+
                else {
                    Ok($struct_name($value))
                }
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                $(
                    else if $value.is_finite() && !($condition) {
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
                    None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }

            pub fn signum(&self) -> f64 {
                self.0.signum()
            }
        }

        // Implémentation des opérations arithmétiques avec des types génériques
        impl<OtherType: TraitMarkerF64> Add<OtherType> for $struct_name {
            type Output = $struct_name;

            fn add(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 + other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> AddAssign<OtherType> for $struct_name {
            fn add_assign(&mut self, other: OtherType) {
                self.0 += other.value();
            }
        }

        // impl AddAssign<f64> for $struct_name {
        //     fn add_assign(&mut self, other: f64) {
        //         self.0 += other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Sub<OtherType> for $struct_name {
            type Output = $struct_name;

            fn sub(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 - other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> SubAssign<OtherType> for $struct_name {
            fn sub_assign(&mut self, other: OtherType) {
                self.0 -= other.value();
            }
        }

        // impl SubAssign<f64> for $struct_name {
        //     fn sub_assign(&mut self, other: f64) {
        //         self.0 -= other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Mul<OtherType> for $struct_name {
            type Output = $struct_name;

            fn mul(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 * other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> Div<OtherType> for $struct_name {
            type Output = $struct_name;

            fn div(self, other: OtherType) -> Self::Output {
                if other.value() == 0.0 {
                    panic!("Division by zero!");
                }
                $struct_name(self.0 / other.value())
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

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:.10}", self.0)
            }
        }
        // Implémentation du trait pour Option<$struct_name>
        define_option_ext!($struct_name);

    };

    // Case without validation conditions but with a default value
    ($struct_name:ident, |$value:ident| {}, {$default_value:expr}) => {
        use std::fmt;
        use std::str::FromStr;
        use std::collections::HashMap;
        use gfs_lib_types::types::Value::Value;
        use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
        use crate::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
        use crate::traits::TraitOptionExt::TraitOptionExt;
        use crate::define_option_ext;

        #[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
        pub struct $struct_name(f64);


        impl TraitMarkerF64 for $struct_name {
            fn value(&self) -> f64 {
                self.0
            }

            //define_to_phantom_type_f64!($struct_name);
        }

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                else {
                    Ok($struct_name($value))
                }
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
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

            pub fn signum(&self) -> f64 {
                self.0.signum()
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name($default_value)
            }
        }

        // Implémentation des opérations arithmétiques avec des types génériques
        impl<OtherType: TraitMarkerF64> Add<OtherType> for $struct_name {
            type Output = $struct_name;

            fn add(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 + other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> AddAssign<OtherType> for $struct_name {
            fn add_assign(&mut self, other: OtherType) {
                self.0 += other.value();
            }
        }

        // impl AddAssign<f64> for $struct_name {
        //     fn add_assign(&mut self, other: f64) {
        //         self.0 += other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Sub<OtherType> for $struct_name {
            type Output = $struct_name;

            fn sub(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 - other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> SubAssign<OtherType> for $struct_name {
            fn sub_assign(&mut self, other: OtherType) {
                self.0 -= other.value();
            }
        }

        // impl SubAssign<f64> for $struct_name {
        //     fn sub_assign(&mut self, other: f64) {
        //         self.0 -= other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Mul<OtherType> for $struct_name {
            type Output = $struct_name;

            fn mul(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 * other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> Div<OtherType> for $struct_name {
            type Output = $struct_name;

            fn div(self, other: OtherType) -> Self::Output {
                if other.value() == 0.0 {
                    panic!("Division by zero!");
                }
                $struct_name(self.0 / other.value())
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

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:.10}", self.0)
            }
        }

        // Implémentation du trait pour Option<$struct_name>
        define_option_ext!($struct_name);

    };

    // Case without validation conditions and without a default value
    ($struct_name:ident, |$value:ident| {}, {}) => {
        use std::fmt;
        use std::str::FromStr;
        use std::collections::HashMap;
        use gfs_lib_types::types::Value::Value;
        use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
        use crate::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
        use crate::traits::TraitOptionExt::TraitOptionExt;
        use crate::define_option_ext;

        #[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
        pub struct $struct_name(f64);


        impl TraitMarkerF64 for $struct_name {
            fn value(&self) -> f64 {
                self.0
            }

            //define_to_phantom_type_f64!($struct_name);
        }

        impl $struct_name {
            pub fn new($value: f64) -> Result<Self, String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
                else {
                    Ok($struct_name($value))
                }
            }

            pub fn set_value(&mut self, $value: f64) -> Result<(), String> {
                if $value.is_nan() {
                    Err(concat!(stringify!($struct_name), " value must not be NaN").to_string())
                } else if $value.is_finite() && $value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be less than or equal to f64::MAX").to_string())
                }
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
                    None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }

            pub fn signum(&self) -> f64 {
                self.0.signum()
            }
        }

        // Implémentation des opérations arithmétiques avec des types génériques
        impl<OtherType: TraitMarkerF64> Add<OtherType> for $struct_name {
            type Output = $struct_name;

            fn add(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 + other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> AddAssign<OtherType> for $struct_name {
            fn add_assign(&mut self, other: OtherType) {
                self.0 += other.value();
            }
        }

        // impl AddAssign<f64> for $struct_name {
        //     fn add_assign(&mut self, other: f64) {
        //         self.0 += other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Sub<OtherType> for $struct_name {
            type Output = $struct_name;

            fn sub(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 - other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> SubAssign<OtherType> for $struct_name {
            fn sub_assign(&mut self, other: OtherType) {
                self.0 -= other.value();
            }
        }

        // impl SubAssign<f64> for $struct_name {
        //     fn sub_assign(&mut self, other: f64) {
        //         self.0 -= other;
        //     }
        // }

        impl<OtherType: TraitMarkerF64> Mul<OtherType> for $struct_name {
            type Output = $struct_name;

            fn mul(self, other: OtherType) -> Self::Output {
                $struct_name(self.0 * other.value())
            }
        }

        impl<OtherType: TraitMarkerF64> Div<OtherType> for $struct_name {
            type Output = $struct_name;

            fn div(self, other: OtherType) -> Self::Output {
                if other.value() == 0.0 {
                    panic!("Division by zero!");
                }
                $struct_name(self.0 / other.value())
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

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:.10}", self.0)
            }
        }
        // Implémentation du trait pour Option<$struct_name>
        define_option_ext!($struct_name);


    };
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // TestStruct1 tests
//     #[test]
//     fn test_teststruct1_new() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let valid_value = TestStruct1::new(1.0);
//         assert!(valid_value.is_ok());
//         assert_eq!(valid_value.unwrap().value(), 1.0);
//
//         let invalid_value = TestStruct1::new(-1.0);
//         assert!(invalid_value.is_err());
//     }
//
//     #[test]
//     fn test_teststruct1_value() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let ts = TestStruct1::new(2.0).unwrap();
//         assert_eq!(ts.value(), 2.0);
//     }
//
//     #[test]
//     fn test_teststruct1_set_value() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let mut ts = TestStruct1::new(2.0).unwrap();
//         assert!(ts.set_value(3.0).is_ok());
//         assert_eq!(ts.value(), 3.0);
//
//         let result = ts.set_value(-1.0);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_teststruct1_to_string_rounded() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let ts = TestStruct1::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_rounded(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct1_to_string_truncated() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let ts = TestStruct1::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_truncated(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct1_provide_from_input_dict() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let mut map = HashMap::new();
//         map.insert("key".to_string(), Value::from_string("2.0".to_string()));
//         let ts = TestStruct1::provide_from_input_dict(&map, "key").unwrap();
//         assert_eq!(ts.value(), 2.0);
//
//         let mut map_empty = HashMap::new();
//         let ts_default = TestStruct1::provide_from_input_dict(&map_empty, "key").unwrap();
//         assert_eq!(ts_default.value(), 0.0);
//     }
//
//     #[test]
//     fn test_teststruct1_default() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let ts: TestStruct1 = Default::default();
//         assert_eq!(ts.value(), 0.0);
//     }
//
//     #[test]
//     fn test_teststruct1_from_str() {
//         define_struct_f64!(TestStruct1, |value| {value >= 0.0 => "Value must be non-negative"}, {0.0});
//         let valid_str = "1.0";
//         let parsed_ts = TestStruct1::from_str(valid_str);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), 1.0);
//
//         let invalid_str = "-1.0";
//         let parsed_ts = TestStruct1::from_str(invalid_str);
//         assert!(parsed_ts.is_err());
//     }
//
//     // TestStruct2 tests
//     #[test]
//     fn test_teststruct2_new() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let valid_value = TestStruct2::new(1.0);
//         assert!(valid_value.is_ok());
//         assert_eq!(valid_value.unwrap().value(), 1.0);
//
//         let invalid_value = TestStruct2::new(-1.0);
//         assert!(invalid_value.is_err());
//     }
//
//     #[test]
//     fn test_teststruct2_value() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let ts = TestStruct2::new(2.0).unwrap();
//         assert_eq!(ts.value(), 2.0);
//     }
//
//     #[test]
//     fn test_teststruct2_set_value() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let mut ts = TestStruct2::new(2.0).unwrap();
//         assert!(ts.set_value(3.0).is_ok());
//         assert_eq!(ts.value(), 3.0);
//
//         let result = ts.set_value(-1.0);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_teststruct2_to_string_rounded() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let ts = TestStruct2::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_rounded(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct2_to_string_truncated() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let ts = TestStruct2::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_truncated(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct2_provide_from_input_dict() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let mut map = HashMap::new();
//         map.insert("key".to_string(), Value::from_string("2.0".to_string()));
//         let ts = TestStruct2::provide_from_input_dict(&map, "key").unwrap();
//         assert_eq!(ts.value(), 2.0);
//
//         let mut map_empty = HashMap::new();
//         let ts_none = TestStruct2::provide_from_input_dict(&map_empty, "key");
//         assert!(ts_none.is_none());
//     }
//
//     #[test]
//     fn test_teststruct2_from_str() {
//         define_struct_f64!(TestStruct2, |value| {value >= 0.0 => "Value must be non-negative"}, {});
//         let valid_str = "1.0";
//         let parsed_ts = TestStruct2::from_str(valid_str);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), 1.0);
//
//         let invalid_str = "-1.0";
//         let parsed_ts = TestStruct2::from_str(invalid_str);
//         assert!(parsed_ts.is_err());
//     }
//
//     // TestStruct3 tests
//     #[test]
//     fn test_teststruct3_new() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let valid_value = TestStruct3::new(1.0);
//         assert!(valid_value.is_ok());
//         assert_eq!(valid_value.unwrap().value(), 1.0);
//
//         // TestStruct3 has no validation conditions, so even negative values should work
//         let valid_value_negative = TestStruct3::new(-1.0);
//         assert!(valid_value_negative.is_ok());
//         assert_eq!(valid_value_negative.unwrap().value(), -1.0);
//     }
//
//     #[test]
//     fn test_teststruct3_value() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let ts = TestStruct3::new(2.0).unwrap();
//         assert_eq!(ts.value(), 2.0);
//     }
//
//     #[test]
//     fn test_teststruct3_set_value() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let mut ts = TestStruct3::new(2.0).unwrap();
//         assert!(ts.set_value(3.0).is_ok());
//         assert_eq!(ts.value(), 3.0);
//
//         // TestStruct3 has no validation conditions, so even negative values should work
//         assert!(ts.set_value(-1.0).is_ok());
//         assert_eq!(ts.value(), -1.0);
//     }
//
//     #[test]
//     fn test_teststruct3_to_string_rounded() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let ts = TestStruct3::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_rounded(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct3_to_string_truncated() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let ts = TestStruct3::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_truncated(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct3_provide_from_input_dict() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let mut map = HashMap::new();
//         map.insert("key".to_string(), Value::from_string("2.0".to_string()));
//         let ts = TestStruct3::provide_from_input_dict(&map, "key").unwrap();
//         assert_eq!(ts.value(), 2.0);
//
//         let mut map_empty = HashMap::new();
//         let ts_default = TestStruct3::provide_from_input_dict(&map_empty, "key").unwrap();
//         assert_eq!(ts_default.value(), 0.0);
//     }
//
//     #[test]
//     fn test_teststruct3_default() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let ts: TestStruct3 = Default::default();
//         assert_eq!(ts.value(), 0.0);
//     }
//
//     #[test]
//     fn test_teststruct3_from_str() {
//         define_struct_f64!(TestStruct3, |value| {}, {0.0});
//         let valid_str = "1.0";
//         let parsed_ts = TestStruct3::from_str(valid_str);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), 1.0);
//
//         let valid_str_negative = "-1.0";
//         let parsed_ts = TestStruct3::from_str(valid_str_negative);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), -1.0);
//     }
//
//     // TestStruct4 tests
//     #[test]
//     fn test_teststruct4_new() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let valid_value = TestStruct4::new(1.0);
//         assert!(valid_value.is_ok());
//         assert_eq!(valid_value.unwrap().value(), 1.0);
//
//         // TestStruct4 has no validation conditions, so even negative values should work
//         let valid_value_negative = TestStruct4::new(-1.0);
//         assert!(valid_value_negative.is_ok());
//         assert_eq!(valid_value_negative.unwrap().value(), -1.0);
//     }
//
//     #[test]
//     fn test_teststruct4_value() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let ts = TestStruct4::new(2.0).unwrap();
//         assert_eq!(ts.value(), 2.0);
//     }
//
//     #[test]
//     fn test_teststruct4_set_value() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let mut ts = TestStruct4::new(2.0).unwrap();
//         assert!(ts.set_value(3.0).is_ok());
//         assert_eq!(ts.value(), 3.0);
//
//         // TestStruct4 has no validation conditions, so even negative values should work
//         assert!(ts.set_value(-1.0).is_ok());
//         assert_eq!(ts.value(), -1.0);
//     }
//
//     #[test]
//     fn test_teststruct4_to_string_rounded() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let ts = TestStruct4::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_rounded(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct4_to_string_truncated() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let ts = TestStruct4::new(2.1234).unwrap();
//         assert_eq!(ts.to_string_truncated(2), "2.12");
//     }
//
//     #[test]
//     fn test_teststruct4_provide_from_input_dict() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let mut map = HashMap::new();
//         map.insert("key".to_string(), Value::from_string("2.0".to_string()));
//         let ts = TestStruct4::provide_from_input_dict(&map, "key").unwrap();
//         assert_eq!(ts.value(), 2.0);
//
//         let mut map_empty = HashMap::new();
//         let ts_none = TestStruct4::provide_from_input_dict(&map_empty, "key");
//         assert!(ts_none.is_none());
//     }
//
//     #[test]
//     fn test_teststruct4_from_str() {
//         define_struct_f64!(TestStruct4, |value| {}, {});
//         let valid_str = "1.0";
//         let parsed_ts = TestStruct4::from_str(valid_str);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), 1.0);
//
//         let valid_str_negative = "-1.0";
//         let parsed_ts = TestStruct4::from_str(valid_str_negative);
//         assert!(parsed_ts.is_ok());
//         assert_eq!(parsed_ts.unwrap().value(), -1.0);
//     }
// }
