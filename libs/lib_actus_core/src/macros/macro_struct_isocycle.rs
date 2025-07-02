#[macro_export]
macro_rules! define_struct_isocycle {
    ($struct_name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $struct_name(IsoCycle);

        impl $struct_name {
            // Create a new instance of the struct with an IsoCycle
            pub fn new(cycle: String) -> Result<Self, String> {
                IsoCycle::from_str(cycle.as_str()).map($struct_name)
            }

            // Get the IsoCycle value
            pub fn value(&self) -> &IsoCycle {
                &self.0
            }

            // Set the IsoCycle value
            pub fn set_value(&mut self, value: IsoCycle) {
                self.0 = value;
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