
#[macro_export]
macro_rules! define_struct_f64 {
    ($struct_name:ident) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(f64);

        impl $struct_name {
            pub fn new(value: f64) -> Result<Self, String> {
                if !value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if value < 0.0 || value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be between 0.0 and a finite number").to_string())
                } else {
                    Ok($struct_name(value))
                }
            }

            pub fn value(&self) -> f64 {
                self.0
            }

            pub fn set_value(&mut self, value: f64) -> Result<(), String> {
                if !value.is_finite() {
                    Err(concat!(stringify!($struct_name), " value must be finite and not NaN").to_string())
                } else if value < 0.0 || value > f64::MAX {
                    Err(concat!(stringify!($struct_name), " value must be between 0.0 and a finite number").to_string())
                } else {
                    self.0 = value;
                    Ok(())
                }
            }
        }
    };
}
