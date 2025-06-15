use std::rc::Rc;

use crate::types::isoDatetime::IsoDatetime;
use chrono::Datelike;
use crate::traits::TraitCountConvention::TraitDayCountConvention;


use crate::types::isoDatetime::traitNaiveDateTimeExtension;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct E30360ISDA {
    pub maturity_date: Rc<IsoDatetime>,
}

impl E30360ISDA {
    pub fn new(maturity_date: Rc<IsoDatetime>) -> Self {
        E30360ISDA {maturity_date}
    }
}

impl TraitDayCountConvention for E30360ISDA {
    // fn new() -> Self {
    //     Self { maturity_date: None }
    // }

    // /// Fixe la date de maturité (équivalent de `public void maturityDate(LocalDateTime ...)`)
    // fn set_maturity_date(&mut self, maturity_date: NaiveDateTime) {
    //     self.maturity_date = Some(maturity_date);
    // }

    // /// Vérifie si `date` est le dernier jour du mois
    // fn is_last_day_of_month(date: NaiveDateTime) -> bool {
    //     let day = date.day();
    //     // On tente de construire "date" + 1 jour, et on vérifie si le mois a changé
    //     let next_day = date + Duration::days(1);
    //     next_day.month() != date.month()
    //         // ou bien : next_day.day() == 1 => c'est un indicateur que date était le dernier jour
    //         // (mais on préfère la comparaison de mois)
    // }

    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        // d1
        let mut d1 = start_time.day();
        if start_time.is_last_day_of_month() {
            d1 = 30;
        }

        // d2
        let mut d2 = end_time.day();
        // Vérification du cas : si end_time == maturity_date et c'est un mois de février => pas d'ajustement
        let is_february = end_time.month() == 2;
        if let maturity = *self.maturity_date {
            // Vérifier end_time == maturityDate ET mois = 2 => on n'ajuste pas d2
            if end_time == maturity && is_february {
                // pas d'ajustement, on laisse d2
            } else if end_time.is_last_day_of_month() {
                d2 = 30;
            }
        } else {
            // Pas de maturité => la règle "dernier jour du mois => d2 = 30"
            if end_time.is_last_day_of_month() {
                d2 = 30;
            }
        }

        let del_d = (d2 as f64) - (d1 as f64);
        let del_m = (end_time.month() as i32) - (start_time.month() as i32);
        let del_y = end_time.year() - start_time.year();

        // Formule standard 30E/360
        360.0 * (del_y as f64) + 30.0 * (del_m as f64) + del_d
    }

    /// Calcule la fraction d'année (dayCount / 360.0)
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        self.day_count(start_time, end_time) / 360.0
    }
}

