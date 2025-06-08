

#[derive(Debug, Eq, PartialEq)]
pub struct STK;
impl STK {
    pub fn new() -> Self {
        return STK;
    }
    pub fn type_str(&self) -> String {
        return "STK contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for STK {
    fn get_option_rank(&self) -> &str {
        "8"
    }
    fn get_identifier(&self) -> &str {
        "stock"
    }
    fn get_name(&self) -> &str {
        "Stock"
    }
    fn get_acronym(&self) -> &str {
        "STK"
    }
    fn get_description(&self) -> &str {
        "Represents stocks/shares/equity."
    }
}    