use std::{error::Error, rc::Rc, collections::HashSet, fmt};
use std::any::Any;
use std::str::FromStr;
use chrono::Days;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::util::RedemptionUtils::RedemptionUtils;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

use crate::functions::{
    ann::stf::STF_PRF_ANN::STF_PRF_ANN,
    lam::pof::{
        POF_IP_LAM::POF_IP_LAM, POF_IPCB_LAM::POF_IPCB_LAM, POF_PRD_LAM::POF_PRD_LAM, POF_TD_LAM::POF_TD_LAM,
    },
    lam::stf::{
        STF_FP_LAM::STF_FP_LAM, STF_IED_LAM::STF_IED_LAM, STF_IPBC_LAM::STF_IPCB_LAM, STF_IPCI2_LAM::STF_IPCI2_LAM, STF_IPCI_LAM::STF_IPCI_LAM,
        STF_MD_LAM::STF_MD_LAM, STF_PRD_LAM::STF_PRD_LAM, STF_RR_LAM::STF_RR_LAM, STF_RRF_LAM::STF_RRF_LAM, STF_SC_LAM::STF_SC_LAM
    },
    nam::pof::POF_PR_NAM::POF_PR_NAM,
    nam::stf::{STF_PR2_NAM::STF_PR2_NAM, STF_PR_NAM::STF_PR_NAM},
    pam::pof::{POF_FP_PAM::POF_FP_PAM, POF_IED_PAM::POF_IED_PAM, POF_IPCI_PAM::POF_IPCI_PAM, POF_MD_PAM::POF_MD_PAM, POF_RR_PAM::POF_RR_PAM, POF_SC_PAM::POF_SC_PAM,},
    pam::stf::{STF_IP_PAM::STF_IP_PAM, STF_TD_PAM::STF_TD_PAM}
};
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

pub struct ANN;

impl ANN {
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events : Vec<dyn Any> = Vec::new(); // A revoir
        let maturity = Self::maturity(model);

        // Initial exchange (IED)
        // ::<InitialExchangeDate, InitialExchangeDate>
        events.push(EventFactory::create_event(
            &model.initial_exchange_date.clone(),
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &model.contract_id,
        ));

