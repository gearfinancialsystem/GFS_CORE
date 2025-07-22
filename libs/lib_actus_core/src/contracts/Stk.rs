
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;



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
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use crate::util_tests::essai_data_observer::DataObserver;

/// Represents the Principal At Maturity payoff algorithm
pub struct STK;

impl TraitContractModel for STK {
    /// Compute next events within the period up to `to` date based on the contract model
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events = Vec::new();


        if model.purchase_date.is_some(){
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }
        if model.cycle_of_dividend_payment.is_some(){
            if model.termination_date.is_none(){
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment,
                            &Some(model.cycle_anchor_date_of_dividend_payment.clone().unwrap() + IsoPeriod::of_years(10)), // definir les constantes
                            &model.cycle_of_dividend_payment,
                            &model.end_of_month_convention,
                            Some(true)
                        ),
                        &EventType::DV,
                        &model.currency,
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.business_day_adjuster,
                        &model.contract_id)
                );
            }
            else {
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment,
                            &model.termination_date,
                            &model.cycle_of_dividend_payment,
                            &model.end_of_month_convention,
                            Some(true)),
                        &EventType::DV,
                        &model.currency,
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.business_day_adjuster,
                        &model.contract_id)
                )
            }
        }
        if model.termination_date.is_some(){
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &model.termination_date,
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.compare_to(&termination.to_iso_datetime_event()) != 1
            });
            events.push(termination.to_iso_datetime_event());
        }
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    &Some(model.status_date.clone().unwrap().value()),
                    &EventType::TD,
                    &model.currency,
                    None,
                    None,
                    &None,
                    &model.contract_id
                )
            }) != -1
        });
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    &Some(to.clone().clone().unwrap()),
                    &EventType::AD,
                    &model.currency,
                    None,
                    None,
                    &None,
                    &model.contract_id
                )
            }) != 1
        });

        events.sort();
        Ok(events.clone())
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    fn apply(
        events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &DataObserver,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        // Initialize state space per status date
        let _maturity = &model.maturity_date.clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("Failed to initialize states_space");
        let mut events = events.clone();
        // Sort events according to their time sequence
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        events.iter_mut().for_each(|e| {
            e.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("E30360"), None, None).ok(),
                &model.business_day_adjuster.clone().unwrap()
            )
        });
        // Return evaluated events
        Ok(events.clone())
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_state_space(
        model: &ContractModel, _observer: &DataObserver, _maturity: &Option<Rc<MaturityDate>>
    ) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        states.status_date = model.status_date.clone();

        Ok(states)
    }
}
impl fmt::Display for STK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STK")
    }
}