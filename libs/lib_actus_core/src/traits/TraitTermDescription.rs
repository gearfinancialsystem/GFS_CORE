pub trait TraitTermDescription {
    /// Return elements as defined in https://github.com/actusfrf/actus-dictionary/blob/master/actus-dictionary-terms.json
    fn get_identifier(&self) -> &str;
    fn get_group(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_acronym(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_allowed_values(&self) -> &str;
    fn get_default_value(&self) -> &str;
    fn get_description(&self) -> &str;
}