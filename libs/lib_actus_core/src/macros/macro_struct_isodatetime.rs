#[macro_export]
macro_rules! define_struct_isodatetime {
    ($struct_name:ident) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $struct_name(IsoDatetime);

        impl $struct_name {
            pub fn new(value: NaiveDateTime) -> Self {
                $struct_name(value)
            }

            pub fn value(&self) -> IsoDatetime {
                self.0
            }

            pub fn set_value(&mut self, value: IsoDatetime) {
                self.0 = value;
            }
            pub fn parse_from_string(s: &str, fmt: &str) -> ParseResult<IsoDatetime> {
                NaiveDateTime::parse_from_str(s, fmt)
            }

            // pub fn is_last_day_of_month(&self) -> bool {
            //     self.0.is_last_day_of_month()
            // }

            // pub fn last_date_of_month(&self) -> Self {
            //     $struct_name(self.0.last_date_of_month())
            // }

        }

        impl std::ops::Add<IsoPeriod> for $struct_name {
            type Output = Self;

            fn add(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.add(other))
            }
        }

        impl std::ops::Sub<IsoPeriod> for $struct_name {
            type Output = Self;

            fn sub(self, other: IsoPeriod) -> Self {
                $struct_name(self.0.sub(other))
            }
        }
    };
}
