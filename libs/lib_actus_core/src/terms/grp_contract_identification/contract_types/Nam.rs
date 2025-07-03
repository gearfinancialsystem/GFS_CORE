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
            model.initial_exchange_date.clone(),
            EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &model.contract_id,
        ));

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory::create_schedule(
            model.cycle_anchor_date_of_principal_redemption.clone(),
            Some(maturity.clone()),
            model.cycle_of_principal_redemption.clone(),
            model.end_of_month_convention.clone().unwrap(),
            false,
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base != Some(InterestCalculationBase::NT(NT)) {
            Rc::new(STF_PR_NAM)
        } else {
            Rc::new(STF_PR2_NAM)
        };

        // Regular principal redemption events
        let mut pr_events = EventFactory::create_events_with_convention(
            &pr_schedule,
            EventType::PR,
            &model.currency,
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        events.append(&mut pr_events.into_iter().collect());

        // Maturity event
        events.push(EventFactory::create_event_with_convention(
            Some(maturity.clone()),
            EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        ));

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &model.contract_id,
            ));
        }

        // Choose the right state transition function for IPCI depending on ipcb attributes
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycle_of_interest_payment.is_some() || model.cycle_anchor_date_of_Interest_payment.is_some() {
            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_Interest_payment.clone(),
                    Some(maturity.clone()),
                    model.cycle_of_interest_payment.clone(),
                    model.end_of_month_convention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            // Check if the cycle anchor dates and cycle periods for interest payments and principal payments are different
            if model.cycle_anchor_date_of_Interest_payment != model.cycle_anchor_date_of_principal_redemption
                || model.cycle_of_interest_payment != model.cycle_of_principal_redemption {
                // Calculate the next principal redemption date by subtracting the cycle period from the anchor date
                let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap());
                let pranxm = model.cycle_anchor_date_of_principal_redemption.unwrap() - prcl.unwrap();

                // Remove any interest payment events that occur on or after the calculated next principal redemption date
                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && (e.event_time.clone().unwrap() > pranxm || e.event_time.clone().unwrap() == pranxm))
                });

                // Create a new interest payment event at the adjusted principal redemption date
                let ipanxm = EventFactory::create_event_with_convention(
                    Some(pranxm),
                    EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                interest_events.insert(ipanxm);

                // Generate new interest payment events based on the updated principal redemption schedule
                let new_interest_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycle_anchor_date_of_principal_redemption.clone(),
                        Some(maturity.clone()),
                        model.cycle_of_principal_redemption.clone(),
                        model.end_of_month_convention.clone().unwrap(),
                        true,
                    ),
                    EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                interest_events.extend(new_interest_events);
            }

            // Adapt if interest capitalization set
            if let Some(capitalization_end_date) = &model.capitalization_end_date {
                let capitalization_end = EventFactory::create_event_with_convention(
                    Some(capitalization_end_date.clone()),
                    EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time)
                });

                interest_events.insert(capitalization_end.clone());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time.clone() {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events.clone().into_iter().collect());
        } else if model.capitalization_end_date.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.capitalization_end_date.clone(),
                EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            ));
        } else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_Interest_payment.is_none() {
            // If no IPCL or IPANX is provided, IP events are set to PR cycle
            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_principal_redemption.clone(),
                    Some(maturity.clone()),
                    model.cycle_of_principal_redemption.clone(),
                    model.end_of_month_convention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_rate_reset.clone(),
                Some(maturity.clone()),
                model.cycle_of_rate_reset.clone(),
                model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                model.status_date.clone(),
                EventType::AD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

            let mut fixed_eventa = sorted_events.iter_mut().find(|e| e.event_time.clone().unwrap() > status_event.event_time.clone().unwrap()).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM)); // Ensure the field name is correct
            fixed_eventa.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());

            // if let Some(mut fixed_event) = sorted_events.iter().find(|&e| e.event_time > status_event.event_time) {
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
                    model.cycle_anchor_date_of_fee.clone(),
                    Some(maturity.clone()),
                    Some(cycle_of_fee.clone()),
                    model.end_of_month_convention.clone().unwrap(),
                    true,
                ),
                EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Scaling events (if specified)
        if let scaling_effect = &model.scaling_effect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycle_anchor_date_of_scaling_index.clone(),
                        Some(maturity.clone()),
                        model.cycle_of_scaling_index.clone(),
                        model.end_of_month_convention.clone().unwrap(),
                        false,
                    ),
                    EventType::SC,
                    &model.currency,
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                events.extend(scaling_events);
            }
        }

        // Interest calculation base events (if specified)
        if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            let icb_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfInterestCalculationBase.clone(),
                    Some(maturity.clone()),
                    model.cycleOfInterestCalculationBase.clone(),
                    model.end_of_month_convention.clone().unwrap(),
                    false,
                ),
                EventType::IPCB,
                &model.currency,
                Some(Rc::new(POF_IPCB_LAM)),
                Some(Rc::new(STF_IPCB_LAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(icb_events);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.status_date.clone(),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_date = maturity.clone(); //to.unwrap_or(maturity);
        let post_date = EventFactory::create_event(
            Some(to_date),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= post_date.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                model.day_count_convention.as_ref().unwrap(),
                model.business_day_adjuster.as_ref().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        events
    }

    fn maturity(model: &ContractModel) -> IsoDatetime {
        if let maturity = model.maturity_date.clone().unwrap() {
            return *maturity.clone();
        }

        let t0 = model.status_date.unwrap();
        let pranx = model.cycle_anchor_date_of_principal_redemption.unwrap();
        let ied = model.initial_exchange_date.unwrap();
        let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
        let last_event: IsoDatetime;

        if pranx >= t0 || pranx == t0 {
            last_event = pranx;
        } else if (ied + prcl.clone()) > t0 || (ied + prcl.clone()) == t0 {
            last_event = ied + prcl.clone();
        } else {
            let mut previous_events = ScheduleFactory::create_schedule_end_time_true(
                model.cycle_anchor_date_of_principal_redemption.clone(),
                model.status_date.clone(),
                model.cycle_of_principal_redemption.clone(),
                model.end_of_month_convention.clone().unwrap(),
            );

            previous_events.retain(|d| d.clone() < t0);
            previous_events.remove(&t0);

            let mut prev_events_list: Vec<_> = previous_events.into_iter().collect();
            prev_events_list.sort();

            last_event = prev_events_list.last().unwrap().clone();
        }

        let time_from_last_event_plus_one_cycle = model.day_count_convention.as_ref().unwrap().day_count_fraction(
            last_event,
            last_event + prcl.clone(),
        );

        let redemption_per_cycle = model.next_principal_redemption_payment.unwrap()
            - (time_from_last_event_plus_one_cycle * model.nominal_interest_rate.unwrap() * model.notional_principal.unwrap());

        let remaining_periods = ((model.notional_principal.unwrap() / redemption_per_cycle).ceil() - 1.0) as i32;

        model.business_day_adjuster.as_ref().unwrap().shift_bd(
            &(last_event + prcl.multiplied_by(remaining_periods)),
        )
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier;
        states.interest_scaling_multiplier = model.notional_scaling_multiplier;
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date;
        states.next_principal_redemption_payment = model.next_principal_redemption_payment;

        if model.initial_exchange_date.unwrap() > model.status_date.unwrap() {
            states.notional_principal = Some(0.0);
            states.nominal_interest_rate = Some(0.0);
            states.interest_calculation_base_amount = Some(0.0);
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = Some(role_sign * model.notional_principal.unwrap());
            states.nominal_interest_rate = model.nominal_interest_rate;

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = states.notional_principal;
            } else {
                states.interest_calculation_base_amount = Some(role_sign * model.interest_calculation_baseAmount.unwrap());
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = Some(0.0);
        } else if model.accrued_interest.is_some() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.accrued_interest = Some(role_sign * model.accrued_interest.unwrap());
        } else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();
            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_Interest_payment.clone(),
                model.maturity_date.clone().map(|rc| (*rc).clone()),
                model.cycle_of_interest_payment.clone(),
                model.end_of_month_convention.clone().unwrap(),
                true,
            ).into_iter().collect();

            ip_schedule.sort();
            let date_earlier_than_t0: Vec<IsoDatetime> = ip_schedule.into_iter().filter(|date| date.clone() < states.status_date.unwrap()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = Some(day_counter.day_count_fraction(
                time_adjuster.shift_sc(t_minus),
                time_adjuster.shift_sc(&states.status_date.unwrap()),
            ) * states.notional_principal.unwrap() * states.nominal_interest_rate.unwrap());
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = Some(0.0);
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued;
        }

        states
    }
}
impl fmt::Display for NAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NAM")
    }
}