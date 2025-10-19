use crate::types::IsoCycle::IsoCycle;
use crate::types::IsoDatetime::IsoDatetime;

pub trait IsoDateTimeConvertTo<T> {
    fn convert<U>(self) -> U
    where
        Self: Into<IsoDatetime>,
        U: From<IsoDatetime>;
}

pub trait IsoDateTimeConvertToOption<T> {
    fn convert_option<U>(self) -> Option<U>
    where
        Self: Sized,
        U: From<IsoDatetime>;
}

pub trait IsoCycleConvertTo<T> {
    fn convert<U>(self) -> U
    where
        Self: Into<IsoCycle>,
        U: From<IsoCycle>;
}

pub trait IsoCycleConvertToOption<T> {
    fn convert_option<U>(self) -> Option<U>
    where
        Self: Sized,
        U: From<IsoCycle>;
}