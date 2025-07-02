#[macro_export]
macro_rules! define_struct_isodatetime {
    ($struct_name:ident) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(IsoDatetime);

        impl $struct_name {
            pub fn new(value: IsoDatetime) -> Result<Self, String> {
                Ok($struct_name(value))
            }

            pub fn value(&self) -> IsoDatetime {
                self.0
            }

            pub fn set_value(&mut self, value: IsoDatetime) {
                self.0 = value;
            }
            pub fn parse_from_string(s: &str, fmt: &str) -> ParseResult<IsoDatetime> {
                NaiveDateTime::parse_from_str(s, fmt)
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
                match Self::parse_from_string(s, "%Y-%m-%dT%H:%M:%S")  {
                    Ok(value) => $struct_name::new(value),
                    Err(_) => Err(format!("Unable to parse {} as isodatetime", s)),
                }
            }
        }

        impl std::ops::Add<IsoPeriod> for $struct_name {
            type Output = Self;

            fn add(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.add(other))
            }
        }

        impl std::ops::Sub<IsoPeriod> for $struct_name {
            type Output = Self;

            fn sub(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.sub(other))
            }
        }
    };
}
