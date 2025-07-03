
use std::error::Error;
use std::fmt;
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
use crate::terms::grp_calendar::end_of_month_convention::end_of_month_convention;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;
use crate::util::RedemptionUtils::RedemptionUtils;

pub struct LAM;

impl LAM {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Initial exchange
        events.push(EventFactory::create_event(
            model.initial_exchange_date,
            EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &model.contract_id,
        ));

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory::create_schedule(
            model.cycle_anchor_date_of_principal_redemption,
            Some(maturity.clone()),
            model.cycle_of_principal_redemption.clone(),
            model.end_of_month_convention.clone().unwrap(),
            false,
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base
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
            &model.currency,
            Some(Rc::new(POF_PRD_LAM)),
            Some(stf),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        ).into_iter().collect();

        events.append(&mut pr_events);

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
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base
            == Some(InterestCalculationBase::NTL(NTL))
        {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycle_of_interest_payment.is_some()
            || model.cycle_anchor_date_of_Interest_payment.is_some()
        {
            let mut interest_events: Vec<ContractEvent> = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_Interest_payment,
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
            ).into_iter().collect();

            // Adapt if interest capitalization is set
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
                    !(e.event_type == EventType::IP
                        && e.event_time == capitalization_end.event_time)
                });

                interest_events.push(capitalization_end.clone());

                for e in &mut interest_events {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time.clone(){
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events);
        } else if model.capitalization_end_date.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.capitalization_end_date,
                EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            ));
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_rate_reset,
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
                model.status_date,
                EventType::AD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

            if let Some(fixed_event) = sorted_events
                .iter_mut()
                .find(|e| e.event_time.clone().unwrap().clone() > status_event.event_time.clone().unwrap())
            {
                let mut fixed_event = fixed_event.clone();
                fixed_event.fstate = Some(Rc::new(STF_RRF_LAM));
                fixed_event.event_type = RRF;
                rate_reset_events.insert(fixed_event);
            }
        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_fee,
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
        if let scaling_effect= &model.scaling_effect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycle_anchor_date_of_scaling_index,
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
                    model.cycleAnchorDateOfInterestCalculationBase,
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
            model.status_date,
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        // let to_date = to.unwrap_or(maturity); // A CHECKER
        let to_date = to;
        let post_date = EventFactory::create_event(
            Some(to_date.clone()),
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
        let maturity = Self::maturity(model);
        let mut states = Self::init_state_space(model, maturity);
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
        if let Some(maturity) = &model.maturity_date {
            return maturity.as_ref().clone();
        }

        let last_event: IsoDatetime;
        let remaining_periods: i32;
        let cycle_anchor_date = model.cycle_anchor_date_of_principal_redemption.clone().unwrap();
        let status_date = model.status_date.clone().unwrap();
        let cycle = model.cycle_of_principal_redemption.clone().unwrap();

        if cycle_anchor_date < status_date {
            let mut previous_events = ScheduleFactory::create_schedule(
                Some(cycle_anchor_date.clone()),
                Some(status_date.clone()),
                Some(cycle.clone()),
                model.end_of_month_convention.clone().unwrap(),
                false,
            );

            let cycle_period = CycleUtils::parse_period(&cycle.clone()).unwrap();

            previous_events.retain(|d| {
                d >= &(status_date + cycle_period.clone())
                    && d != &status_date
            });

            last_event = previous_events.iter().next().unwrap().clone();
            remaining_periods = (model.notional_principal.unwrap()
                / model.next_principal_redemption_payment.unwrap())
                .ceil() as i32;
        } else {
            last_event = cycle_anchor_date;
            remaining_periods = ((model.notional_principal.unwrap()
                / model.next_principal_redemption_payment.unwrap())
                .ceil() as i32)
                - 1;
        }
        let cycle_period = &model.cycle_of_principal_redemption.clone().unwrap();
        let adjuster = end_of_month_convention::new(
            model.end_of_month_convention.clone().unwrap(),
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

        states.maturity_date = Some(maturity);

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
                states.interest_calculation_base_amount = Some(
                    role_sign * model.interest_calculation_baseAmount.unwrap(),
                );
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = Some(0.0);
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = model.accrued_interest;
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = Some(0.0);
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued;
        }

        states.notional_scaling_multiplier = model.notional_scaling_multiplier;
        states.interest_scaling_multiplier = model.notional_scaling_multiplier;
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date;

        if model.next_principal_redemption_payment.is_none() {
            states.next_principal_redemption_payment =
                Some(RedemptionUtils::redemptionAmount(model, &states));
        } else {
            states.next_principal_redemption_payment = model.next_principal_redemption_payment;
        }

        states
    }
}
impl fmt::Display for LAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LAM")
    }
}