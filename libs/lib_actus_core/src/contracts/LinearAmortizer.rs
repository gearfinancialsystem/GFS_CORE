use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;

use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::events::EventType::EventType::RRF;

use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::lam::pof::POF_IP_LAM::POF_IP_LAM;
use crate::functions::lam::pof::POF_IPCB_LAM::POF_IPCB_LAM;
use crate::functions::lam::pof::POF_PRD_LAM::POF_PRD_LAM;
use crate::functions::lam::pof::POF_TD_LAM::POF_TD_LAM;
use crate::functions::lam::stf::STF_FP_LAM::STF_FP_LAM;
use crate::functions::lam::stf::STF_IED_LAM::STF_IED_LAM;
use crate::functions::lam::stf::STF_IPBC_LAM::STF_IPCB_LAM;
use crate::functions::lam::stf::STF_IPCI2_LAM::STF_IPCI2_LAM;
use crate::functions::lam::stf::STF_IPCI_LAM::STF_IPCI_LAM;
use crate::functions::lam::stf::STF_MD_LAM::STF_MD_LAM;
use crate::functions::lam::stf::STF_PR2_LAM::STF_PR2_LAM;
use crate::functions::lam::stf::STF_PR_LAM::STF_PR_LAM;
use crate::functions::lam::stf::STF_PRD_LAM::STF_PRD_LAM;
use crate::functions::lam::stf::STF_RR_LAM::STF_RR_LAM;
use crate::functions::lam::stf::STF_RRF_LAM::STF_RRF_LAM;
use crate::functions::lam::stf::STF_SC_LAM::STF_SC_LAM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_SC_PAM::POF_SC_PAM;
use crate::functions::pam::stf::STF_IP_PAM::STF_IP_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::terms::grp_interest::interest_calculation_base::Ntied::NTIED;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;
use crate::util::RedemptionUtils::RedemptionUtils;

pub struct LinearAmortizer;

impl LinearAmortizer {
    pub fn schedule(
        to: &IsoDatetime,
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
            model.endOfMonthConvention.clone().unwrap(),
            false,
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase
            == Some(InterestCalculationBase::NTL(NTL))
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
        ).into_iter().collect();

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
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase
            == Some(InterestCalculationBase::NTL(NTL))
        {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycleOfInterestPayment.is_some()
            || model.cycleAnchorDateOfInterestPayment.is_some()
        {
            let mut interest_events: Vec<ContractEvent> = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestPayment,
                    Some(maturity.clone()),
                    model.cycleOfInterestPayment.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ).into_iter().collect();

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

                interest_events.push(capitalization_end.clone());

                for e in &mut interest_events {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime.clone(){
                        e.eventType = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
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
                model.endOfMonthConvention.clone().unwrap(),
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
                .iter_mut()
                .find(|e| e.eventTime.clone().unwrap().clone() > status_event.eventTime.clone().unwrap())
            {
                let mut fixed_event = fixed_event.clone();
                fixed_event.fstate = Some(Rc::new(STF_RRF_LAM));
                fixed_event.eventType = RRF;
                rate_reset_events.insert(fixed_event);
            }
        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycleOfFee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    Some(maturity.clone()),
                    Some(cycle_of_fee.clone()),
                    model.endOfMonthConvention.clone().unwrap(),
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
        if let scaling_effect= &model.scalingEffect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfScalingIndex,
                        Some(maturity.clone()),
                        model.cycleOfScalingIndex.clone(),
                        model.endOfMonthConvention.clone().unwrap(),
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
        if model.interestCalculationBase == Some(InterestCalculationBase::NTL(NTL)) {
            let icb_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestCalculationBase,
                    Some(maturity.clone()),
                    model.cycleOfInterestCalculationBase.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
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
        // let to_date = to.unwrap_or(maturity); // A CHECKER
        let to_date = to;
        let post_date = EventFactory::create_event(
            Some(to_date.clone()),
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
        if let Some(maturity) = &model.maturityDate {
            return maturity.as_ref().clone();
        }

        let last_event: IsoDatetime;
        let remaining_periods: i32;
        let cycle_anchor_date = model.cycleAnchorDateOfPrincipalRedemption.clone().unwrap();
        let status_date = model.statusDate.clone().unwrap();
        let cycle = model.cycleOfPrincipalRedemption.clone().unwrap();

        if cycle_anchor_date < status_date {
            let mut previous_events = ScheduleFactory::create_schedule(
                Some(cycle_anchor_date.clone()),
                Some(status_date.clone()),
                Some(cycle.clone()),
                model.endOfMonthConvention.clone().unwrap(),
                false,
            );

            let cycle_period = CycleUtils::parse_period(&cycle.clone()).unwrap();

            previous_events.retain(|d| {
                d >= &(status_date + cycle_period.clone())
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
        let cycle_period = &model.cycleOfPrincipalRedemption.clone().unwrap();
        let adjuster = EndOfMonthConvention::new(
            model.endOfMonthConvention.clone().unwrap(),
            last_event,
            cycle.clone(),
        ).unwrap();

        adjuster.shift(   last_event + CycleUtils::parse_period(cycle_period).unwrap().multiplied_by(remaining_periods)   )
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

            if model.interestCalculationBase == Some(InterestCalculationBase::NT(NT)) {
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
                Some(RedemptionUtils::redemptionAmount(model, &states));
        } else {
            states.nextPrincipalRedemptionPayment = model.nextPrincipalRedemptionPayment;
        }

        states
    }
}
