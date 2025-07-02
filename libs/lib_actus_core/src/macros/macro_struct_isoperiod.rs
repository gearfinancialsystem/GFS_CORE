#[macro_export]
macro_rules! define_struct_isoperiod {
    ($struct_name:ident) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(IsoPeriod);

        impl $struct_name {
            // Create a new instance of the struct with an IsoPeriod
            pub fn new(years: i32, months: i32, days: i32) -> Self {
                $struct_name(IsoPeriod::new(years, months, days))
            }

            // Get the IsoPeriod value
            pub fn value(&self) -> &IsoPeriod {
                &self.0
            }

            // Set the IsoPeriod value
            pub fn set_value(&mut self, value: IsoPeriod) {
                self.0 = value;
            }

            // Parse an IsoPeriod from a string
            pub fn parse_from_string(s: &str) -> Option<IsoPeriod> {
                IsoPeriod::parsex(s)
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
        // Implement Add trait for IsoPeriod
        impl std::ops::Add<IsoPeriod> for $struct_name {
            type Output = Self;
            fn add(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.plus(&other))
            }
        }

        // Implement Sub trait for IsoPeriod
        impl std::ops::Sub<IsoPeriod> for $struct_name {
            type Output = Self;
            fn sub(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.minus(&other))
            }
        }
    };
}
