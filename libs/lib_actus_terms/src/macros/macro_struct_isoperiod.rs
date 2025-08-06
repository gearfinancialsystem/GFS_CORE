
#[macro_export]
macro_rules! define_phantom_imports_isoperiod {
    (PhantomIsoPeriodW) => {

    };
    ($struct_name:ident) => {
        use crate::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
    };
}

#[macro_export]
macro_rules! define_to_phantom_type_isoperiod {
    (PhantomIsoPeriodW) => {
        fn to_phantom_type(&self) -> Self {
            self.clone()
        }
    };
    ($struct_name:ident) => {
        // Implémentation par défaut pour les autres structures
        fn to_phantom_type(&self) -> PhantomIsoPeriodW {
            PhantomIsoPeriodW::new(self.years, self.months, self.days)
        }
    };
}

#[macro_export]
macro_rules! define_struct_isoperiod {
    ($struct_name:ident) => {

        use lib_actus_types::types::IsoPeriod::IsoPeriod;
        use lib_actus_types::types::IsoDatetime::IsoDatetime;
        use lib_actus_types::types::Value::Value;
        use crate::traits::types_markers::TraitMarkerIsoPeriod::TraitMarkerIsoPeriod;
        use std::collections::HashMap;
        use std::fmt;
        use std::str::FromStr;
        use std::ops::Deref;
        use std::convert::AsRef;
        use std::borrow::Borrow;
        use std::ops::{Add, Sub};

        define_phantom_imports_isoperiod!($struct_name);

        #[derive(PartialEq, Debug, Clone, Copy, Hash)]
        pub struct $struct_name(IsoPeriod);

        impl $struct_name {
            pub fn new(years: i32, months: i32, days: i32) -> Self {
                $struct_name(IsoPeriod::new(years, months, days))
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


        impl TraitMarkerIsoPeriod for $struct_name {
            fn value(&self) -> IsoPeriod {
                self.0
            }

            fn set_value(&mut self, value: &IsoPeriod) {
                self.0 = *value;
            }

            fn parse_from_string(s: &str) -> Result<IsoPeriod, String> {
                IsoPeriod::parsex(s).ok_or_else(|| "parsing err".to_string())
            }

            define_to_phantom_type_isoperiod!($struct_name);
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
        
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

    };
}