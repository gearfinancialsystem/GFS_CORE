use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;


use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::functions::stk::pof::POF_DV_STK::POF_DV_STK;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::functions::stk::stf::STF_DV_STK::STF_DV_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoPeriod::IsoPeriod;

/// Represents the Principal At Maturity payoff algorithm
pub struct STK;

impl STK {
    /// Compute next events within the period up to `to` date based on the contract model
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();


        if model.purchaseDate.is_some(){
            events.push(EventFactory::create_event(
                model.purchaseDate,
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                model.contractID.as_ref(),
            ));
        }
        if model.cycleOfDividendPayment.is_some(){
            if model.terminationDate.is_none(){
                events.extend(
                    EventFactory::create_events_with_convention(
                        &ScheduleFactory::create_schedule_end_time_true(
                            model.cycleAnchorDateOfDividendPayment,
                            Some(model.cycleAnchorDateOfDividendPayment.clone().unwrap() + IsoPeriod::of_years(10)), // definir les constantes
                            model.cycleOfDividendPayment.clone(),
                            model.endOfMonthConvention.clone().unwrap()
                        ),
                        EventType::DV,
                        model.currency.as_ref(),
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.businessDayAdjuster.clone().unwrap(),
                        model.contractID.as_ref())
                );
            }
            else {
                events.extend(
                    EventFactory::create_events_with_convention(
                        &ScheduleFactory::create_schedule_end_time_true(
                            model.cycleAnchorDateOfDividendPayment,
                            model.terminationDate.clone(),
                            model.cycleOfDividendPayment.clone(),
                            model.endOfMonthConvention.unwrap()),
                        EventType::DV,
                        model.currency.as_ref(),
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.businessDayAdjuster.clone().unwrap(),
                        model.contractID.as_ref())
                )
            }
        }
        if model.terminationDate.is_some(){
            let termination = EventFactory::create_event(
                model.terminationDate,
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                model.contractID.as_ref(),
            );
            events.retain(|e| {
                e.compare_to(&termination) != 1
            });
            events.push(termination);
        }
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    model.statusDate,
                    EventType::TD,
                    model.currency.as_ref(),
                    None,
                    None,
                    model.contractID.as_ref()
                )
            }) != -1
        });
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    Some(to.clone()),
                    EventType::AD,
                    model.currency.as_ref(),
                    None,
                    None,
                    model.contractID.as_ref()
                )
            }) != 1
        });

        events.sort();
        Ok(events.clone())
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_StateSpace(model);
        let mut events = events.clone();
        // Sort events according to their time sequence
        events.sort();

        events.iter_mut().for_each(|e| {
            e.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("E30360"), None, None).unwrap(),
                &model.businessDayAdjuster.clone().unwrap()
            )
        });
        // Return evaluated events
        events.clone()
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_StateSpace(
        model: &ContractModel,
    ) -> StateSpace {
        let mut states = StateSpace::default();
        states.statusDate = model.statusDate;

        states
    }
}
impl fmt::Display for STK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STK")
    }
}