        // Principal redemption (MD)
        // ::<MaturityDate, MaturityDate>
        events.push(EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &model.contract_id,
        ));

        // Principal redemption schedule (PR)
        let mut stf: Rc<dyn TraitStateTransitionFunction>;
        if model.interest_calculation_base.clone().unwrap() != InterestCalculationBase::NT(NT) {
            stf = Rc::new(STF_PR_NAM)
        } else {
            stf = Rc::new(STF_PR2_NAM)
        };

        let a = &ScheduleFactory::create_schedule(
            &CycleAnchorDateOfPrincipalRedemption::to_opt_isodatetime(&model.cycle_anchor_date_of_principal_redemption),
            &Some(maturity.clone()),
            &model.cycle_of_principal_redemption,
            &model.end_of_month_convention.clone().unwrap(),
            false,
        );
        events.extend(EventFactory::create_events_with_convention(
            a,
            &EventType::PR,
            &model.currency,
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            &model.clone().business_day_adjuster.unwrap(),
            &model.contract_id,
        ));

        // Initial principal redemption fixing event (PRF)
        if model.next_principal_redemption_payment.is_none() {
            events.push(EventFactory::create_event_with_convention(
                model.cycle_anchor_date_of_principal_redemption.clone().map(|d| d - Days::new(1)),
                &EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRF_ANN)),
                &model.clone().business_day_adjuster.unwrap(),
                &model.contract_id,
            ));
        }

        // Fees (FP)
        if model.cycle_of_fee.is_some() {
            events.extend(EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &model.cycle_of_fee,
                    &model.end_of_month_convention.unwrap(),
                    true,
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                &model.clone().business_day_adjuster.unwrap(),
                &model.contract_id,
            ));
        }

        // Purchase (PRD)
        if let Some(purchase_date) = model.purchase_date.clone() {
            events.push(EventFactory::create_event(
                &Some(purchase_date),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &model.contract_id,
            ));
        }

        // Interest payment related events (IP)
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        if model.nominal_interest_rate.is_some() && (model.cycle_of_interest_payment.is_some() || model.cycle_anchor_date_of_interest_payment.is_some()) {
            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_interest_payment.clone(),
                    Some(maturity.clone()),
                    model.cycle_of_interest_payment.clone(),
                    model.end_of_month_convention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.clone().business_day_adjuster.unwrap(),
                &model.contract_id,
            );

            if model.cycle_anchor_date_of_interest_payment != model.cycle_anchor_date_of_principal_redemption || model.cycle_of_interest_payment != model.cycle_of_principal_redemption {
                let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
                let pranxm = model.cycle_anchor_date_of_principal_redemption.clone().unwrap() - prcl;
                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time.clone().unwrap() >= pranxm));

                let ipanxm = EventFactory::create_event_with_convention(
                    Some(pranxm),
                    EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.clone().business_day_adjuster.unwrap(),
                    &model.contract_id,
                );
                interest_events.insert(ipanxm);

                interest_events.extend(EventFactory::create_events_with_convention(
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
                    &model.clone().business_day_adjuster.unwrap(),
                    &model.contract_id,
                ));
            }

            if let Some(capitalization_end_date) = model.capitalization_end_date.clone() {
                let capitalization_end = EventFactory::create_event_with_convention(
                    Some(capitalization_end_date),
                    EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    &model.clone().business_day_adjuster.unwrap(),
                    &model.contract_id,
                );

                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time));
                interest_events.insert(capitalization_end.clone());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.extend(interest_events);
        } else if model.capitalization_end_date.is_some() {
            events.push(EventFactory::create_event_with_convention(
                model.capitalization_end_date.clone(),
                EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.clone().business_day_adjuster.unwrap(),
                &model.contract_id,
            ));
        } else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {
            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_principal_redemption.clone(),
                    Some(maturity.clone()),
                    model.clone().cycle_of_principal_redemption,
                    model.end_of_month_convention.clone().unwrap(),
                    true,
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster.clone().unwrap(),
                &model.contract_id,
            );
            events.extend(interest_events);
        }

        // Interest calculation base (IPCB)
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if interest_calculation_base.clone() == InterestCalculationBase::NTL(NTL) {
                events.extend(EventFactory::create_events_with_convention(
                    &ScheduleFactory::create_schedule(
                        model.cycle_anchor_date_of_interest_calculation_base.clone(),
                        Some(maturity.clone()),
                        model.clone().cycle_of_interest_calculation_base.clone(),
                        model.end_of_month_convention.clone().unwrap(),
                        false,
                    ),
                    EventType::IPCB,
                    &model.currency,
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    &model.clone().business_day_adjuster.unwrap(),
                    &model.contract_id,
                ));
            }
        }

        // Rate reset events (RR)
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_rate_reset.clone(),
                Some(maturity),
                model.cycle_of_rate_reset.clone(),
                model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            &model.clone().business_day_adjuster.unwrap(),
            &model.contract_id,
        );

        if let Some(next_reset_rate) = model.next_reset_rate.clone() {
            let status_event = EventFactory::create_event(
                model.status_date.clone(),
                EventType::AD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            let mut fixed_eventa =  rate_reset_events.clone().iter().find(|e| e > &&status_event).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM));
            fixed_eventa.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());


        }

        events.extend(rate_reset_events.clone());

        let prf_schedule: HashSet<_> = rate_reset_events.clone().iter().map(|e| e.event_time.unwrap()).collect();
        if !prf_schedule.is_empty() {
            events.extend(EventFactory::create_events_with_convention(
                &prf_schedule,
                EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRF_ANN)),
                &model.clone().business_day_adjuster.unwrap(),
                &model.contract_id,
            ));
        }

        // Scaling events (SC)
        if let Some(scaling_effect) = &model.scaling_effect {
            if scaling_effect.to_string().contains('I') || scaling_effect.to_string().contains('N') {
                events.extend(EventFactory::create_events_with_convention(
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
                    &model.clone().business_day_adjuster.unwrap(),
                    &model.contract_id,
                ));
            }
        }

        // Termination event (TD)
        if let Some(termination_date) = model.termination_date.clone() {
            let termination = EventFactory::create_event(
                Some(termination_date),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                &model.contract_id,
            );

            events.retain(|e| e <= &termination);
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

        events.retain(|e| e >= &status_event);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events;

        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        for event in &mut events {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.as_ref().unwrap(),
                &model.clone().business_day_adjuster.unwrap(),
            );
        }

        if let Some(purchase_date) = model.purchase_date.clone() {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date),
                EventType::PRD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e >= &purchase_event);
        }

        events
    }

    fn maturity(model: &ContractModel) -> MaturityDate {
        if let Some(maturity_date) = model.maturity_date.clone() {
            let a = *(maturity_date.clone());
            return a;
        }

        if let Some(amortization_date) = model.amortization_date.as_ref() {
            let a = amortization_date.clone().value().to_string();
            let b = MaturityDate::from_str(a.as_str()).unwrap();
            return b;
        }

        let t0 = model.status_date.as_ref().unwrap();
        let pranx = model.cycle_anchor_date_of_principal_redemption.as_ref().unwrap();
        let ied = model.initial_exchange_date.as_ref().unwrap();
        let copr = model.cycle_of_principal_redemption.as_ref().unwrap();
        let prcl = copr.clone().value().extract_period().unwrap();

        let last_event = if pranx >= t0 {
            pranx
        } else if ied + prcl.clone() > t0 {
            ied + prcl.clone()
        } else {
            let mut previous_events: Vec<IsoDatetime> = ScheduleFactory::create_schedule_end_time_true(
                &model.cycle_anchor_date_of_principal_redemption,
                &Some(t0),
                &model.clone().cycle_of_principal_redemption,
                &model.clone().end_of_month_convention.unwrap(),
            ).into_iter().collect();

            previous_events.retain(|&d| d > t0.value());
            previous_events.sort();
            *previous_events.last().unwrap()
        };

        let time_from_last_event_plus_one_cycle = model.day_count_convention.as_ref().unwrap().day_count_fraction(last_event.value(), last_event + prcl.clone());
        let redemption_per_cycle = model.next_principal_redemption_payment.clone().unwrap() - (time_from_last_event_plus_one_cycle * model.nominal_interest_rate.clone().unwrap() * model.notional_principal.clone().unwrap());
        let remaining_periods = ((model.notional_principal.clone().unwrap() / redemption_per_cycle).ceil() - 1.0) as i32;

        model.clone().business_day_adjuster.unwrap().shift_bd(&(last_event.clone() + prcl.multiplied_by(remaining_periods)))
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();
        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();
        states.maturity_date = Some(Self::maturity(model));

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok(); //Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();// Some(0.0);
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok(); //Some(0.0);
        } else {
            states.notional_principal = NotionalPrincipal::new(&model.contract_role.clone().unwrap().role_sign() * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = Some(model.nominal_interest_rate.clone().unwrap());

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::from_str(states.notional_principal.clone().unwrap().value().to_string().as_str()).ok();
            } else {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(&model.clone().contract_role.clone().unwrap().role_sign() * model.interest_calculation_base_amount.clone().unwrap().value()).ok();
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = AccruedInterest::new(&model.contract_role.clone().unwrap().role_sign() * model.accrued_interest.clone().unwrap().value()).ok();
        } else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.clone().business_day_adjuster.unwrap();

            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &states.maturity_date.clone(),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention.clone().unwrap(),
                true,
            ).into_iter().collect();

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<_> = ip_schedule.iter().filter(|&&date| date < states.status_date.clone().unwrap().value()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = AccruedInterest::new(day_counter.day_count_fraction(
                time_adjuster.shift_sc(*t_minus),
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            ) * states.notional_principal.clone().unwrap().value() * states.nominal_interest_rate.clone().unwrap().value()).ok();
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }

        if model.next_principal_redemption_payment.is_none() {
            if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
                // Fixed at initial PRF event
            } else {
                states.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(RedemptionUtils::redemptionAmount(model, &states)).ok();
            }
        } else {
            states.next_principal_redemption_payment = model.next_principal_redemption_payment.clone();
        }

        states
    }
}

impl fmt::Display for ANN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ANN")
    }
}


