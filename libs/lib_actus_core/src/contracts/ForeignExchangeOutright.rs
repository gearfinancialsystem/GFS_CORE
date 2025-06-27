use std::error::Error;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::types::isoDatetime::IsoDatetime;

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
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;

pub struct ForeignExchangeOutright;

impl ForeignExchangeOutright {
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_STK)),
                model.contractID.as_ref(),
            ));
        }

        // Termination event
        if let Some(termination_date) = &model.terminationDate {
            events.push(EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_STK)),
                model.contractID.as_ref(),
            ));
        } else {
            // Settlement events
            if model.deliverySettlement == Some(DeliverySettlement::D(D)) || model.deliverySettlement.is_none() {
                events.push(EventFactory::create_event_with_convention(
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    EventType::MD,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_MD1_FXOUT)),
                    Some(Rc::new(STF_MD1_FXOUT)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                ));

                events.push(EventFactory::create_event_with_convention(
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    EventType::MD,
                    model.currency2.as_ref(),
                    Some(Rc::new(POF_MD2_FXOUT)),
                    Some(Rc::new(STF_MD2_FXOUT)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                ));
            } else {
                let shifted_maturity_date = model.businessDayAdjuster.as_ref().unwrap().shift_bd(
                    &(model.maturityDate.clone().map(|rc| (*rc).clone()).unwrap() + model.settlementPeriod.clone().unwrap())
                );

                events.push(EventFactory::create_event_with_convention(
                    Some(shifted_maturity_date),
                    EventType::STD,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_STD_FXOUT)),
                    Some(Rc::new(STF_STD_FXOUT)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                ));
            }
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.statusDate,
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e >= &status_event);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort();

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        events.sort();

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &DayCountConvention::new_AAISDA(),
                model.businessDayAdjuster.as_ref().unwrap(),
            );
        }

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();
        states.statusDate = model.statusDate;
        states
    }
}
