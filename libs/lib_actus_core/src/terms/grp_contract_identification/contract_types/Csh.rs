use std::error::Error;
use std::fmt;
use crate::events::ContractEvent::ContractEvent;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

pub struct CSH;

impl CSH {
    pub fn schedule(
        _to: &IsoDatetime,
        _model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, Box<dyn Error>> {
        Ok(Vec::new())
    }

    pub fn apply(
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent<IsoDatetime, IsoDatetime>> {
        // Initialize state space per status date
        let mut states = StateSpace::default();
        states.status_date = model.status_date.clone();
        states.notional_principal = NotionalPrincipal::new(&model.contract_role.clone().unwrap().role_sign() * model.notional_principal.clone().unwrap().value()).ok();

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("AAISDA"), None, None).expect("etet"),
                &BusinessDayAdjuster::new("NOS", model.calendar.clone()).expect("good ba"),  //&DayCountConvention::new(None, None),
            );
        }

        // Return evaluated events
        events
    }
}
impl fmt::Display for CSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSH")
    }
}