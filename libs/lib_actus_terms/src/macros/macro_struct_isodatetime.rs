#[macro_export]
macro_rules! define_struct_isodatetime {
    ($struct_name:ident) => {
        use chrono::NaiveDateTime;
        use chrono::ParseResult;
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
        use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

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

            pub fn add_period(&self, period: IsoPeriod) -> Self {
                $struct_name(self.0.add(period))
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
        
        impl<'a, P> Add<&'a P> for &'a $struct_name
        where
            P: AsRef<IsoPeriod> + 'a,
        {
            type Output = $struct_name;
            fn add(self, other: &'a P) -> Self::Output {
                self.add_period(other.as_ref().clone())
            }
        }

        impl<'a, P> Sub<&'a P> for &'a $struct_name
        where
            P: AsRef<IsoPeriod> + 'a,
        {
            type Output = $struct_name;
            fn sub(self, other: &'a P) -> Self::Output {
                self.add_period(other.as_ref().clone())
            }
        }

    };
}