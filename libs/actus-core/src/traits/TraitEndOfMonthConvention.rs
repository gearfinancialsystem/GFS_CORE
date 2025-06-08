use chrono::NaiveDateTime;

pub trait TraitEndOfMonthConvention {
 /// Décale une date vers la fin du mois selon une convention spécifique.
 ///
 /// # Arguments
 ///
 /// * `date` - La date à décaler.
 ///
 /// # Retourne
 ///
 /// Une nouvelle date décalée.
 fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime;
}
 