use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;

use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};

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
use crate::functions::lam::stf::STF_PRD_LAM::STF_PRD_LAM;
use crate::functions::lam::stf::STF_RR_LAM::STF_RR_LAM;
use crate::functions::lam::stf::STF_RRF_LAM::STF_RRF_LAM;
use crate::functions::lam::stf::STF_SC_LAM::STF_SC_LAM;
use crate::functions::nam::pof::POF_PR_NAM::POF_PR_NAM;
use crate::functions::nam::stf::STF_PR2_NAM::STF_PR2_NAM;
use crate::functions::nam::stf::STF_PR_NAM::STF_PR_NAM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_SC_PAM::POF_SC_PAM;
use crate::functions::pam::stf::STF_IP_PAM::STF_IP_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;
pub struct NAM;

impl NAM {
    pub fn schedule(
        _to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Initial exchange
        events.push(EventFactory::create_event(
            model.initialExchangeDate.clone(),
            EventType::IED,
            model.currency.as_ref(),
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            model.contractID.as_ref(),
        ));

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory::create_schedule(
            model.cycleAnchorDateOfPrincipalRedemption.clone(),
            Some(maturity.clone()),
            model.cycleOfPrincipalRedemption.clone(),
            model.endOfMonthConvention.clone().unwrap(),
            false,
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase != Some(InterestCalculationBase::NT(NT)) {
            Rc::new(STF_PR_NAM)
        } else {
            Rc::new(STF_PR2_NAM)
        };

        // Regular principal redemption events
        let mut pr_events = EventFactory::create_events_with_convention(
            &pr_schedule,
            EventType::PR,
            model.currency.as_ref(),
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
        );

        events.append(&mut pr_events.into_iter().collect());

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
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycleOfInterestPayment.is_some() || model.cycleAnchorDateOfInterestPayment.is_some() {
            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestPayment.clone(),
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
            );

            // Check if the cycle anchor dates and cycle periods for interest payments and principal payments are different
            if model.cycleAnchorDateOfInterestPayment != model.cycleAnchorDateOfPrincipalRedemption
                || model.cycleOfInterestPayment != model.cycleOfPrincipalRedemption {
                // Calculate the next principal redemption date by subtracting the cycle period from the anchor date
                let prcl = CycleUtils::parse_period(&model.cycleOfPrincipalRedemption.clone().unwrap());
                let pranxm = model.cycleAnchorDateOfPrincipalRedemption.unwrap() - prcl.unwrap();

                // Remove any interest payment events that occur on or after the calculated next principal redemption date
                interest_events.retain(|e| {
                    !(e.eventType == EventType::IP && (e.eventTime.clone().unwrap() > pranxm || e.eventTime.clone().unwrap() == pranxm))
                });

                // Create a new interest payment event at the adjusted principal redemption date
                let ipanxm = EventFactory::create_event_with_convention(
                    Some(pranxm),
                    EventType::IP,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                interest_events.insert(ipanxm);

                // Generate new interest payment events based on the updated principal redemption schedule
                let new_interest_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfPrincipalRedemption.clone(),
                        Some(maturity.clone()),
                        model.cycleOfPrincipalRedemption.clone(),
                        model.endOfMonthConvention.clone().unwrap(),
                        true,
                    ),
                    EventType::IP,
                    model.currency.as_ref(),
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.businessDayAdjuster.as_ref().unwrap(),
                    model.contractID.as_ref(),
                );

                interest_events.extend(new_interest_events);
            }

            // Adapt if interest capitalization set
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
                    !(e.eventType == EventType::IP && e.eventTime == capitalization_end.eventTime)
                });

                interest_events.insert(capitalization_end.clone());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime.clone() {
                        e.eventType = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events.clone().into_iter().collect());
        } else if model.capitalizationEndDate.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.capitalizationEndDate.clone(),
                EventType::IPCI,
                model.currency.as_ref(),
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        } else if model.cycleOfInterestPayment.is_none() && model.cycleAnchorDateOfInterestPayment.is_none() {
            // If no IPCL or IPANX is provided, IP events are set to PR cycle
            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfPrincipalRedemption.clone(),
                    Some(maturity.clone()),
                    model.cycleOfPrincipalRedemption.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
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
            Some(Rc::new(STF_RR_LAM)),
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
            sorted_events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

            let mut fixed_eventa = sorted_events.iter_mut().find(|e| e.eventTime.clone().unwrap() > status_event.eventTime.clone().unwrap()).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM)); // Ensure the field name is correct
            fixed_eventa.eventType = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());

            // if let Some(mut fixed_event) = sorted_events.iter().find(|&e| e.eventTime > status_event.eventTime) {
            //     let mut fixed_event = fixed_event.clone(); // Clone the event to get an owned value
            //     fixed_eventxfstate = Some(Rc::new(STF_RRF_LAM)); // Ensure the field name is correct
            //     fixed_eventxeventType = EventType::RRF;
            //     rate_reset_events.insert(fixed_eventx.clone()); // Use push to add to the vector
            // }


        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee.clone(),
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
        if let scaling_effect = &model.scalingEffect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycleAnchorDateOfScalingIndex.clone(),
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
                    model.cycleAnchorDateOfInterestCalculationBase.clone(),
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
            model.statusDate.clone(),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

        // Remove all post to-date events
        let to_date = maturity.clone(); //to.unwrap_or(maturity);
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

    fn maturity(model: &ContractModel) -> IsoDatetime {
        if let maturity = model.maturityDate.clone().unwrap() {
            return *maturity.clone();
        }

        let t0 = model.statusDate.unwrap();
        let pranx = model.cycleAnchorDateOfPrincipalRedemption.unwrap();
        let ied = model.initialExchangeDate.unwrap();
        let prcl = CycleUtils::parse_period(&model.cycleOfPrincipalRedemption.clone().unwrap()).unwrap();
        let last_event: IsoDatetime;

        if pranx >= t0 || pranx == t0 {
            last_event = pranx;
        } else if (ied + prcl.clone()) > t0 || (ied + prcl.clone()) == t0 {
            last_event = ied + prcl.clone();
        } else {
            let mut previous_events = ScheduleFactory::create_schedule_end_time_true(
                model.cycleAnchorDateOfPrincipalRedemption.clone(),
                model.statusDate.clone(),
                model.cycleOfPrincipalRedemption.clone(),
                model.endOfMonthConvention.clone().unwrap(),
            );

            previous_events.retain(|d| d.clone() < t0);
            previous_events.remove(&t0);

            let mut prev_events_list: Vec<_> = previous_events.into_iter().collect();
            prev_events_list.sort();

            last_event = prev_events_list.last().unwrap().clone();
        }

        let time_from_last_event_plus_one_cycle = model.dayCountConvention.as_ref().unwrap().day_count_fraction(
            last_event,
            last_event + prcl.clone(),
        );

        let redemption_per_cycle = model.nextPrincipalRedemptionPayment.unwrap()
            - (time_from_last_event_plus_one_cycle * model.nominalInterestRate.unwrap() * model.notional_principal.unwrap());

        let remaining_periods = ((model.notional_principal.unwrap() / redemption_per_cycle).ceil() - 1.0) as i32;

        model.businessDayAdjuster.as_ref().unwrap().shift_bd(
            &(last_event + prcl.multiplied_by(remaining_periods)),
        )
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notionalScalingMultiplier = model.notionalScalingMultiplier;
        states.interestScalingMultiplier = model.interestScalingMultiplier;
        states.contractPerformance = model.contractPerformance;
        states.statusDate = model.statusDate;
        states.nextPrincipalRedemptionPayment = model.nextPrincipalRedemptionPayment;

        if model.initialExchangeDate.unwrap() > model.statusDate.unwrap() {
            states.notionalPrincipal = Some(0.0);
            states.nominalInterestRate = Some(0.0);
            states.interestCalculationBaseAmount = Some(0.0);
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notionalPrincipal = Some(role_sign * model.notional_principal.unwrap());
            states.nominalInterestRate = model.nominalInterestRate;

            if model.interestCalculationBase == Some(InterestCalculationBase::NT(NT)) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                states.interestCalculationBaseAmount = Some(role_sign * model.interestCalculationBaseAmount.unwrap());
            }
        }

        if model.nominalInterestRate.is_none() {
            states.accruedInterest = Some(0.0);
        } else if model.accruedInterest.is_some() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.accruedInterest = Some(role_sign * model.accruedInterest.unwrap());
        } else {
            let day_counter = model.dayCountConvention.as_ref().unwrap();
            let time_adjuster = model.businessDayAdjuster.as_ref().unwrap();
            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                model.cycleAnchorDateOfInterestPayment.clone(),
                model.maturityDate.clone().map(|rc| (*rc).clone()),
                model.cycleOfInterestPayment.clone(),
                model.endOfMonthConvention.clone().unwrap(),
                true,
            ).into_iter().collect();

            ip_schedule.sort();
            let date_earlier_than_t0: Vec<IsoDatetime> = ip_schedule.into_iter().filter(|date| date.clone() < states.statusDate.unwrap()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accruedInterest = Some(day_counter.day_count_fraction(
                time_adjuster.shift_sc(t_minus),
                time_adjuster.shift_sc(&states.statusDate.unwrap()),
            ) * states.notionalPrincipal.unwrap() * states.nominalInterestRate.unwrap());
        }

        if model.fee_rate.is_none() {
            states.feeAccrued = Some(0.0);
        } else if model.feeAccrued.is_some() {
            states.feeAccrued = model.feeAccrued;
        }

        states
    }
}
impl fmt::Display for NAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NAM")
    }
}