// Ce code Java définit une interface RiskFactorModelProvider qui représente un observateur externe des facteurs de risque dans le contexte de la finance. Cette interface fournit des méthodes pour récupérer des informations sur les facteurs de risque utilisés dans les modèles financiers, comme les taux d'intérêt, les indicateurs de défaut, et les valeurs de marché. Ces informations sont essentielles pour calculer les paiements d'instruments financiers dans un environnement dynamique.

use chrono::NaiveDateTime;
use crate::traits::ContractModelTrait::ContractModelTrait;
use crate::event::Event::Event;
use crate::states::StateSpace::StateSpace;
use std::collections::HashSet;

pub trait TraitRiskFactorModel {
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> HashSet<String>;

    /// Returns the set of event times for a particular risk factor
    ///
    /// The default implementation returns an empty set of events.
    fn events(&self, _attributes: &dyn ContractModelTrait) -> HashSet<Event> {
        HashSet()
    }

    /// Returns the state of a particular risk factor at a future time
    fn state_at(
        &self,
        id: &str,
        time: &NaiveDateTime,
        states: &StateSpace,
        attributes: &dyn ContractModelTrait,
    ) -> f64;
}
