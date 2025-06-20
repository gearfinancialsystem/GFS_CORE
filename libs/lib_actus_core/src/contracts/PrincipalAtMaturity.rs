use std::error::Error;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;

use crate::functions::pam::pof::{
    POF_FP_PAM::POF_FP_PAM,
    POF_IED_PAM::POF_IED_PAM,
    POF_IP_PAM::POF_IP_PAM,
    POF_IPCI_PAM::POF_IPCI_PAM,
    POF_MD_PAM::POF_MD_PAM,
    POF_PRD_PAM::POF_PRD_PAM,
    POF_RR_PAM::POF_RR_PAM,
    POF_SC_PAM::POF_SC_PAM,
    POF_TD_PAM::POF_TD_PAM
};

use crate::functions::pam::stf::{
STF_FP_PAM::STF_FP_PAM,
    STF_IED_PAM::STF_IED_PAM,
    STF_IP_PAM::STF_IP_PAM,
    STF_IPCI_PAM::STF_IPCI_PAM,
    STF_MD_PAM::STF_MD_PAM,
    STF_PRD_PAM::STF_PRD_PAM,
    STF_RR_PAM::STF_RR_PAM,
    STF_RRF_PAM::STF_RRF_PAM,
    STF_SC_PAM::STF_SC_PAM,
    STF_TD_PAM::STF_TD_PAM,
};

use crate::state_space::StateSpace::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;

/// Represents the Principal At Maturity payoff algorithm
pub struct PrincipalAtMaturity;

impl PrincipalAtMaturity {
    /// Compute next events within the period up to `to` date based on the contract model
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events: Vec<ContractEvent> = Vec::new();

        // Initial exchange (IED)
        events.push(
            EventFactory::create_event(
                model.initialExchangeDate,
                EventType::IED,
                model.currency.as_ref(),
                Some(Rc::new(POF_IED_PAM)),
                Some(Rc::new(STF_IED_PAM)),
                model.contractID.as_ref(),
            )
        );

        // Principal redemption (MD)
        
