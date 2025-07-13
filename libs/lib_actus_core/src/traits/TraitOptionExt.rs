
pub trait TraitOptionExt<T> {
    fn itself_or(&self, value: f64) -> T;
    fn itself_or_option(&self, value: f64) -> Option<T>;
}