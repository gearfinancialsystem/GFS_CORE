use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
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
use crate::functions::lam::stf::STF_SC_LAM::STF_SC_LAM;
use crate::functions::lax::pof::POF_PI_LAX::POF_PI_LAX;
use crate::functions::lax::pof::POF_PR_LAX::POF_PR_LAX;
use crate::functions::lax::stf::STF_PI_LAX2::STF_PI_LAX2;
use crate::functions::lax::stf::STF_PI_LAX::STF_PI_LAX;
use crate::functions::lax::stf::STF_PR_LAX2::STF_PR_LAX2;
use crate::functions::lax::stf::STF_PR_LAX::STF_PR_LAX;
use crate::functions::lax::stf::STF_RR_LAX::STF_RR_LAX;
use crate::functions::lax::stf::STF_RRF_LAX::STF_RRF_LAX;
use crate::functions::lax::stf::STF_RRY_LAM::STF_RRY_LAM;
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
use crate::terms::grp_contract_identification::contract_types::Lam::LAM;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;


pub struct LAX;

impl LAX {
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model, Some(to.clone()));

        // Initial exchange (IED)
        events.push(EventFactory::create_event(
            model.initial_exchange_date.clone(),
            EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &model.contract_id,
        ));

        // Purchase event (PRD)
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

        // Principal redemption schedule
        if let Some(pr_anchor_dates) = &model.arrayCycleAnchorDateOfPrincipalRedemption {
            let pr_cycle = model.arraycycle_of_principal_redemption.as_ref().map(|cycles| cycles.clone());
            let pr_payments = model.arrayNextPrincipalRedemptionPayment.as_ref().unwrap();
            let pr_inc_dec = model.arrayIncreaseDecrease.as_ref().unwrap();

            for i in 0..pr_anchor_dates.len() {
                let pr_type = if pr_inc_dec[i] == ArrayIncreaseDecrease::DEC(DEC) {
                    EventType::PR
                } else {
                    EventType::PI
                };

                let pr_stf: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                    if pr_type == EventType::PR {

                        Rc::new(STF_PR_LAX::new(pr_payments[i]))
                    } else {
                        Rc::new(STF_PI_LAX::new(pr_payments[i]))
                    }
                } else {
                    if pr_type == EventType::PR {
                        Rc::new(STF_PR_LAX2::new(pr_payments[i]))
                    } else {
                        Rc::new(STF_PI_LAX2::new(pr_payments[i]))
                    }
                };

                let pr_pof: Rc<dyn TraitPayOffFunction> = if pr_type == EventType::PR {
                    Rc::new(POF_PR_LAX::new(pr_payments[i]))
                } else {
                    Rc::new(POF_PI_LAX::new(pr_payments[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    Some(pr_anchor_dates[i].clone()),
                    Some(maturity.clone()),
                    pr_cycle.as_ref().map(|cycles| cycles[i].clone()),
                    model.end_of_month_convention.clone().unwrap(),
                    false,
                );

                let mut pr_events = EventFactory::create_events_with_convention(
                    &schedule,
                    pr_type,
                    &model.currency,
                    Some(pr_pof),
                    Some(pr_stf),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                events.append(&mut pr_events.into_iter().collect());
            }
        }

        // Maturity event (MD)
        events.push(EventFactory::create_event_with_convention(
            Some(maturity.clone()),
            EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        ));

        // Interest payment schedule
        if let Some(ip_anchor_dates) = &model.arrayCycleAnchorDateOfInterestPayment {
            let mut ip_cycle = model.arrayCycleOfInterestPayment.clone().unwrap().iter().map(|s| Some(s.clone())).collect::<Vec<_>>();

            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_array_schedule(
                    ip_anchor_dates.clone(),
                    maturity.clone(),
                    ip_cycle,
                    model.end_of_month_convention.clone().unwrap(),
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            if let Some(capitalization_end_date) = &model.capitalization_end_date {
                let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                    Rc::new(STF_IPCI_LAM)
                } else {
                    Rc::new(STF_IPCI2_LAM)
                };

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
                let mut v: Vec<ContractEvent> = interest_events.clone().into_iter().collect();
                for e in v.iter_mut() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }

                interest_events.insert(capitalization_end);
            }

            events.append(&mut interest_events.into_iter().collect());
        } else if let Some(capitalization_end_date) = &model.capitalization_end_date {
            let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                Rc::new(STF_IPCI_LAM)
            } else {
                Rc::new(STF_IPCI2_LAM)
            };

            events.push(EventFactory::create_event_with_convention(
                Some(capitalization_end_date.clone()),
                EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            ));
        }

        // Rate reset schedule
        if let Some(rr_anchor_dates) = &model.arrayCycleAnchorDateOfRateReset {
            let rr_cycle = model.arrayCycleOfRateReset.as_ref().map(|cycles| cycles.clone());
            let rr_rates = model.arrayRate.as_ref().unwrap();
            let rr_fixed_var = model.arrayFixedVariable.as_ref().unwrap();

            for i in 0..rr_anchor_dates.len() {
                let rr_type = if rr_fixed_var[i] == ArrayFixedVariable::F(F) {
                    EventType::RRF
                } else {
                    EventType::RR
                };

                let rr_stf: Rc<dyn TraitStateTransitionFunction> = if rr_type == EventType::RRF {
                    Rc::new(STF_RRF_LAX::new(rr_rates[i]))
                } else {
                    Rc::new(STF_RR_LAX::new(rr_rates[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    Some(rr_anchor_dates[i].clone()),
                    Some(maturity.clone()),
                    rr_cycle.as_ref().map(|cycles| cycles[i].clone()),
                    model.end_of_month_convention.clone().unwrap(),
                    false,
                );

                let mut rate_reset_events = EventFactory::create_events_with_convention(
                    &schedule,
                    rr_type,
                    &model.currency,
                    Some(Rc::new(POF_RR_PAM)),
                    Some(rr_stf),
                    model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                events.append(&mut rate_reset_events.into_iter().collect());
            }

            if let Some(next_reset_rate) = &model.next_reset_rate {
                let mut rate_reset_events: Vec<ContractEvent> = events.iter()
                    .filter(|e| e.event_type == EventType::RR || e.event_type == EventType::RRF)
                    .cloned()
                    .collect();

                rate_reset_events.sort();

                if let Some(fixed_event) = rate_reset_events.iter_mut()
                    .find(|e| e.event_time > model.status_date) {
                    fixed_event.fstate = Some(Rc::new(STF_RRY_LAM));
                    events.push(fixed_event.clone());
                }
            }
        }

        // Fee schedule
        if let Some(fee_cycle) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_fee,
                    Some(maturity.clone()),
                    Some(fee_cycle.clone()),
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

        // Scaling events
        if let scaling_effect = &model.scaling_effect.clone().unwrap().to_string() {
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

        // Interest calculation base events
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NTL(NTL) {
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
        let status_date = model.status_date;
        let status_event = EventFactory::create_event(
            status_date.clone(),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let maturity = Self::maturity(model, None);
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

    fn maturity(model: &ContractModel, to: Option<IsoDatetime>) -> IsoDatetime {
        if let Some(maturity_date) = &model.maturity_date {
            return maturity_date.clone().as_ref().clone();
        }

        //let day_counter = model.day_count_convention.as_ref().unwrap();
        let time_adjuster = model.business_day_adjuster.as_ref().unwrap();
        let notional_principal = model.notional_principal.unwrap();
        let pr_anchor_dates = model.arrayCycleAnchorDateOfPrincipalRedemption.as_ref().unwrap();
        let pr_inc_dec: Vec<i32> = model.arrayIncreaseDecrease.as_ref().unwrap().iter().map(|s| if s.clone() == ArrayIncreaseDecrease::INC(INC) { 1 } else { -1 }).collect();
        let pr_payments = model.arrayNextPrincipalRedemptionPayment.as_ref().unwrap();

        if model.arraycycle_of_principal_redemption.is_none() {
            return pr_anchor_dates.last().unwrap().clone();
        }

        let pr_cycle = model.arraycycle_of_principal_redemption.as_ref().unwrap();
        let mut t = model.status_date.clone();
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
                    model.end_of_month_convention.clone().unwrap(),
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
                    t = Some(pr_anchor_dates[index + 1].clone());

                    for _ in 0..no_of_pr_events - 1 {
                        t = Some(t.clone().unwrap()  + CycleUtils::parse_period(&pr_cycle[index + 1]).unwrap());
                    }

                    sum += no_of_pr_events as f64 * pr_inc_dec[index + 1] as f64 * pr_payments[index + 1];
                    break;
                } else {
                    index += 1;

                    for _ in 0..no_of_pr_events {
                        t = Some(t.unwrap() + CycleUtils::parse_period(&pr_cycle[index - 1]).unwrap());
                    }
                }
            }
        } else {
            let no_of_pr_events = (notional_principal / pr_payments[0]).ceil() as usize;
            t = Some(pr_anchor_dates[0].clone());

            for _ in 0..no_of_pr_events - 1 {
                t = Some(t.unwrap() + CycleUtils::parse_period(&pr_cycle[0]).unwrap());
            }
        }

        time_adjuster.shift_bd(&t.unwrap())
    }

    fn init_state_space(model: &ContractModel, maturity: IsoDatetime) -> StateSpace {
        let mut states = StateSpace::default();

        states.status_date = model.status_date.clone();
        states.notional_scaling_multiplier = Some(1.0);
        states.interest_scaling_multiplier = Some(1.0);

        if model.initial_exchange_date.unwrap() > model.status_date.clone().unwrap() {
            states.notional_principal = Some(0.0);
            states.nominal_interest_rate = Some(0.0);
            states.interest_calculation_base_amount = Some(0.0);
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = Some(role_sign * model.notional_principal.unwrap());
            states.nominal_interest_rate = Some(model.nominal_interest_rate.unwrap());
            states.accrued_interest = Some(role_sign * model.accrued_interest.unwrap_or(0.0));
            states.fee_accrued = Some(model.fee_accrued.unwrap_or(0.0));

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = states.notional_principal;
            } else {
                states.interest_calculation_base_amount = Some(role_sign * model.interest_calculation_baseAmount.unwrap_or(0.0));
            }
        }

        states
    }
}
impl fmt::Display for LAX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LAX")
    }
}