        events.push(EventFactory::create_event(
            model.maturityDate.clone().map(|rc| (*rc).clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            model.contractID.as_ref(),
        ));

        // Purchase (PRD)
        if model.purchaseDate.is_some() {
            events.push(EventFactory::create_event(
                model.purchaseDate,
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_PAM)),
                Some(Rc::new(STF_PRD_PAM)),
                model.contractID.as_ref(),
            ));
        }

        // Interest payment related events
        if model.nominalInterestRate.is_some()
            && (model.cycleOfInterestPayment.is_some() || model.cycleAnchorDateOfInterestPayment.is_some())
        {
            // Generate raw interest payment events (IP)
            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestPayment.clone(),
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    model.cycleOfInterestPayment.clone(),
                    model.endOfMonthConvention.unwrap(),
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_PAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.businessDayConvention.clone().unwrap(),
                model.contractID.as_ref(),
            );

            // Adapt if interest capitalization is set
            if model.capitalizationEndDate.is_some() {
                // Remove IP events at IPCED and add IPCI event instead
                let capitalization_end = EventFactory::create_event_with_convention(
                    model.capitalizationEndDate,
                    EventType::IPCI,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(Rc::new(STF_IPCI_PAM)),
                    &model.businessDayConvention.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                // Remove IP events that occur at capitalization end date
                interest_events.retain(|e| {
                    !(e.eventType == EventType::IP && e.eventTime == Some(capitalization_end.get_event_time()))
                });

                // Add capitalization end event
                interest_events.insert(capitalization_end.clone());
                let mut vec: Vec<_> = interest_events.clone().into_iter().collect();
                // Change events with time <= IPCED and cont_type IP to IPCI


                vec.iter_mut()
                    .filter(|e| e.eventType == EventType::IP &&
                        e.get_event_time() <= capitalization_end.get_event_time())
                    .for_each(|e| {
                        e.chg_eventType(EventType::IPCI);
                        e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                        e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                    });

                // for e in vec.iter_mut() {
                //     if e.get_eventType() == EventType::IP
                //         && e.get_event_time() <= capitalization_end.get_event_time()
                //     {
                //         e.chg_eventType(EventType::IPCI);
                //         e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                //         e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                //     }
                // }
                interest_events = vec.into_iter().collect();
            }

            events.extend(interest_events);
        } else if model.capitalizationEndDate.is_some() {
            // If no interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.capitalizationEndDate,
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.businessDayConvention.clone().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        // Rate reset events (RR)
        let mut rate_reset_events = EventFactory::create_events_with_convention(
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
            Some(Rc::new(STF_RR_PAM)),
            &model.businessDayConvention.clone().unwrap(),
            model.contractID.as_ref(),
        );

        // Adapt fixed rate reset event
        if model.nextResetRate.is_some() {
            let status_event = EventFactory::create_event(
                model.statusDate,
                EventType::AD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );
            let mut vec: Vec<_> = rate_reset_events.clone().into_iter().collect();
            vec.sort();
            let fixed_event = vec.iter_mut().filter(|e| e.compare_to(&status_event)  == 1 ).next();

            if let Some(fixed_event_val) = fixed_event {
                fixed_event_val.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_val.chg_eventType(EventType::RRF);
                rate_reset_events.insert(fixed_event_val.clone());
            }


        }

        // Add all rate reset events
        events.extend(rate_reset_events);

        // Fee payment events (FP), if specified
        if model.cycleOfFee.is_some() {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    model.cycleOfFee.clone(),
                    model.endOfMonthConvention.unwrap(),
                    true,
                ),
                EventType::FP,
                model.currency.as_ref(),
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                &model.businessDayConvention.clone().unwrap(),
                model.contractID.as_ref(),
            );
            events.extend(fee_events);
        }

        // Scaling events (SC), if specified

        if model.scalingEffect.is_some() && (model.scalingEffect.clone().unwrap().to_string().contains('I') || model.scalingEffect.clone().unwrap().to_string().contains('N'))
        {
            let scaling_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfScalingIndex,
                    model.maturityDate.clone().map(|rc| (*rc).clone()),
                    model.cycleOfScalingIndex.clone(),
                    model.endOfMonthConvention.unwrap(),
                    false,
                ),
                EventType::SC,
                model.currency.as_ref(),
                Some(Rc::new(POF_SC_PAM)),
                Some(Rc::new(STF_SC_PAM)),
                &model.businessDayConvention.clone().unwrap(),
                model.contractID.as_ref(),
            );
            events.extend(scaling_events);
        }

        // Termination event (TD)
        if model.terminationDate.is_some() {
            let termination = EventFactory::create_event(
                model.terminationDate,
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
                model.contractID.as_ref(),
            );

            // Remove all events occurring after termination date
            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_date = model.statusDate;
        let status_event = EventFactory::create_event(
            status_date,
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );
        events.retain(|e| e >= &status_event);

        // Remove all events after the `to` date
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

    /// Apply a set of events to the current state of a contract and return the post-event states
    pub fn apply(
        events: &mut Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_StateSpace(model);

        // Sort events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.dayCountConvention.clone().unwrap(),
                &model.businessDayConvention.clone().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if model.purchaseDate.is_some() {
            let purchase_date = model.purchaseDate;
            let purchase_event = EventFactory::create_event(
                purchase_date,
                EventType::PRD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );
            events.retain(|e| {
                e.get_eventType() == EventType::AD || e >= &purchase_event
            });
        }

        // Return evaluated events
        events.clone()
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_StateSpace(
        model: &ContractModel,
    ) -> StateSpace {
        let mut states = StateSpace::default();

        states.notionalScalingMultiplier = model.notionalScalingMultiplier;
        states.interestScalingMultiplier = model.interestScalingMultiplier;
        states.contractPerformance = model.contractPerformance;
        states.statusDate = model.statusDate;

        let initial_exchange_date: IsoDatetime = model.initialExchangeDate.unwrap();
        if initial_exchange_date > states.statusDate.unwrap() {
            states.notionalPrincipal = Some(0.0);
            states.nominalInterestRate = Some(0.0);
        } else {

            let role_sign = model.contractRole.as_ref().map_or(1.0, |a| a.role_sign());
            states.notionalPrincipal = Some(role_sign * model.notionalPrincipal.unwrap());
            states.nominalInterestRate = model.nominalInterestRate;
        }

        // Initialize accrued interest
        if model.nominalInterestRate.is_none() {
            states.accruedInterest =  Some(0.0);
        } else if model.accruedInterest.is_some() {
            states.accruedInterest = model.accruedInterest;
        } else {
            let day_counter = model.dayCountConvention.as_ref().unwrap();
            let time_adjuster = model.businessDayConvention.as_ref().unwrap();


            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfInterestPayment,
                model.maturityDate.clone().map(|rc| (*rc).clone()),
                model.cycleOfInterestPayment.clone(),
                model.endOfMonthConvention.unwrap(),
                true,
            ).into_iter().collect();

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<&IsoDatetime> = ip_schedule
                .iter()
                .filter(|&&date| date < states.statusDate.unwrap())
                .collect();

            let t_minus = date_earlier_than_t0.last();
            println!("ok");
            states.accruedInterest = Some(day_counter.day_count_fraction(time_adjuster.shift_bd(t_minus.unwrap()),
                                                                    time_adjuster.shift_bd(&states.statusDate.unwrap()))
                * states.notionalPrincipal.unwrap()
                * states.nominalInterestRate.unwrap());

        }

        if model.feeRate.is_none() {
            states.feeAccrued = Some(0.0);
        } else if model.feeAccrued.is_some() {
            states.feeAccrued = model.feeAccrued;
        }
        // TODO: Implement last two possible initializations if needed

        states
    }
}
