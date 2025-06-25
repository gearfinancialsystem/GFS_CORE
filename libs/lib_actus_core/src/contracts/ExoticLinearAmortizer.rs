use std::error::Error;
use std::rc::Rc;
use std::collections::HashSet;
use crate::events::ContractEvent;
use crate::events::EventFactory;
use crate::events::EventType;
use crate::externals::RiskFactorModel;
use crate::functions::lam::{POF_IPCB_LAM, POF_IP_LAM, POF_PRD_LAM, POF_TD_LAM, STF_FP_LAM, STF_IED_LAM, STF_IPCB_LAM, STF_IPCI2_LAM, STF_IPCI_LAM, STF_PRD_LAM, STF_SC_LAM};
use crate::functions::lax::{POF_PI_LAX, POF_PR_LAX, STF_PI_LAX, STF_PI_LAX2, STF_PR_LAX, STF_PR_LAX2, STF_RRF_LAX, STF_RRY_LAM, STF_RR_LAX};
use crate::functions::pam::{POF_AD_PAM, POF_FP_PAM, POF_IED_PAM, POF_IPCI_PAM, POF_MD_PAM, POF_RR_PAM, POF_SC_PAM, STF_AD_PAM, STF_IP_PAM, STF_MD_LAM, STF_TD_PAM};
use crate::state_space::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::time::ScheduleFactory;
use crate::attributes::ContractModel;
use crate::conventions::InterestCalculationBase;

pub struct ExoticLinearAmortizer;

