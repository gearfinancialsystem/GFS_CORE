use std::rc::Rc;

use crate::types::isoDatetime::IsoDatetime;
use crate::types::isoDatetime::traitNaiveDateTimeExtension;
use chrono::Datelike;
use crate::traits::TraitCountConvention::TraitDayCountConvention;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct E283666 {
    pub maturity_date: Rc<IsoDatetime>,
}

impl E283666 {
    pub fn new(maturity_date: Rc<IsoDatetime>) -> Self {
        E283666 {maturity_date}
    }
}
impl TraitDayCountConvention for E283666 {
    /// Construit un E283666 avec une maturité donnée.
    /// (Si vous voulez vraiment un "pas de maturité",
    /// il faut prévoir une date sentinelle ou un bool).
    // pub fn new(maturity_date: NaiveDateTime) -> Self {
    //     Self { maturity_date }
    // }

    // /// Modifie la maturité
    // pub fn set_maturity_date(&mut self, maturity_date: NaiveDateTime) {
    //     self.maturity_date = maturity_date;
    // }

    /// Calcule le nombre de jours, selon la convention 28/336
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        // Ajustement de d1
        let mut d1 = start_time.day();
        if start_time.is_last_day_of_month() {
            d1 = 28;
        }

        // Ajustement de d2
        let mut d2 = end_time.day();
        // On reprend la logique initiale:
        // if !(end_time == *maturity_date || end_time.month() == 2)
        //     && end_time.day() == end_time.with_day(0).unwrap().day() {
        //     d2 = 28;
        // } else if d2 > 28 {
        //     d2 = 28;
        // }
        //
        // En version adaptée:
        if !(end_time == *self.maturity_date || end_time.month() == 2)
            && end_time.is_last_day_of_month()
        {
            d2 = 28;
        } else if d2 > 28 {
            d2 = 28;
        }

        let del_d = d2 as f64 - d1 as f64;
        let del_m = end_time.month() as i32 - start_time.month() as i32;
        let del_y = end_time.year() - start_time.year();

        (336.0 * del_y as f64) + (28.0 * del_m as f64) + del_d
    }

    /// Calcule la fraction (days / 336.0)
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        self.day_count(start_time, end_time) / 336.0
    }
}

