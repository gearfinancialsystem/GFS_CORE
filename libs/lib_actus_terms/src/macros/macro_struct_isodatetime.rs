#[macro_export]
macro_rules! define_phantom_imports_isodatetime {
    (PhantomIsoDatetimeW) => {

    };
    ($struct_name:ident) => {
        use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
    };
}

#[macro_export]
macro_rules! define_to_phantom_type_isodatetime {
    (PhantomIsoDatetimeW) => {
        // Implémentation spécifique pour PhantomIsoDatetimeW
        fn to_phantom_type(&self) -> Self {
            *self
        }
    };
    ($struct_name:ident) => {
        // Implémentation par défaut pour les autres structures
        fn to_phantom_type(&self) -> PhantomIsoDatetimeW {
            PhantomIsoDatetimeW::new(self.value()).expect("Conversion to PhantomIsoDatetimeW doesn't work")
        }
    };
}

#[macro_export]
macro_rules! define_struct_isodatetime {
    ($struct_name:ident) => {
        use chrono::NaiveDateTime;
        use chrono::Weekday;
        use chrono::Datelike;
        use std::str::FromStr;
        use std::collections::HashMap;
        use std::fmt;
        use std::ops::Add;
        use std::ops::Sub;
        use std::hash::Hash;
        use std::hash::Hasher;

        use lib_actus_types::types::IsoDatetime::IsoDatetime;
        use lib_actus_types::types::IsoPeriod::IsoPeriod;
        use lib_actus_types::types::Value::Value;
        use crate::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
        define_phantom_imports_isodatetime!($struct_name);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $struct_name(IsoDatetime);

        impl TraitMarkerIsoDatetime for $struct_name {
            fn value(&self) -> IsoDatetime {
                self.0
            }

            fn set_value(&mut self, value: &IsoDatetime) {
                self.0 = IsoDatetime(value.0.clone());
            }

            fn parse_from_string(s: &str, fmt: &str) -> Result<IsoDatetime, String> {
                match NaiveDateTime::parse_from_str(s, fmt) {
                    Ok(dt) => Ok(IsoDatetime(dt)),
                    Err(e) => Err(format!("{}", e)),
                }
            }

            define_to_phantom_type_isodatetime!($struct_name);

        }

        impl $struct_name {
            pub fn new(value: IsoDatetime) -> Result<Self, String> {
                Ok($struct_name(value))
            }

            pub fn to_opt_isodatetime(option_s: &Option<$struct_name>) -> Option<IsoDatetime> {
                option_s.clone().map(|mons| mons.value())
            }


            pub fn numdays_between_dates(&self, dt2: &$struct_name) -> f64 {
                 (*self.0 - *dt2.0).num_days() as f64
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

            pub fn add_period<P: Into<IsoPeriod>>(&self, period: P) -> Self {
                $struct_name(self.0.add(period.into()))
            }

            pub fn sub_period<P: Into<IsoPeriod>>(&self, period: P) -> Self {
                $struct_name(self.0.sub(period.into()))
            }

            pub fn year(&self) -> i32 {
                self.0.0.year()
            }

            pub fn month(&self) -> u32 {
                self.0.0.month()
            }

            pub fn day(&self) -> u32 {
                self.0.0.day()
            }

            pub fn leap_year(&self) -> bool {
                self.0.0.date().leap_year()
            }

            pub fn weekday(&self) -> Weekday {
                self.0.0.date().weekday()
            }
            pub fn is_last_day_of_month(&self) -> bool {
                self.0.is_last_day_of_month()
            }
            pub fn timestamp_millis(&self) -> i64 {
                self.0.0.and_utc().timestamp_millis()
            }
        }

        impl From<IsoDatetime> for $struct_name {
            fn from(iso_datetime: IsoDatetime) -> Self {
                $struct_name(iso_datetime)
            }
        }

        impl Into<IsoDatetime> for $struct_name {
            fn into(self) -> IsoDatetime {
                self.0
            }
        }

        impl Hash for $struct_name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match Self::parse_from_string(s, "%Y-%m-%dT%H:%M:%S")  {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as isodatetime", s)),
                }
            }
        }

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }


    };
}