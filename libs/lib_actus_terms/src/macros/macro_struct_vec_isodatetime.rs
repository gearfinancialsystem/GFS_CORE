#[macro_export]
macro_rules! define_struct_vec_isodatetime {
    ($struct_name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $struct_name(Vec<IsoDatetime>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<IsoDatetime>) -> Self {
                $struct_name(values)
            }

            pub fn values(&self) -> &Vec<IsoDatetime> {
                &self.0
            }

            pub fn add_value(&mut self, value: IsoDatetime) {
                self.0.push(value);
            }

            pub fn set_values(&mut self, values: Vec<IsoDatetime>) {
                self.0 = values;
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &IsoDatetime) -> bool {
                self.0.contains(value)
            }

            pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
                string_map.get(key).and_then(|s| {
                    if let Some(values) = s.as_vec() {
                        let parsed_cycles: Vec<IsoDatetime> = values
                            .iter()
                            .filter_map(|v| v.as_string().and_then(|s| IsoDatetime::from_str(&s).ok()))
                            .collect();
        
                        if !parsed_cycles.is_empty() {
                            Some($struct_name(parsed_cycles))
                        } else {
                            None
                        }
                    } else {
                        None // Not a vector type
                    }
                })
            }
        }
   
        impl Add<IsoPeriod> for $struct_name {
            type Output = Self;
            fn add(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.into_iter().map(|date| date.add(other.clone())).collect())
            }
        }

        impl Sub<IsoPeriod> for $struct_name {
            type Output = Self;
            fn sub(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.into_iter().map(|date| date.sub(other.clone())).collect())
            }
        }
   
   
    };
}