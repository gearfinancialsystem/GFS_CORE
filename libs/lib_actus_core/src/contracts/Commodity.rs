use std::error::Error;
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
use crate::types::isoDatetime::IsoDatetime;

pub struct Commodity;

impl Commodity {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let status_date = model.statusDate.clone().unwrap();
        let purchase_date = model.purchaseDate.clone();
        let termination_date = model.terminationDate.clone();

        // Purchase
        if let Some(pd) = purchase_date {
            if pd > status_date && to > &pd {
                events.push(EventFactory::create_event(
                    Some(pd),
                    EventType::PRD,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_STK)),
                    Some(Rc::new(STF_PRD_STK)),
                    model.contractID.as_ref(),
                ));
            }
        }

        // Termination
        if let Some(td) = termination_date {
            if td > status_date && to > &td {
                events.push(EventFactory::create_event(
                    Some(td),
                    EventType::TD,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_TD_STK)),
                    Some(Rc::new(STF_TD_STK)),
                    model.contractID.as_ref(),
                ));
            }
        }

        Ok(events)
    }

    pub fn apply(
        mut events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        // Initialize state space per status date
        let mut states = StateSpace::default();
        states.statusDate = model.statusDate.clone();

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("AAISDA"), None, None).expect("etet"),
                &BusinessDayAdjuster::new("NOS", model.calendar.clone().unwrap()).expect("good NOS"),
            );
        }

        // Return evaluated events
        Ok(events)
    }
}