impl ExoticLinearAmortizer {
    pub fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model, to);

        // Initial exchange (IED)
        events.push(EventFactory::create_event(
            model.initialExchangeDate.clone(),
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            model.contractID.as_ref(),
        ));

        // Purchase event (PRD)
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                model.contractID.as_ref(),
            ));
        }

        // Principal redemption schedule
        if let Some(pr_anchor_dates) = &model.arrayCycleAnchorDateOfPrincipalRedemption {
            let pr_cycle = model.arrayCycleOfPrincipalRedemption.as_ref().map(|cycles| cycles.clone());
            let pr_payments = model.arrayNextPrincipalRedemptionPayment.as_ref().unwrap();
            let pr_inc_dec = model.arrayIncreaseDecrease.as_ref().unwrap();

            for i in 0..pr_anchor_dates.len() {
                let pr_type = if pr_inc_dec[i].eq_ignore_ascii_case("DEC") {
                    EventType::PR
                } else {
                    EventType::PI
                };

                let pr_stf: Rc<dyn StateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL) {
                    if pr_type == EventType::PR {
                        Rc::new(STF_PR_LAX(pr_payments[i]))
                    } else {
                        Rc::new(STF_PI_LAX(pr_payments[i]))
                    }
                } else {
                    if pr_type == EventType::PR {
                        Rc::new(STF_PR_LAX2(pr_payments[i]))
                    } else {
                        Rc::new(STF_PI_LAX2(pr_payments[i]))
                    }
                };

                let pr_pof: Rc<dyn PayOffFunction> = if pr_type == EventType::PR {
                    Rc::new(POF_PR_LAX(pr_payments[i]))
                } else {
                    Rc::new(POF_PI_LAX(pr_payments[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    Some(pr_anchor_dates[i].clone()),
                    Some(maturity.clone()),
                    pr_cycle.as_ref().map(|cycles| cycles[i].clone()),
                    model.endOfMonthConvention,
                    false,
                );

                let mut pr_events = EventFactory::create_events_with_convention(
                    &schedule,
                    pr_type,
                    model.currency.as_ref(),
                    Some(pr_pof),
                    Some(pr_stf),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                events.append(&mut pr_events);
            }
        }

        // Maturity event (MD)
        events.push(EventFactory::create_event_with_convention(
            Some(maturity.clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        ));

        // Interest payment schedule
        if let Some(ip_anchor_dates) = &model.arrayCycleAnchorDateOfInterestPayment {
            let ip_cycle = model.arrayCycleOfInterestPayment.as_ref().map(|cycles| cycles.clone());

            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_array_schedule(
                    ip_anchor_dates,
                    Some(maturity.clone()),
                    ip_cycle,
                    model.endOfMonthConvention,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            if let Some(capitalization_end_date) = &model.capitalizationEndDate {
                let stf_ipci: Rc<dyn StateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL) {
                    Rc::new(STF_IPCI_LAM)
                } else {
                    Rc::new(STF_IPCI2_LAM)
                };

                let capitalization_end = EventFactory::create_event_with_convention(
                    Some(capitalization_end_date.clone()),
                    EventType::IPCI,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                interest_events.retain(|e| {
                    !(e.eventType == EventType::IP && e.eventTime == capitalization_end.eventTime)
                });

                for e in &mut interest_events {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime {
                        e.eventType = EventType::IPCI;
                        e.fPayOff = Some(Rc::new(POF_IPCI_PAM));
                        e.fStateTrans = Some(stf_ipci.clone());
                    }
                }

                interest_events.insert(capitalization_end);
            }

            events.append(&mut interest_events);
        } else if let Some(capitalization_end_date) = &model.capitalizationEndDate {
            let stf_ipci: Rc<dyn StateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL) {
                Rc::new(STF_IPCI_LAM)
            } else {
                Rc::new(STF_IPCI2_LAM)
            };

            events.push(EventFactory::create_event_with_convention(
                Some(capitalization_end_date.clone()),
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        // Rate reset schedule
        if let Some(rr_anchor_dates) = &model.arrayCycleAnchorDateOfRateReset {
            let rr_cycle = model.arrayCycleOfRateReset.as_ref().map(|cycles| cycles.clone());
            let rr_rates = model.arrayRate.as_ref().unwrap();
            let rr_fixed_var = model.arrayFixedVariable.as_ref().unwrap();

            for i in 0..rr_anchor_dates.len() {
                let rr_type = if rr_fixed_var[i].eq_ignore_ascii_case("FIX") {
                    EventType::RRF
                } else {
                    EventType::RR
                };

                let rr_stf: Rc<dyn StateTransitionFunction> = if rr_type == EventType::RRF {
                    Rc::new(STF_RRF_LAX(rr_rates[i]))
                } else {
                    Rc::new(STF_RR_LAX(rr_rates[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    Some(rr_anchor_dates[i].clone()),
                    Some(maturity.clone()),
                    rr_cycle.as_ref().map(|cycles| cycles[i].clone()),
                    model.endOfMonthConvention,
                    false,
                );

                let mut rate_reset_events = EventFactory::create_events_with_convention(
                    &schedule,
                    rr_type,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_RR_PAM)),
                    Some(rr_stf),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                events.append(&mut rate_reset_events);
            }

            if let Some(next_reset_rate) = &model.nextResetRate {
                let mut rate_reset_events: Vec<ContractEvent> = events.iter()
                    .filter(|e| e.eventType == EventType::RR || e.eventType == EventType::RRF)
                    .cloned()
                    .collect();

                rate_reset_events.sort();

                if let Some(fixed_event) = rate_reset_events.iter_mut()
                    .find(|e| e.eventTime > model.statusDate) {
                    fixed_event.fStateTrans = Some(Rc::new(STF_RRY_LAM));
                    events.push(fixed_event.clone());
                }
            }
        }

        // Fee schedule
        if let Some(fee_cycle) = &model.cycleOfFee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    Some(maturity.clone()),
                    Some(fee_cycle.clone()),
                    model.endOfMonthConvention,
                    true,
                ),
                EventType::FP,
                model.currency.as_ref(),
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(fee_events);
        }

        // Scaling events
        if let Some(scaling_effect) = &model.scalingEffect {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfScalingIndex,
                        Some(maturity.clone()),
                        model.cycleOfScalingIndex.clone(),
                        model.endOfMonthConvention,
                        false,
                    ),
                    EventType::SC,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                events.extend(scaling_events);
            }
        }

        // Interest calculation base events
        if let Some(interest_calculation_base) = &model.interestCalculationBase {
            if *interest_calculation_base == InterestCalculationBase::NTL {
                let icb_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfInterestCalculationBase,
                        Some(maturity.clone()),
                        model.cycleOfInterestCalculationBase.clone(),
                        model.endOfMonthConvention,
                        false,
                    ),
                    EventType::IPCB,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                events.extend(icb_events);
            }
        }

        // Termination event
        if let Some(termination_date) = &model.terminationDate {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventTime <= termination.eventTime);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_date = model.statusDate;
        let status_event = EventFactory::create_event(
            Some(status_date.clone()),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            to,
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

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let maturity = Self::maturity(model, None);
        let mut states = Self::init_state_space(model, maturity);
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

    fn maturity(model: &ContractModel, to: Option<IsoDatetime>) -> IsoDatetime {
        if let Some(maturity_date) = &model.maturityDate {
            return maturity_date.clone();
        }

        let day_counter = model.dayCountConvention.as_ref().unwrap();
        let time_adjuster = model.businessDayAdjuster.as_ref().unwrap();
        let notional_principal = model.notionalPrincipal.unwrap();
        let pr_anchor_dates = model.arrayCycleAnchorDateOfPrincipalRedemption.as_ref().unwrap();
        let pr_inc_dec: Vec<i32> = model.arrayIncreaseDecrease.as_ref().unwrap().iter().map(|s| if s.eq_ignore_ascii_case("INC") { 1 } else { -1 }).collect();
        let pr_payments = model.arrayNextPrincipalRedemptionPayment.as_ref().unwrap();

        if model.arrayCycleOfPrincipalRedemption.is_none() {
            return pr_anchor_dates.last().unwrap().clone();
        }

        let pr_cycle = model.arrayCycleOfPrincipalRedemption.as_ref().unwrap();
        let mut t = model.statusDate.clone();
        let mut sum = 0.0;

        if pr_cycle.len() > 1 {
            let mut index = 0;
            let mut no_of_pr_events = 0;
            let mut pr_schedule = HashSet::new();

            loop {
                pr_schedule = ScheduleFactory::create_schedule(
                    Some(pr_anchor_dates[index].clone()),
                    Some(pr_anchor_dates[index + 1].clone()),
                    Some(pr_cycle[index].clone()),
                    model.endOfMonthConvention,
                    false,
                );

                no_of_pr_events = if (pr_schedule.len() as f64 * pr_payments[index] * pr_inc_dec[index] as f64) + notional_principal + sum >= 0.0 {
                    pr_schedule.len()
                } else {
                    ((notional_principal + sum) / pr_payments[index]).ceil() as usize
                };

                sum += no_of_pr_events as f64 * pr_inc_dec[index] as f64 * pr_payments[index];

                if pr_anchor_dates.len() - 2 == index {
                    no_of_pr_events = ((sum + notional_principal) / pr_payments[index + 1]).ceil().abs() as usize;
                    t = pr_anchor_dates[index + 1].clone();

                    for _ in 0..no_of_pr_events - 1 {
                        t = t.plus_period(&pr_cycle[index + 1]);
                    }

                    sum += no_of_pr_events as f64 * pr_inc_dec[index + 1] as f64 * pr_payments[index + 1];
                    break;
                } else {
                    index += 1;

                    for _ in 0..no_of_pr_events {
                        t = t.plus_period(&pr_cycle[index - 1]);
                    }
                }
            }
        } else {
            let no_of_pr_events = (notional_principal / pr_payments[0]).ceil() as usize;
            t = pr_anchor_dates[0].clone();

            for _ in 0..no_of_pr_events - 1 {
                t = t.plus_period(&pr_cycle[0]);
            }
        }

        time_adjuster.shift_event_time(t)
    }

    fn init_state_space(model: &ContractModel, maturity: IsoDatetime) -> StateSpace {
        let mut states = StateSpace::default();

        states.statusDate = model.statusDate.clone();
        states.notionalScalingMultiplier = 1.0;
        states.interestScalingMultiplier = 1.0;

        if model.initialExchangeDate.unwrap() > model.statusDate {
            states.notionalPrincipal = 0.0;
            states.nominalInterestRate = 0.0;
            states.interestCalculationBaseAmount = 0.0;
        } else {
            let role_sign = model.contractRole.as_ref().map_or(1.0, |role| role.role_sign());
            states.notionalPrincipal = role_sign * model.notionalPrincipal.unwrap();
            states.nominalInterestRate = model.nominalInterestRate.unwrap();
            states.accruedInterest = role_sign * model.accruedInterest.unwrap_or(0.0);
            states.feeAccrued = model.feeAccrued.unwrap_or(0.0);

            if model.interestCalculationBase == Some(InterestCalculationBase::NT) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                states.interestCalculationBaseAmount = role_sign * model.interestCalculationBaseAmount.unwrap_or(0.0);
            }
        }

        states
    }
}
