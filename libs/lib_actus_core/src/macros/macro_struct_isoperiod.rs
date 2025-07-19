#[macro_export]
macro_rules! define_struct_isoperiod {
    ($struct_name:ident) => {
        use crate::types::IsoPeriod::IsoPeriod;
        use crate::types::IsoDatetime::IsoDatetime; // Import manquant
        use crate::util::Value::Value;
        use std::collections::HashMap;
        use std::str::FromStr;
        use std::ops::Deref;
        use std::convert::AsRef;
        use std::borrow::Borrow;
        use std::ops::{Add, Sub};

        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(IsoPeriod);

        impl $struct_name {
            pub fn new(years: i32, months: i32, days: i32) -> Self {
                $struct_name(IsoPeriod::new(years, months, days))
            }

            pub fn set_value(&mut self, value: IsoPeriod) {
                self.0 = value;
            }

            pub fn parse_from_string(s: &str) -> Option<IsoPeriod> {
                IsoPeriod::parsex(s)
            }
            
            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None,
                    Some(s) => {
                        match Self::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value),
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }

        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match IsoPeriod::parsex(s) {
                    Some(p) => Ok($struct_name(p)),
                    None => Err(format!("Unable to parse IsoDuration"))
                }
            }
        }

        impl From<$struct_name> for IsoPeriod {
            fn from(val: $struct_name) -> IsoPeriod {
                val.0
            }
        }
        
        impl Deref for $struct_name {
            type Target = IsoPeriod;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<IsoPeriod> for $struct_name {
            fn as_ref(&self) -> &IsoPeriod {
                &self.0
            }
        }

        impl Borrow<IsoPeriod> for $struct_name {
            fn borrow(&self) -> &IsoPeriod {
                &self.0
            }
        }
        
        
        impl<'a, DT> Add<&'a DT> for &'a $struct_name
        where
            DT: AsRef<IsoDatetime> + 'a,
        {
            type Output = IsoDatetime;
            fn add(self, other: &'a DT) -> Self::Output {
                // Accès via le trait AsRef
                other.as_ref().add(self.0.clone())
            }
        }
    };
}