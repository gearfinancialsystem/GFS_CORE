#[derive(PartialEq, Debug, Clone)]
pub struct DelinquencyRate(f64);
impl DelinquencyRate {
    // Constructeur avec validation utilisant Result
    pub fn new(value: f64) -> Result<Self, String> {
        if !value.is_finite() {
            Err("DelinquencyRate value must be finite and not NaN".to_string())
        } else if value < 0.0 || value > f64::MAX {
            Err("DelinquencyRate value must be between 0.0 and a finite number".to_string())
        } else {
            Ok(DelinquencyRate(value))
        }
    }

    // Accesseur (Getter)
    pub fn value(&self) -> f64 {
        self.0
    }

    // Mutateur (Setter) avec validation utilisant Result
    pub fn set_value(&mut self, value: f64) -> Result<(), String> {
        if !value.is_finite() {
            Err("DelinquencyRate value must be finite and not NaN".to_string())
        } else if value < 0.0 || value > f64::MAX {
            Err("DelinquencyRate value must be between 0.0 and a finite number".to_string())
        } else {
            self.0 = value;
            Ok(())
        }
    }
}