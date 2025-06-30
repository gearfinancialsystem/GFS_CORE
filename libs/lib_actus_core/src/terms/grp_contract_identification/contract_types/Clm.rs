use std::error::Error;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::functions::clm::pof::POF_IED_CLM::POF_IED_CLM;
use crate::functions::clm::pof::POF_IP_CLM::POF_IP_CLM;
use crate::functions::clm::stf::STF_IP_CLM::STF_IP_CLM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_MD_PAM::STF_MD_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;

pub struct CLM;

impl CLM {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Determine maturity of the contract
        let maturity = Self::maturity(model, to);

        // Initial exchange
        events.push(EventFactory::create_event(
            model.initialExchangeDate.clone(),
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_CLM)),
            Some(Rc::new(STF_IED_PAM)),
            model.contractID.as_ref(),
        ));

        // Interest payment event
        events.push(EventFactory::create_event(
            Some(maturity.clone()),
            EventType::IP,
            model.currency.as_ref(),
            Some(Rc::new(POF_IP_CLM)),
            Some(Rc::new(STF_IP_CLM)),
            model.contractID.as_ref(),
        ));

        // Principal redemption
        events.push(EventFactory::create_event(
            Some(maturity.clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            model.contractID.as_ref(),
        ));

        // Interest payment capitalization (if specified)
        if model.cycleOfInterestPayment.is_some() {
            let cycle_anchor_date = if model.cycleAnchorDateOfInterestPayment.is_none() {
                model.initialExchangeDate.clone().unwrap() + CycleUtils::parse_period(&model.cycleOfInterestPayment.clone().unwrap()).unwrap()
            } else {
                model.cycleAnchorDateOfInterestPayment.clone().unwrap()
            };

            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    Some(cycle_anchor_date),
                    Some(maturity.clone()),
                    model.cycleOfInterestPayment.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                    false,
                ),
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfRateReset.clone(),
                Some(maturity.clone()),
                model.cycleOfRateReset.clone(),
                model.endOfMonthConvention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            model.currency.as_ref(),
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        );

        // Adapt fixed rate reset event
        if model.nextResetRate.is_some() {
            let status_event = EventFactory::create_event(
                model.statusDate.clone(),
                EventType::AD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort();

            if let Some(fixed_event) = sorted_events.iter().find(|&&e| e.compare_to(&status_event) == 1).cloned() {
                let mut fixed_event_clone = fixed_event.clone();
                fixed_event_clone.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_clone.chg_eventType(EventType::RRF);
                rate_reset_events.insert(fixed_event_clone);
            }
        }

        events.extend(rate_reset_events);

        // Fees (if specified)
        if model.cycleOfFee.is_some() {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee.clone(),
                    Some(maturity.clone()),
                    model.cycleOfFee.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                    false,
                ),
                EventType::FP,
                model.currency.as_ref(),
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(fee_events);
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

        events.retain(|e| e.compare_to(&status_event) != -1);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        // Sort the events according to their time of occurrence
        events.sort();

        Ok(events)
    }

    pub fn apply(
        mut events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_state_space(model);

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.dayCountConvention.clone().unwrap(),
                &model.businessDayAdjuster.clone().unwrap(),
            );
        }

        events
    }

    fn maturity(model: &ContractModel, to: &IsoDatetime) -> IsoDatetime {
        model.maturityDate.clone().unwrap_or(Rc::new(to.clone())).clone().as_ref().clone()
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notionalScalingMultiplier = Some(1.0);
        states.interestScalingMultiplier = Some(1.0);
        states.statusDate = model.statusDate.clone();

        if model.initialExchangeDate.clone().unwrap() <= model.statusDate.clone().unwrap() {
            let role_sign = model.contractRole.as_ref().unwrap().role_sign();
            states.notionalPrincipal = Some(role_sign * model.notionalPrincipal.unwrap());
            states.nominalInterestRate = model.nominalInterestRate;
            states.accruedInterest = Some(role_sign * model.accruedInterest.unwrap());
            states.feeAccrued = model.feeAccrued;
        }

        states
    }
}