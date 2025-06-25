use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;

use crate::attributes::ContractModel;
use crate::conventions::businessday::BusinessDayAdjuster;
use crate::conventions::daycount::DayCountCalculator;
use crate::conventions::endofmonth::EndOfMonthAdjuster;
use crate::events::{ContractEvent, EventFactory, EventType};
use crate::externals::RiskFactorModel;
use crate::functions::lam::{
    POF_IED_PAM, POF_IP_LAM, POF_IPCI_PAM, POF_MD_PAM, POF_PRD_LAM, POF_RR_PAM, POF_TD_LAM,
    STF_IED_LAM, STF_IP_PAM, STF_IPCI_LAM, STF_IPCI2_LAM, STF_MD_LAM, STF_PR2_LAM, STF_PR_LAM,
    STF_PRD_LAM, STF_RR_LAM, STF_RRF_LAM, STF_TD_PAM,
};
use crate::functions::pam::{POF_FP_PAM, POF_SC_PAM};
use crate::functions::StateTransitionFunction;
use crate::state_space::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::types::InterestCalculationBase;
use crate::util::{CycleUtils, RedemptionUtils};

pub struct LinearAmortizer;

impl LinearAmortizer {
    pub fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Initial exchange
        events.push(EventFactory::create_event(
            model.initialExchangeDate,
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            model.contractID.as_ref(),
        ));

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory::create_schedule(
            model.cycleAnchorDateOfPrincipalRedemption,
            Some(maturity.clone()),
            model.cycleOfPrincipalRedemption.clone(),
            model.endOfMonthConvention,
            false,
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn StateTransitionFunction> = if model.interestCalculationBase
            == Some(InterestCalculationBase::NTL)
        {
            Rc::new(STF_PR_LAM)
        } else {
            Rc::new(STF_PR2_LAM)
        };

        // Regular principal redemption events
        let mut pr_events = EventFactory::create_events_with_convention(
            &pr_schedule,
            EventType::PR,
            model.currency.as_ref(),
            Some(Rc::new(POF_PRD_LAM)),
            Some(stf),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        );

        events.append(&mut pr_events);

        // Maturity event
        events.push(EventFactory::create_event_with_convention(
            Some(maturity.clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        ));

        // Purchase event
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

        // Choose the right state transition function for IPCI depending on ipcb attributes
        let stf_ipci: Rc<dyn StateTransitionFunction> = if model.interestCalculationBase
            == Some(InterestCalculationBase::NTL)
        {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycleOfInterestPayment.is_some()
            || model.cycleAnchorDateOfInterestPayment.is_some()
        {
            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestPayment,
                    Some(maturity.clone()),
                    model.cycleOfInterestPayment.clone(),
                    model.endOfMonthConvention,
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            // Adapt if interest capitalization is set
            if let Some(capitalization_end_date) = &model.capitalizationEndDate {
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
                    !(e.eventType == EventType::IP
                        && e.eventTime == capitalization_end.eventTime)
                });

                interest_events.push(capitalization_end);

                for e in &mut interest_events {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime {
                        e.eventType = EventType::IPCI;
                        e.fPayOff = Some(Rc::new(POF_IPCI_PAM));
                        e.fStateTrans = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events);
        } else if model.capitalizationEndDate.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.capitalizationEndDate,
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfRateReset,
                Some(maturity.clone()),
                model.cycleOfRateReset.clone(),
                model.endOfMonthConvention,
                false,
            ),
            EventType::RR,
            model.currency.as_ref(),
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            model.businessDayAdjuster.as_ref().unwrap(),
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

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

            if let Some(fixed_event) = sorted_events
                .iter()
                .find(|&&e| e.eventTime > status_event.eventTime)
            {
                let mut fixed_event = fixed_event.clone();
                fixed_event.fStateTrans = Some(Rc::new(STF_RRF_LAM));
                fixed_event.eventType = EventType::RRF;
                rate_reset_events.push(fixed_event);
            }
        }

        events.append(&mut rate_reset_events);

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycleOfFee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    Some(maturity.clone()),
                    Some(cycle_of_fee.clone()),
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

        // Scaling events (if specified)
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

        // Interest calculation base events (if specified)
        if model.interestCalculationBase == Some(InterestCalculationBase::NTL) {
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
        let status_event = EventFactory::create_event(
            model.statusDate,
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

        // Remove all post to-date events
        let to_date = to.unwrap_or(maturity);
        let post_date = EventFactory::create_event(
            Some(to_date),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime <= post_date.eventTime);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let maturity = Self::maturity(model);
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

    fn maturity(model: &ContractModel) -> IsoDatetime {
        if let Some(maturity) = model.maturityDate {
            return maturity;
        }

        let last_event: IsoDatetime;
        let remaining_periods: i32;
        let cycle_anchor_date = model.cycleAnchorDateOfPrincipalRedemption.unwrap();
        let status_date = model.statusDate.unwrap();
        let cycle = model.cycleOfPrincipalRedemption.as_ref().unwrap();

        if cycle_anchor_date < status_date {
            let mut previous_events = ScheduleFactory::create_schedule(
                Some(cycle_anchor_date.clone()),
                Some(status_date.clone()),
                Some(cycle.clone()),
                model.endOfMonthConvention,
                false,
            );

            let cycle_period = CycleUtils::parse_period(cycle);
            previous_events.retain(|d| {
                d >= &status_date.minus_period(&cycle_period)
                    && d != &status_date
            });

            last_event = previous_events.iter().next().unwrap().clone();
            remaining_periods = (model.notionalPrincipal.unwrap()
                / model.nextPrincipalRedemptionPayment.unwrap())
                .ceil() as i32;
        } else {
            last_event = cycle_anchor_date;
            remaining_periods = ((model.notionalPrincipal.unwrap()
                / model.nextPrincipalRedemptionPayment.unwrap())
                .ceil() as i32)
                - 1;
        }

        let adjuster = EndOfMonthAdjuster::new(
            model.endOfMonthConvention,
            last_event,
            cycle.clone(),
        );

        adjuster.shift(last_event.plus_period(&cycle_period.multiplied_by(remaining_periods)))
    }

    fn init_state_space(
        model: &ContractModel,
        maturity: IsoDatetime,
    ) -> StateSpace {
        let mut states = StateSpace::default();

        states.maturityDate = Some(maturity);

        if model.initialExchangeDate.unwrap() > model.statusDate.unwrap() {
            states.notionalPrincipal = Some(0.0);
            states.nominalInterestRate = Some(0.0);
            states.interestCalculationBaseAmount = Some(0.0);
        } else {
            let role_sign = model.contractRole.as_ref().map_or(1.0, |role| role.role_sign());
            states.notionalPrincipal = Some(role_sign * model.notionalPrincipal.unwrap());
            states.nominalInterestRate = model.nominalInterestRate;

            if model.interestCalculationBase == Some(InterestCalculationBase::NT) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                states.interestCalculationBaseAmount = Some(
                    role_sign * model.interestCalculationBaseAmount.unwrap(),
                );
            }
        }

        if model.nominalInterestRate.is_none() {
            states.accruedInterest = Some(0.0);
        } else if model.accruedInterest.is_some() {
            states.accruedInterest = model.accruedInterest;
        }

        if model.feeRate.is_none() {
            states.feeAccrued = Some(0.0);
        } else if model.feeAccrued.is_some() {
            states.feeAccrued = model.feeAccrued;
        }

        states.notionalScalingMultiplier = model.notionalScalingMultiplier;
        states.interestScalingMultiplier = model.interestScalingMultiplier;
        states.contractPerformance = model.contractPerformance;
        states.statusDate = model.statusDate;

        if model.nextPrincipalRedemptionPayment.is_none() {
            states.nextPrincipalRedemptionPayment =
                Some(RedemptionUtils::redemption_amount(model, &states));
        } else {
            states.nextPrincipalRedemptionPayment = model.nextPrincipalRedemptionPayment;
        }

        states
    }
}
