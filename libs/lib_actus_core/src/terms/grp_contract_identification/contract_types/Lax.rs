use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use chrono::format::Fixed::Internal;
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
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::ArrayCycleAnchorDateOfInterestPayment::ArrayCycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::IncreaseDecreaseElement;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoCycle::IsoCycle;
use crate::types::IsoDatetime::IsoDatetime;


pub struct LAX;

impl TraitContractModel for LAX {
    
    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model, Some(to.clone()));

        // Initial exchange (IED)
        let e: ContractEvent<InitialExchangeDate, InitialExchangeDate> = EventFactory::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Purchase event (PRD)
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Principal redemption schedule
        if let Some(pr_anchor_dates) = &model.array_cycle_anchor_date_of_principal_redemption {
            let pr_cycle = model.array_cycle_of_principal_redemption.as_ref().map(|cycles| cycles.clone());
            let pr_payments = model.array_next_principal_redemption_payment.as_ref().unwrap();
            let pr_inc_dec = model.array_increase_decrease.as_ref().unwrap();

            for i in 0..pr_anchor_dates.len() {
                let pr_type = if pr_inc_dec.values()[i] == IncreaseDecreaseElement::DEC(DEC) {
                    EventType::PR
                } else {
                    EventType::PI
                };

                let pr_stf: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                    if pr_type == EventType::PR {

                        Rc::new(STF_PR_LAX::new(pr_payments.values()[i]))
                    } else {
                        Rc::new(STF_PI_LAX::new(pr_payments.values()[i]))
                    }
                } else {
                    if pr_type == EventType::PR {
                        Rc::new(STF_PR_LAX2::new(pr_payments.values()[i]))
                    } else {
                        Rc::new(STF_PI_LAX2::new(pr_payments.values()[i]))
                    }
                };

                let pr_pof: Rc<dyn TraitPayOffFunction> = if pr_type == EventType::PR {
                    Rc::new(POF_PR_LAX::new(pr_payments.values()[i]))
                } else {
                    Rc::new(POF_PI_LAX::new(pr_payments.values()[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    &Some(pr_anchor_dates.values()[i].clone()),
                    &Some(maturity.clone()),
                    &pr_cycle.as_ref().map(|cycles| cycles.values()[i].clone()),
                    &model.end_of_month_convention.clone(),
                    Some(false),
                );

                let mut pr_events = EventFactory::create_events(
                    &schedule,
                    &pr_type,
                    &model.currency,
                    Some(pr_pof),
                    Some(pr_stf),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.append(&mut pr_events.into_iter().collect());
            }
        }

        // Maturity event (MD)
        let e = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());


        let z: Vec<CycleAnchorDateOfInterestPayment> = *&model.array_cycle_anchor_date_of_interest_payment.clone().unwrap().values().iter().map(
            |d| CycleAnchorDateOfInterestPayment::new(d.clone()).ok().expect("er")
        ).collect();
        // Interest payment schedule
        if let Some(ip_anchor_dates) = &model.array_cycle_anchor_date_of_interest_payment {
            let mut ip_cycle = model.array_cycle_of_interest_payment.clone().unwrap().values().iter().map(|s| Some(s.clone())).collect::<Vec<_>>();

            let s = ScheduleFactory::<
            ArrayCycleAnchorDateOfInterestPayment,
                MaturityDate,
                Vec<IsoCycle>,
                IsoDatetime
            >::create_array_schedule(
                &ip_anchor_dates,
                &maturity,
                &ip_cycle,
                &model.end_of_month_convention,
            );
            let mut interest_events = EventFactory::create_events(
                &s,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            if let Some(capitalization_end_date) = &model.capitalization_end_date {
                let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                    Rc::new(STF_IPCI_LAM)
                } else {
                    Rc::new(STF_IPCI2_LAM)
                };

                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date.clone()),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time)
                });
                let mut v: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = interest_events.clone().into_iter().collect();
                for e in v.iter_mut() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }

                interest_events.insert(capitalization_end.to_iso_datetime_event());
            }

            events.append(&mut interest_events.into_iter().collect());
        } else if let Some(capitalization_end_date) = &model.capitalization_end_date {
            let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                Rc::new(STF_IPCI_LAM)
            } else {
                Rc::new(STF_IPCI2_LAM)
            };

            events.push(EventFactory::create_event(
                &Some(capitalization_end_date.clone()),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.business_day_adjuster,
                &model.contract_id,
            ));
        }

        // Rate reset schedule
        if let Some(rr_anchor_dates) = &model.array_cycle_anchor_date_of_rate_reset {
            let rr_cycle = model.array_cycle_of_rate_reset.as_ref().map(|cycles| cycles.clone());
            let rr_rates = model.array_rate.as_ref().unwrap();
            let rr_fixed_var = model.array_fixed_variable.as_ref().unwrap();

            for i in 0..rr_anchor_dates.len() {
                let rr_type = if rr_fixed_var[i] == ArrayFixedVariable::F(F) {
                    EventType::RRF
                } else {
                    EventType::RR
                };

                let rr_stf: Rc<dyn TraitStateTransitionFunction> = if rr_type == EventType::RRF {
                    Rc::new(STF_RRF_LAX::new(rr_rates.values()[i]))
                } else {
                    Rc::new(STF_RR_LAX::new(rr_rates.values()[i]))
                };

                let schedule = ScheduleFactory::create_schedule(
                    &Some(rr_anchor_dates.values()[i].clone()),
                    &Some(maturity.clone()),
                    &rr_cycle.as_ref().map(|cycles| cycles.values()[i].clone()),
                    &model.end_of_month_convention,
                    Some(false),
                );

                let mut rate_reset_events = EventFactory::create_events(
                    &schedule,
                    &rr_type,
                    &model.currency,
                    Some(Rc::new(POF_RR_PAM)),
                    Some(rr_stf),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.append(&mut rate_reset_events.into_iter().collect());
            }

            if let Some(next_reset_rate) = &model.next_reset_rate {
                let mut rate_reset_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = events.iter()
                    .filter(|e| e.event_type == EventType::RR || e.event_type == EventType::RRF)
                    .cloned()
                    .collect();

                rate_reset_events.sort();

                if let Some(fixed_event) = rate_reset_events.iter_mut()
                    .find(|e| e.event_time > model.status_date.clone()) {
                    fixed_event.fstate = Some(Rc::new(STF_RRY_LAM));
                    events.push(fixed_event.clone());
                }
            }
        }

        // Fee schedule
        if let Some(fee_cycle) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &Some(fee_cycle.clone()),
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Scaling events
        if let scaling_effect = &model.scaling_effect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_scaling_index,
                        &Some(maturity.clone()),
                        &model.cycle_of_scaling_index,
                        &model.end_of_month_convention,
                        Some(false),
                    ),
                    &EventType::SC,
                    &model.currency,
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.extend(scaling_events);
            }
        }

        // Interest calculation base events
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NTL(NTL) {
                let icb_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_interest_calculation_base,
                        &Some(maturity.clone()),
                        &model.cycle_of_interest_calculation_base,
                        &model.end_of_month_convention,
                        Some(false),
                    ),
                    &EventType::IPCB,
                    &model.currency,
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.extend(icb_events);
            }
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_date = model.status_date.clone();
        let status_event = EventFactory::create_event(
            &status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, model: &ContractModel, observer: &RiskFactorModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
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
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        events
    }

    fn init_state_space(model: &ContractModel, maturity: IsoDatetime) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();

        states.status_date = model.status_date.clone();
        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok(); // Some(1.0);
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();//Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();// Some(0.0);
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok();// Some(0.0);
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * {
                if model.accrued_interest.is_none() {
                    AccruedInterest::new(0.0).ok().unwrap().value()
                }
                else {
                    model.accrued_interest.clone().unwrap().value()
                }
            }).ok();
            states.fee_accrued = {
                if states.fee_accrued.is_none() {
                    FeeAccrued::new(0.0).ok()
                }
                else {
                    states.fee_accrued.clone()
                }
            };

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
            } else {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(role_sign * {
                    if model.interest_calculation_base_amount.is_none() {
                        InterestCalculationBaseAmount::new(0.0).ok().unwrap()
                    }
                    else {
                        model.interest_calculation_base_amount.clone().unwrap()
                    }
                        }.value() ).ok();
            }
        }

        Ok(states.clone())
    }

}

