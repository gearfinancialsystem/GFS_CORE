pub trait TraitEnumOptionDescription {
    /// Return elements as defined in https://github.com/actusfrf/actus-dictionary/blob/master/actus-dictionary-terms.json
    fn get_option_rank(&self) -> &str;
    fn get_identifier(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_acronym(&self) -> &str;
    fn get_description(&self) -> &str;
}