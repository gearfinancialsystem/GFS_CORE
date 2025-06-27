use std::error::Error;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::swppv::pof::POF_IED_SWPPV::POF_IED_SWPPV;
use crate::functions::swppv::pof::POF_IP_SWPPV::POF_IP_SWPPV;
use crate::functions::swppv::pof::POF_IPFIx_SWPPV::POF_IPFix_SWPPV;
use crate::functions::swppv::pof::POF_IPFloat_SWPPV::POF_IPFloat_SWPPV;
use crate::functions::swppv::pof::POF_MD_SWPPV::POF_MD_SWPPV;
use crate::functions::swppv::stf::STF_IED_SWPPV::STF_IED_SWPPV;
use crate::functions::swppv::stf::STF_IP_SWPPV::STF_IP_SWPPV;
use crate::functions::swppv::stf::STF_IPFix_SWPPV::STF_IPFix_SWPPV;
use crate::functions::swppv::stf::STF_IPFloat_SWPPV::STF_IPFloat_SWPPV;
use crate::functions::swppv::stf::STF_MD_SWPPV::STF_MD_SWPPV;
use crate::functions::swppv::stf::STF_PRD_SWPPV::STF_PRD_SWPPV;
use crate::functions::swppv::stf::STF_RR_SWPPV::STF_RR_SWPPV;
use crate::functions::swppv::stf::STF_TD_SWPPV::STF_TD_SWPPV;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::isoDatetime::IsoDatetime;

pub struct PlainVanillaInterestRateSwap;

impl PlainVanillaInterestRateSwap {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_SWPPV)),
                model.contractID.as_ref(),
            ));
        }

        // Initial exchange event
        events.push(EventFactory::create_event(
            model.initialExchangeDate.clone(),
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_SWPPV)),
            Some(Rc::new(STF_IED_SWPPV)),
            model.contractID.as_ref(),
        ));

        // Principal redemption event
        events.push(EventFactory::create_event(
            model.maturityDate.clone().map(|rc| (*rc).clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_SWPPV)),
            Some(Rc::new(STF_MD_SWPPV)),
            model.contractID.as_ref(),
        ));

        // Interest payment events
        if model.deliverySettlement == Some(DeliverySettlement::D(D)) || model.deliverySettlement.is_none() {
            // In case of physical delivery (delivery of individual cash flows)
            let interest_schedule = ScheduleFactory::create_schedule_end_time_true(
                model.cycleAnchorDateOfInterestPayment.clone(),
                model.maturityDate.clone().map(|rc| (*rc).clone()),
                model.cycleOfInterestPayment.clone(),
                model.endOfMonthConvention.clone().unwrap(),
            );

            // Fixed rate events
            let fixed_rate_events = EventFactory::create_events_with_convention(
                &interest_schedule,
                EventType::IPFX,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPFix_SWPPV)),
                Some(Rc::new(STF_IPFix_SWPPV)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            // Floating rate events
            let floating_rate_events = EventFactory::create_events_with_convention(
                &interest_schedule,
                EventType::IPFL,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPFloat_SWPPV)),
                Some(Rc::new(STF_IPFloat_SWPPV)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(fixed_rate_events);
            events.extend(floating_rate_events);
        } else {
            // In case of cash delivery (cash settlement)
            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule_end_time_true(
                    model.cycleAnchorDateOfInterestPayment.clone(),
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    model.cycleOfInterestPayment.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_SWPPV)),
                Some(Rc::new(STF_IP_SWPPV)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfRateReset.clone(),
                model.maturityDate.clone().map(|rc| (*rc).clone()),
                model.cycleOfRateReset.clone(),
                model.endOfMonthConvention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            model.currency.as_ref(),
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_SWPPV)),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        );

        events.extend(rate_reset_events);

        // Termination event
        if let Some(termination_date) = &model.terminationDate {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_SWPPV)),
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventTime <= termination.eventTime);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.statusDate.clone(),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime <= to_event.eventTime);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                model.dayCountConvention.as_ref().unwrap(),
                model.businessDayAdjuster.as_ref().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchaseDate {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventType == EventType::AD || e.eventTime >= purchase_event.eventTime);
        }

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notionalScalingMultiplier = Some(1.0);
        states.statusDate = model.statusDate;

        if model.initialExchangeDate <= model.statusDate {
            let role_sign = model.contractRole.as_ref().map_or(1.0, |role| role.role_sign());
            states.notionalPrincipal = Some(role_sign * model.notionalPrincipal.unwrap());
            states.nominalInterestRate = model.nominalInterestRate;
            states.nominalInterestRate2 = model.nominalInterestRate2;
            states.accruedInterest = Some(role_sign * model.accruedInterest.unwrap());
            states.accruedInterest2 = Some(role_sign * model.accruedInterest2.unwrap());
            states.lastInterestPeriod = Some(0.0);
        }

        states
    }
}
