use std::error::Error;
use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::HashSet;

use log::{debug, info, warn, error};

// Assuming these are defined elsewhere in your Rust project
use crate::events::ContractEvent;
use crate::events::EventFactory;
use crate::events::EventType;
use crate::externals::RiskFactorModel;
use crate::state_space::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::time::ScheduleFactory;
use crate::attributes::ContractModel;
use crate::conventions::businessday::BusinessDayAdjuster;
use crate::conventions::daycount::DayCountCalculator;
use crate::functions::pam::*;
use crate::functions::lam::*;
use crate::functions::nam::*;
use crate::functions::ann::*;
use crate::functions::StateTransitionFunction;

pub struct Annuity;

impl Annuity {
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Initial exchange (IED)
        events.push(EventFactory::create_event(
            model.initialExchangeDate,
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            model.contractID.as_ref(),
        ));

        // Principal redemption (MD)
        events.push(EventFactory::create_event(
            maturity,
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            model.contractID.as_ref(),
        ));

        // Principal redemption schedule (PR)
        let stf = if model.interestCalculationBase != InterestCalculationBase::NT {
            Rc::new(STF_PR_NAM)
        } else {
            Rc::new(STF_PR2_NAM)
        };

        events.extend(EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfPrincipalRedemption,
                maturity,
                model.cycleOfPrincipalRedemption,
                model.endOfMonthConvention,
                false,
            ),
            EventType::PR,
            model.currency.as_ref(),
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            model.businessDayAdjuster.as_ref(),
            model.contractID.as_ref(),
        ));

        // Initial principal redemption fixing event (PRF)
        if model.nextPrincipalRedemptionPayment.is_none() {
            events.push(EventFactory::create_event(
                model.cycleAnchorDateOfPrincipalRedemption.map(|d| d - Duration::days(1)),
                EventType::PRF,
                model.currency.as_ref(),
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRF_ANN)),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            ));
        }

        // Fees (FP)
        if model.cycleOfFee.is_some() {
            events.extend(EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    maturity,
                    model.cycleOfFee,
                    model.endOfMonthConvention,
                    true,
                ),
                EventType::FP,
                model.currency.as_ref(),
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            ));
        }

        // Purchase (PRD)
        if let Some(purchase_date) = model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                model.contractID.as_ref(),
            ));
        }

        // Interest payment related events (IP)
        let stf_ipci = if model.interestCalculationBase == Some(InterestCalculationBase::NTL) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        if model.nominalInterestRate.is_some() && (model.cycleOfInterestPayment.is_some() || model.cycleAnchorDateOfInterestPayment.is_some()) {
            let mut interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestPayment,
                    maturity,
                    model.cycleOfInterestPayment,
                    model.endOfMonthConvention,
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            );

            if model.cycleAnchorDateOfInterestPayment != model.cycleAnchorDateOfPrincipalRedemption || model.cycleOfInterestPayment != model.cycleOfPrincipalRedemption {
                let prcl = CycleUtils::parse_period(&model.cycleOfPrincipalRedemption.unwrap());
                let pranxm = model.cycleAnchorDateOfPrincipalRedemption.unwrap() - prcl;
                interest_events.retain(|e| !(e.eventType == EventType::IP && e.eventTime >= pranxm));

                let ipanxm = EventFactory::create_event(
                    Some(pranxm),
                    EventType::IP,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.businessDayAdjuster.as_ref(),
                    model.contractID.as_ref(),
                );
                interest_events.push(ipanxm);

                interest_events.extend(EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfPrincipalRedemption,
                        maturity,
                        model.cycleOfPrincipalRedemption,
                        model.endOfMonthConvention,
                        true,
                    ),
                    EventType::IP,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.businessDayAdjuster.as_ref(),
                    model.contractID.as_ref(),
                ));
            }

            if let Some(capitalization_end_date) = model.capitalizationEndDate {
                let capitalization_end = EventFactory::create_event(
                    Some(capitalization_end_date),
                    EventType::IPCI,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci),
                    model.businessDayAdjuster.as_ref(),
                    model.contractID.as_ref(),
                );

                interest_events.retain(|e| !(e.eventType == EventType::IP && e.eventTime == capitalization_end.eventTime));
                interest_events.push(capitalization_end);

                for e in &mut interest_events {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime {
                        e.eventType = EventType::IPCI;
                        e.fPayOff = Some(Rc::new(POF_IPCI_PAM));
                        e.fStateTrans = Some(stf_ipci.clone());
                    }
                }
            }

            events.extend(interest_events);
        } else if model.capitalizationEndDate.is_some() {
            events.push(EventFactory::create_event(
                model.capitalizationEndDate,
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            ));
        } else if model.cycleOfInterestPayment.is_none() && model.cycleAnchorDateOfInterestPayment.is_none() {
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfPrincipalRedemption,
                    maturity,
                    model.cycleOfPrincipalRedemption,
                    model.endOfMonthConvention,
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            );
            events.extend(interest_events);
        }

        // Interest calculation base (IPCB)
        if let Some(interest_calculation_base) = &model.interestCalculationBase {
            if interest_calculation_base == &InterestCalculationBase::NTL {
                events.extend(EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfInterestCalculationBase,
                        maturity,
                        model.cycleOfInterestCalculationBase,
                        model.endOfMonthConvention,
                        false,
                    ),
                    EventType::IPCB,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    model.businessDayAdjuster.as_ref(),
                    model.contractID.as_ref(),
                ));
            }
        }

        // Rate reset events (RR)
        let mut rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfRateReset,
                maturity,
                model.cycleOfRateReset,
                model.endOfMonthConvention,
                false,
            ),
            EventType::RR,
            model.currency.as_ref(),
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            model.businessDayAdjuster.as_ref(),
            model.contractID.as_ref(),
        );

        if let Some(next_reset_rate) = model.nextResetRate {
            let status_event = EventFactory::create_event(
                model.statusDate,
                EventType::AD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            if let Some(fixed_event) = rate_reset_events.iter_mut().find(|e| e > &status_event) {
                fixed_event.fStateTrans = Some(Rc::new(STF_RRF_LAM));
                fixed_event.eventType = EventType::RRF;
                rate_reset_events.push(fixed_event.clone());
            }
        }

        events.extend(rate_reset_events);

        let prf_schedule: HashSet<_> = rate_reset_events.iter().map(|e| e.eventTime).collect();
        if !prf_schedule.is_empty() {
            events.extend(EventFactory::create_events(
                &prf_schedule.into_iter().collect::<Vec<_>>(),
                EventType::PRF,
                model.currency.as_ref(),
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRF_ANN)),
                model.businessDayAdjuster.as_ref(),
                model.contractID.as_ref(),
            ));
        }

        // Scaling events (SC)
        if let Some(scaling_effect) = &model.scalingEffect {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                events.extend(EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfScalingIndex,
                        maturity,
                        model.cycleOfScalingIndex,
                        model.endOfMonthConvention,
                        false,
                    ),
                    EventType::SC,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    model.businessDayAdjuster.as_ref(),
                    model.contractID.as_ref(),
                ));
            }
        }

        // Termination event (TD)
        if let Some(termination_date) = model.terminationDate {
            let termination = EventFactory::create_event(
                Some(termination_date),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                model.contractID.as_ref(),
            );

            events.retain(|e| e <= &termination);
            events.push(termination);
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
        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events;

        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        for event in &mut events {
            event.eval(
                &mut states,
                model,
                observer,
                &model.dayCountConvention.as_ref().unwrap(),
                &model.businessDayAdjuster.as_ref().unwrap(),
            );
        }

        if let Some(purchase_date) = model.purchaseDate {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date),
                EventType::PRD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventType == EventType::AD || e >= &purchase_event);
        }

        events
    }

    fn maturity(model: &ContractModel) -> IsoDatetime {
        if let Some(maturity_date) = model.maturityDate {
            return maturity_date;
        }

        if let Some(amortization_date) = model.amortizationDate {
            return amortization_date;
        }

        let t0 = model.statusDate.unwrap();
        let pranx = model.cycleAnchorDateOfPrincipalRedemption.unwrap();
        let ied = model.initialExchangeDate.unwrap();
        let prcl = CycleUtils::parse_period(&model.cycleOfPrincipalRedemption.as_ref().unwrap());

        let last_event = if pranx >= t0 {
            pranx
        } else if ied + prcl > t0 {
            ied + prcl
        } else {
            let mut previous_events = ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfPrincipalRedemption,
                Some(t0),
                model.cycleOfPrincipalRedemption,
                model.endOfMonthConvention,
            );

            previous_events.retain(|&d| d > t0);
            previous_events.sort();
            *previous_events.last().unwrap()
        };

        let time_from_last_event_plus_one_cycle = model.dayCountConvention.as_ref().unwrap().day_count_fraction(last_event, last_event + prcl);
        let redemption_per_cycle = model.nextPrincipalRedemptionPayment.unwrap() - (time_from_last_event_plus_one_cycle * model.nominalInterestRate.unwrap() * model.notionalPrincipal.unwrap());
        let remaining_periods = ((model.notionalPrincipal.unwrap() / redemption_per_cycle).ceil() - 1.0) as i32;

        model.businessDayAdjuster.as_ref().unwrap().shift_event_time(last_event + prcl * remaining_periods)
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();
        states.notionalScalingMultiplier = model.notionalScalingMultiplier;
        states.interestScalingMultiplier = model.interestScalingMultiplier;
        states.contractPerformance = model.contractPerformance;
        states.statusDate = model.statusDate;
        states.maturityDate = Some(Self::maturity(model));

        if model.initialExchangeDate.unwrap() > model.statusDate.unwrap() {
            states.notionalPrincipal = 0.0;
            states.nominalInterestRate = 0.0;
            states.interestCalculationBaseAmount = 0.0;
        } else {
            states.notionalPrincipal = ContractRoleConvention::role_sign(&model.contractRole.unwrap()) * model.notionalPrincipal.unwrap();
            states.nominalInterestRate = model.nominalInterestRate.unwrap();

            if model.interestCalculationBase == Some(InterestCalculationBase::NT) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                states.interestCalculationBaseAmount = ContractRoleConvention::role_sign(&model.contractRole.unwrap()) * model.interestCalculationBaseAmount.unwrap();
            }
        }

        if model.nominalInterestRate.is_none() {
            states.accruedInterest = 0.0;
        } else if model.accruedInterest.is_some() {
            states.accruedInterest = ContractRoleConvention::role_sign(&model.contractRole.unwrap()) * model.accruedInterest.unwrap();
        } else {
            let day_counter = model.dayCountConvention.as_ref().unwrap();
            let time_adjuster = model.businessDayAdjuster.as_ref().unwrap();

            let mut ip_schedule = ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfInterestPayment,
                states.maturityDate,
                model.cycleOfInterestPayment,
                model.endOfMonthConvention,
                true,
            );

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<_> = ip_schedule.iter().filter(|&&date| date < states.statusDate.unwrap()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accruedInterest = day_counter.day_count_fraction(
                time_adjuster.shift_calc_time(*t_minus),
                time_adjuster.shift_calc_time(states.statusDate.unwrap()),
            ) * states.notionalPrincipal * states.nominalInterestRate;
        }

        if model.feeRate.is_none() {
            states.feeAccrued = 0.0;
        } else if model.feeAccrued.is_some() {
            states.feeAccrued = model.feeAccrued.unwrap();
        }

        if model.nextPrincipalRedemptionPayment.is_none() {
            if model.initialExchangeDate.unwrap() > model.statusDate.unwrap() {
                // Fixed at initial PRF event
            } else {
                states.nextPrincipalRedemptionPayment = RedemptionUtils::redemption_amount(model, &states);
            }
        } else {
            states.nextPrincipalRedemptionPayment = model.nextPrincipalRedemptionPayment;
        }

        states
    }
}
