use std::error::Error;
use std::rc::Rc;

use crate::events::ContractEvent;
use crate::events::EventFactory;
use crate::events::EventType;
use crate::externals::RiskFactorModel;
use crate::state_space::StateSpace;
use crate::attributes::ContractModel;
use crate::conventions::daycount::DayCountCalculator;
use crate::conventions::businessday::BusinessDayAdjuster;
use crate::functions::stk::{POF_PRD_STK, STF_PRD_STK, POF_TD_STK, STF_TD_STK};
use crate::types::IsoDatetime;

pub struct Commodity;

impl Commodity {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let status_date = model.status_date.clone().unwrap();
        let purchase_date = model.purchase_date.clone();
        let termination_date = model.termination_date.clone();

        // Purchase
        if let Some(pd) = purchase_date {
            if pd > status_date && to > &pd {
                events.push(EventFactory::create_event(
                    Some(pd),
                    EventType::PRD,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_STK)),
                    Some(Rc::new(STF_PRD_STK)),
                    model.contract_id.as_ref(),
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
                    model.contract_id.as_ref(),
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
        states.status_date = model.status_date.clone();

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountCalculator::new("AA", None),
                &BusinessDayAdjuster::new(None, None),
            );
        }

        // Return evaluated events
        Ok(events)
    }
}
