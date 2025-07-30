

#[macro_export]
macro_rules! define_struct_isocycle {
    ($struct_name:ident) => {
        use lib_actus_types::types::Value::Value;
        use crate::traits::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
        use lib_actus_types::types::IsoCycle::IsoCycle;
        use std::str::FromStr;
        use std::collections::HashMap;
        use std::fmt;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $struct_name(IsoCycle);

        impl $struct_name {
            // Create a new instance of the struct with an IsoCycle
            pub fn new(cycle: String) -> Result<Self, String> {
                //IsoCycle::from_str(cycle.as_str()).map($struct_name)
                $struct_name::from_str(cycle.as_str())
            }
            pub fn new_with_isocycle(iso_cycle: IsoCycle) -> Self {
                $struct_name(iso_cycle)
            }


            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
                    Some(s) => {
                        match $struct_name::from_str(s.as_string().unwrap().as_str()) {
                            Ok(value) => Some(value), // Valeur valide
                            Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                        }
                    }
                }
            }
        }

        impl TraitMarkerIsoCycle for $struct_name {
            // Get the IsoCycle value
            fn value(&self) -> IsoCycle {
                self.0.clone()
            }

            // Set the IsoCycle value
            fn set_value(&mut self, value: &IsoCycle) {
                self.0 = value.clone();
            }
        }

        //Implémentation du trait From<IsoDatetime>
        impl From<IsoCycle> for $struct_name {
            fn from(iso_cycle: IsoCycle) -> Self {
                $struct_name(iso_cycle)
            }
        }
        
        // Implémentation du trait Hash
        impl std::hash::Hash for $struct_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl FromStr for $struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match IsoCycle::from_str(s) {
                    Ok(value) => Ok($struct_name(value)),
                    Err(_) => Err(format!("Unable to parse {} as IsoCycle", s)),
                }
            }
        }
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "($struct_name {})", self.0)
            }
        }

    };
}