
use chrono::NaiveDateTime;
use chrono::{Datelike, Duration, Timelike};
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn shift(&self, datetime: &NaiveDateTime) -> NaiveDateTime {
        // NaiveDateTime creation with last day of month
        // Extraire l'année et le mois
        let year = datetime.year();
        let month = datetime.month();

        // Déterminer le premier jour du mois suivant
        let first_day_next_month = if month == 12 {
            NaiveDateTime::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDateTime::from_ymd_opt(year, month + 1, 1).unwrap()
        };

        // Revenir d'un jour pour obtenir le dernier jour du mois actuel
        let last_day = first_day_next_month - Duration::days(1);

        // Créer un `NaiveDateTime` avec la même heure que l'entrée
        let shifted_datetime = last_day.and_hms_opt(
            datetime.hour(), 
            datetime.minute(), 
            datetime.second());

        shifted_datetime.unwrap_or(last_day)
    }
}




