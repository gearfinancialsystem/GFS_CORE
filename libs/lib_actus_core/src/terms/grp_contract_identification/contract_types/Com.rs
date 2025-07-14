use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;

pub struct COM;

impl TraitContractModel for COM {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let status_date = model.status_date.clone().unwrap();
        let purchase_date = model.purchase_date.clone();
        let termination_date = model.termination_date.clone();

        // Purchase
        if let Some(pd) = purchase_date {
            if pd.clone().value() > status_date.clone().value() && to.clone().unwrap() > pd.clone().value() {
                let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                    &Some(pd),
                    &EventType::PRD,
                    &model.currency,
                    Some(Rc::new(POF_PRD_STK)),
                    Some(Rc::new(STF_PRD_STK)),
                    &None,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            }
        }

        // Termination
        if let Some(td) = termination_date {
            if td.clone().value() > status_date.clone().value() && to.clone().unwrap() > td.clone().value() {
                let e: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                    &Some(td),
                    &EventType::TD,
                    &model.currency,
                    Some(Rc::new(POF_TD_STK)),
                    Some(Rc::new(STF_TD_STK)),
                    &None,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            }
        }

        Ok(events)
    }

    fn apply(
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        // Initialize state space per status date
        let mut states = StateSpace::default();
        states.status_date = model.status_date.clone();

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("AAISDA"), None, None).expect("etet"),
                &BusinessDayAdjuster::new("NOS", model.calendar.clone()).expect("good NOS"),
            );
        }

        // Return evaluated events
        Ok(events)
    }

    fn init_state_space(model: &ContractModel, _observer: &RiskFactorModel, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        Ok(states)
    }
}
impl fmt::Display for COM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "COM")
    }
}