#[macro_export]
macro_rules! define_struct_string {
    ($struct_name:ident, "normale") => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(String);

        impl $struct_name {
            pub fn new(value: String) -> Result<Self, String> {
                Ok($struct_name(value))
            }

            pub fn value(&self) -> String {
                self.0.clone()
            }

            pub fn set_value(&mut self, value: String) {
                self.0 = value;
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None, // A VERIFIER // Clé absente : valeur par défaut dans un Some
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
                let value = s.to_string();
                $struct_name::new(value)
            }
        }
    };
    ($struct_name:ident, "currency") => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(String);

        impl $struct_name {
            pub fn new(value: String) -> Result<Self, String> {
                if CURRENCIES.contains(&value.as_str()) {
                    Ok($struct_name(value))
                } else {
                    Err(format!("La devise '{}' n'est pas valide.", value))
                }
            }

            pub fn value(&self) -> String {
                self.0.clone()
            }

            pub fn set_value(&mut self, value: String) {
                if let Err(e) = $struct_name::new(value.clone()) {
                    panic!("Erreur lors de la mise à jour de la valeur : {}", e);
                }
                self.0 = value;
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                match string_map.get(key) {
                    None => None, // A VERIFIER // Clé absente : valeur par défaut dans un Some
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
                let value = s.to_string();
                $struct_name::new(value)
            }
        }
    };
}