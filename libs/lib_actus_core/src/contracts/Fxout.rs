use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;

use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractModel::ContractModel;
use crate::functions::fxout::pof::POF_MD1_FXOUT::POF_MD1_FXOUT;
use crate::functions::fxout::pof::POF_MD2_FXOUT::POF_MD2_FXOUT;
use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_STD_FXOUT::POF_STD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;
use crate::functions::fxout::stf::STF_MD1_FXOUT::STF_MD1_FXOUT;
use crate::functions::fxout::stf::STF_MD2_FXOUT::STF_MD2_FXOUT;
use crate::functions::fxout::stf::STF_STD_FXOUT::STF_STD_FXOUT;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub struct FXOUT;

impl TraitContractModel for FXOUT {
    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let e: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        } else {
            // Settlement events
            if model.delivery_settlement == Some(DeliverySettlement::D(D)) || model.delivery_settlement.is_none() {
                let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &EventType::MD,
                    &model.currency,
                    Some(Rc::new(POF_MD1_FXOUT)),
                    Some(Rc::new(STF_MD1_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());

                let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &EventType::MD,
                    &Some(model.currency2.clone().unwrap().to_currency()),
                    Some(Rc::new(POF_MD2_FXOUT)),
                    Some(Rc::new(STF_MD2_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            } else {
                let shifted_maturity_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                    &(model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap() +
                        model.settlement_period.clone().unwrap().value().clone()
                    ).value()
                );

                let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                    &Some(shifted_maturity_date),
                    &EventType::STD,
                    &model.currency,
                    Some(Rc::new(POF_STD_FXOUT)),
                    Some(Rc::new(STF_STD_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            }
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.to_iso_datetime_event() >= status_event.to_iso_datetime_event());

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort();

        Ok(events)
    }

    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, model: &ContractModel, observer: &DataObserver) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let _maturity = &model.maturity_date.clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("Failed to initialize state space");
        let mut events = events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new(Some("AAISDA"), None, None).ok(),
                model.business_day_adjuster.as_ref().unwrap(),
            );
        }

        Ok(events)
    }

    fn init_state_space(model: &ContractModel, _observer: &DataObserver, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        states.status_date = model.status_date.clone();
        Ok(states)
    }
}
impl fmt::Display for FXOUT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FXOUT")
    }
}