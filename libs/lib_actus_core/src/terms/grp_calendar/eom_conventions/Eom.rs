use crate::types::isoDatetime::IsoDatetime;

use chrono::{Datelike, Duration, NaiveDate};
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct EOM;

impl EOM {
    pub fn new() -> Self {
        return EOM;
    }
    pub fn type_str(&self) -> String {
        return "EOM eom".to_string();
    }
}

impl TraitEndOfMonthConvention for EOM {
    fn shift(&self, datetime: &IsoDatetime) -> IsoDatetime {
        // NaiveDateTime creation with last day of month
        // Extraire l'année et le mois
        let year = datetime.year();
        let month = datetime.month();

        // Déterminer le premier jour du mois suivant
        let first_day_next_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
        };

        // Revenir d'un jour pour obtenir le dernier jour du mois actuel
        let last_day = first_day_next_month - Duration::days(1);

        // Créer un `NaiveDateTime` avec la même heure que l'entrée
        let shifted_datetime = last_day;

        shifted_datetime // .unwrap_or(last_day)
    }
}