impl LAX {
    fn maturity(model: &ContractModel) -> MaturityDate {
        if let Some(maturity_date) = &model.maturity_date {
            return maturity_date.clone().as_ref().clone();
        }

        //let day_counter = model.day_count_convention.as_ref().unwrap();
        let time_adjuster = model.business_day_adjuster.as_ref().unwrap();
        let notional_principal = model.notional_principal.clone().unwrap();
        let pr_anchor_dates = model.array_cycle_anchor_date_of_principal_redemption.as_ref().unwrap();
        let pr_inc_dec: Vec<i32> = model.array_increase_decrease.as_ref().unwrap().values().iter().map(|s| if s.clone() == IncreaseDecreaseElement::INC(INC) { 1 } else { -1 }).collect();
        let pr_payments = model.array_next_principal_redemption_payment.as_ref().unwrap();

        if model.array_cycle_of_principal_redemption.is_none() {
            return MaturityDate::new(pr_anchor_dates.values().last().unwrap().clone()).expect("Should return a maturity date");
        }

        let pr_cycle = model.array_cycle_of_principal_redemption.as_ref().unwrap();
        let mut t = model.status_date.clone().unwrap().value();
        let mut sum = 0.0;

        if pr_cycle.len() > 1 {
            let mut index = 0;
            let mut no_of_pr_events = 0;
            let mut pr_schedule = HashSet::new();

            loop {
                pr_schedule = ScheduleFactory::< // a changer avec les vrai types sous-jacents aux array pour que ce soit plus propre
                    IsoDatetime,
                    IsoDatetime,
                    IsoCycle,
                    IsoDatetime
                >::create_schedule(
                    &Some(pr_anchor_dates.values()[index].clone()),
                    &Some(pr_anchor_dates.values()[index + 1].clone()),
                    &Some(pr_cycle.values()[index].clone()),
                    &model.end_of_month_convention.clone(),
                    Some(false),
                );

                no_of_pr_events = if (pr_schedule.len() as f64 * pr_payments.values()[index] * pr_inc_dec[index] as f64) + notional_principal.value() + sum >= 0.0 {
                    pr_schedule.len()
                } else {
                    ((notional_principal.value() + sum) / pr_payments.values()[index]).ceil() as usize
                };

                sum += no_of_pr_events as f64 * pr_inc_dec[index] as f64 * pr_payments.values()[index];

                if pr_anchor_dates.len() - 2 == index {
                    no_of_pr_events = ((sum + notional_principal.value()) / pr_payments.values()[index + 1]).ceil().abs() as usize;
                    t = pr_anchor_dates.values()[index + 1].clone();

                    for _ in 0..no_of_pr_events - 1 {
                        t = t.clone() + pr_cycle.values()[index + 1].extract_period().clone().unwrap();
                    }


                    sum += no_of_pr_events as f64 * pr_inc_dec[index + 1] as f64 * pr_payments.values()[index + 1];
                    break;
                } else {
                    index += 1;

                    for _ in 0..no_of_pr_events {
                        t = t.clone() + pr_cycle.values()[index - 1].extract_period().clone().unwrap();
                    }
                }
            }
        } else {
            let no_of_pr_events = (notional_principal.value() / pr_payments.values()[0]).ceil() as usize;
            t = pr_anchor_dates.values()[0].clone();

            for _ in 0..no_of_pr_events - 1 {
                t = t.clone() + pr_cycle.values()[0].extract_period().clone().unwrap();
            }
        }

        MaturityDate::new(time_adjuster.shift_bd(&t)).ok().expect("Should return a maturity date")
    }
}



impl fmt::Display for LAX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LAX")
    }
}