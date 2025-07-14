
pub trait TraitOptionExt<T> {
    fn itself_or(&self, value: f64) -> T;
    fn itself_or_option(&self, value: f64) -> Option<T>;
    fn add_assign(&mut self, other: f64);
    fn sub_assign(&mut self, other: f64);
}