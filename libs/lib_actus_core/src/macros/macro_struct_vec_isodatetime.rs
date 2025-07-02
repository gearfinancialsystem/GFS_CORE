
#[macro_export]
macro_rules! define_struct_vec_isodatetime {
    ($struct_name:ident) => {
        #[derive(PartialEq, Debug, Clone)]
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

            pub fn sort(&mut self) {
                self.0.sort();
            }

            pub fn filter_by_month(&self, month: u32) -> Vec<IsoDatetime> {
                self.0.iter().filter(|&date| date.month() == month).cloned().collect()
            }

            pub fn filter_by_year(&self, year: i32) -> Vec<IsoDatetime> {
                self.0.iter().filter(|&date| date.year() == year).cloned().collect()
            }

            pub fn parse_from_string(s: &str, fmt: &str) -> ParseResult<Vec<IsoDatetime>> {
                s.split(',')
                    .map(|date_str| date_str.trim())
                    .map(|date_str| NaiveDateTime::parse_from_str(date_str, fmt))
                    .collect()
            }

            pub fn add_period(&mut self, period: IsoPeriod) {
                self.0 = self.0.iter().map(|&date| date.add(period)).collect();
            }

            pub fn sub_period(&mut self, period: IsoPeriod) {
                self.0 = self.0.iter().map(|&date| date.sub(period)).collect();
            }
        }

        impl Add<IsoPeriod> for $struct_name {
            type Output = Self;
            fn add(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.into_iter().map(|date| date.add(other)).collect())
            }
        }

        impl Sub<IsoPeriod> for $struct_name {
            type Output = Self;
            fn sub(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.into_iter().map(|date| date.sub(other)).collect())
            }
        }
    };
}
