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
                let pr_type = if pr_inc_dec[i] == ArrayIncreaseDecrease::DEC(DEC) {
                    EventType::PR
                } else {
                    EventType::PI
                };

                let pr_stf: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL(NTL)) {
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
                    model.endOfMonthConvention.clone().unwrap(),
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

                events.append(&mut pr_events.into_iter().collect());
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
            let mut ip_cycle = model.arrayCycleOfInterestPayment.clone().unwrap().iter().map(|s| Some(s.clone())).collect::<Vec<_>>();

            let mut interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_array_schedule(
                    ip_anchor_dates.clone(),
                    maturity.clone(),
                    ip_cycle,
                    model.endOfMonthConvention.clone().unwrap(),
                ),
                EventType::IP,
                model.currency.as_ref(),
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            if let Some(capitalization_end_date) = &model.capitalizationEndDate {
                let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL(NTL)) {
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
                let mut v: Vec<ContractEvent> = interest_events.clone().into_iter().collect();
                for e in v.iter_mut() {
                    if e.eventType == EventType::IP && e.eventTime <= capitalization_end.eventTime {
                        e.eventType = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }

                interest_events.insert(capitalization_end);
            }

            events.append(&mut interest_events.into_iter().collect());
        } else if let Some(capitalization_end_date) = &model.capitalizationEndDate {
            let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interestCalculationBase == Some(InterestCalculationBase::NTL(NTL)) {
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
                    model.endOfMonthConvention.clone().unwrap(),
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

                events.append(&mut rate_reset_events.into_iter().collect());
            }

            if let Some(next_reset_rate) = &model.nextResetRate {
                let mut rate_reset_events: Vec<ContractEvent> = events.iter()
                    .filter(|e| e.eventType == EventType::RR || e.eventType == EventType::RRF)
                    .cloned()
                    .collect();

                rate_reset_events.sort();

                if let Some(fixed_event) = rate_reset_events.iter_mut()
                    .find(|e| e.eventTime > model.statusDate) {
                    fixed_event.fstate = Some(Rc::new(STF_RRY_LAM));
                    events.push(fixed_event.clone());
                }
            }
        }

        // Fee schedule
        if let Some(fee_cycle) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycleAnchorDateOfFee,
                    Some(maturity.clone()),
                    Some(fee_cycle.clone()),
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

        // Scaling events
        if let scaling_effect = &model.scalingEffect.clone().unwrap().to_string() {
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

        // Interest calculation base events
        if let Some(interest_calculation_base) = &model.interestCalculationBase {
            if *interest_calculation_base == InterestCalculationBase::NTL(NTL) {
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
            status_date.clone(),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
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
            return maturity_date.clone().as_ref().clone();
        }

        //let day_counter = model.dayCountConvention.as_ref().unwrap();
        let time_adjuster = model.businessDayAdjuster.as_ref().unwrap();
        let notional_principal = model.notional_principal.unwrap();
        let pr_anchor_dates = model.arrayCycleAnchorDateOfPrincipalRedemption.as_ref().unwrap();
        let pr_inc_dec: Vec<i32> = model.arrayIncreaseDecrease.as_ref().unwrap().iter().map(|s| if s.clone() == ArrayIncreaseDecrease::INC(INC) { 1 } else { -1 }).collect();
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
                    model.endOfMonthConvention.clone().unwrap(),
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

        states.statusDate = model.statusDate.clone();
        states.notionalScalingMultiplier = Some(1.0);
        states.interestScalingMultiplier = Some(1.0);

        if model.initialExchangeDate.unwrap() > model.statusDate.clone().unwrap() {
            states.notionalPrincipal = Some(0.0);
            states.nominalInterestRate = Some(0.0);
            states.interestCalculationBaseAmount = Some(0.0);
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notionalPrincipal = Some(role_sign * model.notional_principal.unwrap());
            states.nominalInterestRate = Some(model.nominalInterestRate.unwrap());
            states.accruedInterest = Some(role_sign * model.accruedInterest.unwrap_or(0.0));
            states.feeAccrued = Some(model.feeAccrued.unwrap_or(0.0));

            if model.interestCalculationBase == Some(InterestCalculationBase::NT(NT)) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                states.interestCalculationBaseAmount = Some(role_sign * model.interestCalculationBaseAmount.unwrap_or(0.0));
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