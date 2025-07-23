use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_events::events::EventFactory::EventFactory;
use lib_actus_events::events::EventType::EventType;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
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
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use lib_actus_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use lib_actus_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_fees::FeeRate::FeeRate;
use lib_actus_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use lib_actus_terms::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use lib_actus_terms::terms::grp_reset_rate::LifeCap::LifeCap;
use lib_actus_terms::terms::grp_reset_rate::LifeFloor::LifeFloor;
use lib_actus_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::PeriodCap::PeriodCap;
use lib_actus_terms::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use lib_actus_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use lib_actus_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use lib_actus_types::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use lib_actus_terms::terms::grp_contract_identification::ContractType::ContractType;
use lib_actus_terms::terms::grp_interest::ArrayCycleAnchorDateOfInterestPayment::ArrayCycleAnchorDateOfInterestPayment;
use lib_actus_terms::terms::grp_interest::ArrayCycleOfInterestPayment::ArrayCycleOfInterestPayment;
use lib_actus_terms::terms::grp_notional_principal::ArrayCycleAnchorDateOfPrincipalRedemption::ArrayCycleAnchorDateOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::ArrayCycleOfPrincipalRedemption::ArrayCycleOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::ArrayIncreaseDecrease::{ArrayIncreaseDecrease, IncreaseDecreaseElement};
use lib_actus_terms::terms::grp_notional_principal::ArrayNextPrincipalRedemptionPayment::ArrayNextPrincipalRedemptionPayment;
use lib_actus_terms::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use lib_actus_terms::terms::grp_notional_principal::increase_decrease::INC::INC;
use lib_actus_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use lib_actus_terms::terms::grp_reset_rate::ArrayCycleAnchorDateOfRateReset::ArrayCycleAnchorDateOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::ArrayCycleOfRateReset::ArrayCycleOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::ArrayFixedVariable::{ArrayFixedVariable, FixedVariableElement};
use lib_actus_terms::terms::grp_reset_rate::ArrayRate::ArrayRate;
use lib_actus_terms::terms::grp_reset_rate::fixed_variable::F::F;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_types::types::IsoCycle::IsoCycle;
use lib_actus_types::types::Value::Value;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct LAX {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for LAX {
    fn new() -> Self {
        Self {
            contract_terms: ContractTerms::default(),
            contract_events: Vec::<ContractEvent<IsoDatetime, IsoDatetime>>::new(),
            contract_risk_factors: None,
            contract_structure: None,
            states_space: StatesSpace::default(),
            result_vec_toggle: false,
            result_vec: None,
        }
    }

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>) {
        // Déclarations simples sans dépendances
        let calendar = Calendar::provide_rc(sm, "calendar");

        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };

        // Champs qui dépendent d'autres champs
        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };

        let cycle_of_interest_calculation_base = CycleOfInterestCalculationBase::provide_from_input_dict (sm, "cycleOfInterestCalculationBase");
        let cycle_anchor_date_of_interest_calculation_base = if cycle_of_interest_calculation_base.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict (sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestCalculationBase::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestCalculationBase::provide_from_input_dict(sm, "cycleAnchorDateOfInterestCalculationBase3")
        };

        let interest_calculation_base_tmp = InterestCalculationBase::provide_from_input_dict(sm, "interestCalculationBase");
        let interest_calculation_base = if interest_calculation_base_tmp.is_none() {
            InterestCalculationBase::new("NT").ok()
        } else {
            interest_calculation_base_tmp
        };

        let mut fee_rate = FeeRate::provide_from_input_dict(sm, "feeRate");
        if fee_rate.is_none() {
            fee_rate = FeeRate::new(0.0).ok();
        }

        let b = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
        let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
            let a = initial_exchange_date.value().to_string();
            CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
        } else {
            CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };

        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let b = eomc.unwrap();
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {
            eomc.unwrap()
        };
        let z = ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment");


        let cm = ContractTerms {
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
            maturity_date: maturity_date,
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            array_cycle_anchor_date_of_principal_redemption: ArrayCycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "arrayCycleAnchorDateOfPrincipalRedemption"),
            array_cycle_of_principal_redemption: ArrayCycleOfPrincipalRedemption::provide_from_input_dict(sm, "arrayCycleOfPrincipalRedemption"),
            array_next_principal_redemption_payment: ArrayNextPrincipalRedemptionPayment::provide_from_input_dict(sm, "arrayNextPrincipalRedemptionPayment"),
            array_increase_decrease: ArrayIncreaseDecrease::provide_from_input_dict(sm, "arrayIncreaseDecrease"),
            array_cycle_anchor_date_of_interest_payment: ArrayCycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "arrayCycleAnchorDateOfInterestPayment"),
            array_cycle_of_interest_payment: ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            array_cycle_anchor_date_of_rate_reset: ArrayCycleAnchorDateOfRateReset::provide_from_input_dict(sm, "arrayCycleAnchorDateOfRateReset"),
            array_cycle_of_rate_reset: ArrayCycleOfRateReset::provide_from_input_dict(sm, "arrayCycleOfRateReset"),
            array_rate: ArrayRate::provide_from_input_dict(sm, "arrayRate"),
            array_fixed_variable: ArrayFixedVariable::provide_from_input_dict(sm, "arrayFixedVariable"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            fee_rate: fee_rate,
            end_of_month_convention: end_of_month_convention,
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
            period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
            life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
            life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
            cycle_anchor_date_of_interest_calculation_base: cycle_anchor_date_of_interest_calculation_base,
            cycle_of_interest_calculation_base: CycleOfInterestCalculationBase::provide_from_input_dict(sm, "cycleOfInterestCalculationBase"),
            interest_calculation_base: interest_calculation_base,
            interest_calculation_base_amount: InterestCalculationBaseAmount::provide_from_input_dict(sm, "interestCalculationBaseAmount"),
            cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
            ..Default::default()
        };


        self.contract_terms = ct;
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = None;
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>) {
        self.contract_structure = None;
    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new());
    }

    fn schedule(&mut self, to: Option<IsoDatetime>) {
        let model = &self.contract_terms;
        let mut events = Vec::new();
        let maturity = Self::maturity(model);


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
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());


        // let z: Vec<CycleAnchorDateOfInterestPayment> = ArrayCycleAnchorDateOfInterestPayment::with_values(
        //     &model.array_cycle_anchor_date_of_interest_payment.clone().unwrap().values().iter().map(
        //     |d| CycleAnchorDateOfInterestPayment::new(d.clone()).ok().expect("er")
        //     ).collect()
        // );


        // Interest payment schedule
        if let Some(ip_anchor_dates) = &model.array_cycle_anchor_date_of_interest_payment {
            let z2: Vec<CycleAnchorDateOfInterestPayment> = {
                let a = ip_anchor_dates.values().iter().map(|d| CycleAnchorDateOfInterestPayment::new(d.clone()).ok().unwrap()
                ).collect();
                a
            };

            //let mut ip_cycle = model.array_cycle_of_interest_payment.clone().unwrap().values().iter().map(|s| s.clone()).collect::<Vec<_>>();

            let ip_cycle: Vec<CycleOfInterestPayment> = {
                let a = model.array_cycle_of_interest_payment.clone().unwrap().values().iter().map(|d| CycleOfInterestPayment::new_with_isocycle(d.clone())
                ).collect();
                a
            };


            let s = ScheduleFactory::<
                CycleAnchorDateOfInterestPayment,
                MaturityDate,
                CycleOfInterestPayment,
                IsoDatetime
            >::create_array_schedule(
                &z2,
                &Some(maturity.clone()),
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
        } 
        else if let Some(capitalization_end_date) = &model.capitalization_end_date {
            let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
                Rc::new(STF_IPCI_LAM)
            } else {
                Rc::new(STF_IPCI2_LAM)
            };

            let e: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event(
                &Some(capitalization_end_date.clone()),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Rate reset schedule
        if let Some(rr_anchor_dates) = &model.array_cycle_anchor_date_of_rate_reset {
            let rr_cycle = model.array_cycle_of_rate_reset.as_ref().map(|cycles| cycles.clone());
            let rr_rates = model.array_rate.as_ref().unwrap();
            let rr_fixed_var = model.array_fixed_variable.as_ref().unwrap();

            for i in 0..rr_anchor_dates.len() {
                let rr_type = if rr_fixed_var.values()[i] == FixedVariableElement::F(F) {
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
                    .find(|e| e.event_time > Some(model.status_date.clone().unwrap().value())) {
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
        if let Some(scaling_effect) = &model.scaling_effect.clone() {
            if scaling_effect.to_string().contains('I') || scaling_effect.to_string().contains('N') {
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
        let to_event: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        // remove all pre-status date events
        events.retain(|e| e.compare_to(&to_event.to_iso_datetime_event()) != -1);


        let to_event : ContractEvent<MaturityDate, MaturityDate>= EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );
        // remove all post to-date events
        events.retain(|e| e.compare_to(&to_event.to_iso_datetime_event()) != 1);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }


        let maturity = Self::maturity(&self.contract_terms);
        self.init_state_space(maturity);
        let events = &mut self.contract_events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        if let Some(purchase_date) = &self.contract_terms.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

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

        self.states_space = states;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &self.states_space,
                &self.contract_terms,
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                },
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            println!("{:?}", a);


            self.contract_events[id_ce].payoff = Some(a);
            // let curr_ce_clone = &curr_ce.clone();
            if self.result_vec_toggle == true {
                if let Some(rv) = &mut self.result_vec {
                    let mut a = ResultSet::new();
                    a.set_result_set(&self.states_space, &self.contract_events[id_ce]);

                    rv.push(a)
                }
            }
        }

        // on peut la retravailler pour etre plus direct et efficace
    }

    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let mut curr_ce= self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &mut self.states_space,
                &self.contract_terms,
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                }
                ,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

}

impl LAX {
    fn maturity(&self) -> MaturityDate {
        if let Some(maturity_date) = &self.contract_terms.maturity_date {
            return maturity_date.clone().as_ref().clone();
        }

        //let day_counter = model.day_count_convention.as_ref().unwrap();
        let time_adjuster = &self.contract_terms.business_day_adjuster.as_ref().unwrap();
        let notional_principal = &self.contract_terms.notional_principal.clone().unwrap();
        let pr_anchor_dates = &self.contract_terms.array_cycle_anchor_date_of_principal_redemption.as_ref().unwrap();
        let pr_inc_dec: Vec<i32> = &self.contract_terms.array_increase_decrease.as_ref().unwrap().values().iter().map(|s| if s.clone() == IncreaseDecreaseElement::INC(INC) { 1 } else { -1 }).collect();
        let pr_payments = &self.contract_terms.array_next_principal_redemption_payment.as_ref().unwrap();

        if &self.contract_terms.array_cycle_of_principal_redemption.is_none() {
            return MaturityDate::new(pr_anchor_dates.values().last().unwrap().clone()).expect("Should return a maturity date");
        }

        let pr_cycle = &self.contract_terms.array_cycle_of_principal_redemption.as_ref().unwrap();
        let mut t = &self.contract_terms.status_date.clone().unwrap().value();
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
                    &self.contract_terms.end_of_month_convention.clone(),
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