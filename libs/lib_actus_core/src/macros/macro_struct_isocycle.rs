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

            // Parse an IsoCycle from a string
            pub fn parse_from_string(s: &str) -> Result<IsoCycle, String> {
                IsoCycle::from_str(s)
            }
        }
    };
}