#[macro_export]
macro_rules! define_struct_vec_isocycle {
    ($struct_name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $struct_name(Vec<IsoCycle>);

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name(Vec::new())
            }

            pub fn with_values(values: Vec<IsoCycle>) -> Self {
                $struct_name(values)
            }

            pub fn values(&self) -> &Vec<IsoCycle> {
                &self.0
            }

            pub fn add_value(&mut self, value: IsoCycle) {
                self.0.push(value);
            }

            pub fn set_values(&mut self, values: Vec<IsoCycle>) {
                self.0 = values;
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn contains(&self, value: &IsoCycle) -> bool {
                self.0.contains(value)
            }

            pub fn parse_from_string(s: &str) -> Result<Vec<IsoCycle>, String> {
                s.split(',')
                    .map(|cycle_str| cycle_str.trim())
                    .map(|cycle_str| IsoCycle::from_str(cycle_str))
                    .collect()
            }

            pub fn filter_by_period(&self) -> Vec<IsoCycle> {
                self.0.iter().filter(|cycle| matches!(cycle, IsoCycle::PeriodCycleAdjuster(_))).cloned().collect()
            }

            pub fn filter_by_weekday(&self) -> Vec<IsoCycle> {
                self.0.iter().filter(|cycle| matches!(cycle, IsoCycle::WeekdayCycleAdjuster(_))).cloned().collect()
            }
        }
    };
}