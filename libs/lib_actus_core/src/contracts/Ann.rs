use std::{rc::Rc, collections::HashSet, fmt};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use chrono::Days;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_events::events::EventFactory::EventFactory;
use lib_actus_events::events::EventType::EventType;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use lib_actus_terms::terms::grp_contract_identification::ContractType::ContractType;
use lib_actus_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use lib_actus_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::util::RedemptionUtils::RedemptionUtils;

use crate::functions::{
    ann::stf::STF_PRD_ANN::STF_PRD_ANN,
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

use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use lib_actus_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_fees::FeeBasis::FeeBasis;
use lib_actus_terms::terms::grp_fees::FeeRate::FeeRate;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use lib_actus_terms::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use lib_actus_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use lib_actus_terms::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use lib_actus_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use lib_actus_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use lib_actus_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use lib_actus_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use lib_actus_terms::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use lib_actus_terms::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use lib_actus_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use lib_actus_terms::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use lib_actus_terms::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use lib_actus_terms::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use lib_actus_terms::terms::grp_optionality::PenaltyRate::PenaltyRate;
use lib_actus_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use lib_actus_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use lib_actus_terms::terms::grp_reset_rate::LifeCap::LifeCap;
use lib_actus_terms::terms::grp_reset_rate::LifeFloor::LifeFloor;
use lib_actus_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::NextResetRate::NextResetRate;
use lib_actus_terms::terms::grp_reset_rate::PeriodCap::PeriodCap;
use lib_actus_terms::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use lib_actus_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use lib_actus_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use lib_actus_types::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct ANN {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for ANN {
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
        let calendar = Calendar::provide_rc(sm, "calendar");
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };

        // Champs qui dépendent d'autres champs
        let cycle_of_fee = CycleOfFee::provide_from_input_dict (sm, "cycleOfFee");
        let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfFee::from_str(&a).ok()
        } else {
            CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
        };

        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict (sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };

        let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment");
        let cycle_point_of_rate_reset =
            if let Some(point) = &cycle_point_of_interest_payment {
                if point.to_string() == "B" {
                    CyclePointOfRateReset::from_str("E").ok()
                } else {
                    CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset")
                }
            } else {
                None
            };


        let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex");
        let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfScalingIndex::from_str(&a).ok()
        } else {
            CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex")
        };


        let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict (sm, "cycleOfOptionality");
        let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfOptionality::from_str(&a).ok()
        } else {
            CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
        };



        let cycle_anchor_date_of_rate_reset = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_reset.is_some() {
            cycle_anchor_date_of_rate_reset
        } else {
            if cycle_of_rate_reset.is_none() {
                None
            }
            else {
                let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
                CycleAnchorDateOfRateReset::new(a).ok()
            }

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

        let cycle_of_principal_redemption = CycleOfPrincipalRedemption::provide_from_input_dict (sm, "cycleOfPrincipalRedemption");

        let b = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
        let z = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption");
        let cycle_anchor_date_of_principal_redemption = if z.is_some() {
            z
        }
        else { CycleAnchorDateOfPrincipalRedemption::new(b.unwrap().value()).ok() };

        // let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
        //     let a = initial_exchange_date.value().to_string();
        //     CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
        // } else {
        //     CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
        // };

        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };

        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let w = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
        let accrued_interest = if w.is_some() {
            w
        }
        else {
            AccruedInterest::new(0.0).ok()
        };


        let w = FeeRate::provide_from_input_dict(sm, "feeRate");
        let fee_rate = if w.is_some() {
            w
        }
        else {
            FeeRate::new(0.0).ok()
        };


        let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
        let period_cap = if w.is_some() {
            w
        }
        else {
            PeriodCap::new(f64::INFINITY).ok()
        };

        let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
        let period_floor = if w.is_some() {
            w
        }
        else {
            PeriodFloor::new(f64::NEG_INFINITY).ok()
        };


        let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
        let life_cap = if w.is_some() {
            w
        }
        else {
            LifeCap::new(f64::INFINITY).ok()
        };

        let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
        let life_floor = if w.is_some() {
            w
        }
        else {
            LifeFloor::new(f64::NEG_INFINITY).ok()
        };
        let ct = ContractTerms {
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
            cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
            fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
            fee_rate: fee_rate,
            fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            accrued_interest: accrued_interest,
            capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
            scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
            notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
            interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
            cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
            cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
            scaling_effect: ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
            cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
            cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
            penalty_type: PenaltyType::provide_from_input_dict(sm, "penaltyType"),
            penalty_rate: PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
            object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            life_cap: life_cap,
            life_floor: life_floor,
            period_cap: period_cap,
            period_floor: period_floor,
            fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            maturity_date: maturity_date,
            cycle_anchor_date_of_interest_calculation_base: cycle_anchor_date_of_interest_calculation_base,
            cycle_of_interest_calculation_base: CycleOfInterestCalculationBase::provide_from_input_dict(sm, "cycleOfInterestCalculationBase"),
            interest_calculation_base: interest_calculation_base,
            interest_calculation_base_amount: InterestCalculationBaseAmount::provide_from_input_dict(sm, "interestCalculationBaseAmount"),
            cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
            cycle_of_principal_redemption: cycle_of_principal_redemption,
            next_principal_redemption_payment: NextPrincipalRedemptionPayment::provide_from_input_dict(sm, "nextPrincipalRedemptionPayment"),
            amortization_date: AmortizationDate::provide_from_input_dict(sm, "amortizationDate"),
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
        let mut events : Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new(); // A revoir
        let model = &self.contract_terms;
        let maturity = Self::maturity(model);
        let model = &self.contract_terms;
        // Initial exchange (IED)
        // ::<InitialExchangeDate, InitialExchangeDate>
        let e : ContractEvent<InitialExchangeDate, InitialExchangeDate>= EventFactory::create_event(
            &model.initial_exchange_date.clone(),
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption (MD)
        // ::<MaturityDate, MaturityDate>
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption schedule (PR)
        let mut stf: Rc<dyn TraitStateTransitionFunction>;
        if model.interest_calculation_base.clone().unwrap() != InterestCalculationBase::NT(NT) {
            stf = Rc::new(STF_PR_NAM)
        } else {
            stf = Rc::new(STF_PR2_NAM)
        };
        
        let a = &ScheduleFactory::<
            CycleAnchorDateOfPrincipalRedemption,
            MaturityDate,
            CycleOfPrincipalRedemption,
            IsoDatetime
        >::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption,
            &Some(maturity.clone()),
            &model.cycle_of_principal_redemption,
            &model.end_of_month_convention.clone(),
            Some(false),
        );
        let es = EventFactory::create_events(
            a,
            &EventType::PR,
            &model.currency,
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            &model.business_day_adjuster.clone(),
            &model.contract_id,
        );
        events.extend(es);

        // Initial principal redemption fixing event (PRF)
        if model.next_principal_redemption_payment.is_none() {
            let e: ContractEvent<CycleAnchorDateOfPrincipalRedemption,
                CycleAnchorDateOfPrincipalRedemption> = EventFactory::create_event(
                &CycleAnchorDateOfPrincipalRedemption::new((model.cycle_anchor_date_of_principal_redemption.clone().map(|d|
                    d.value() - Days::new(1))).unwrap()).ok(),
                &EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRD_ANN)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Fees (FP)
        if model.cycle_of_fee.is_some() {
            events.extend(EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &model.cycle_of_fee,
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            ));
        }

        // Purchase (PRD)
        if let Some(purchase_date) = model.purchase_date.clone() {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Interest payment related events (IP)
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        if model.nominal_interest_rate.is_some() &&
            (model.cycle_of_interest_payment.is_some() ||
                model.cycle_anchor_date_of_interest_payment.is_some()) {
            let mut interest_events = EventFactory::create_events(
                &ScheduleFactory::<CycleAnchorDateOfInterestPayment,
                MaturityDate,
                CycleOfInterestPayment,
                IsoDatetime>::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment,
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );

            if model.cycle_anchor_date_of_interest_payment.clone().unwrap().value()
                != model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value()
                    || model.cycle_of_interest_payment.clone().unwrap().value()
                != model.cycle_of_principal_redemption.clone().unwrap().value() {

                //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
                let prcl = model.cycle_of_principal_redemption.clone().unwrap().value().extract_period().unwrap();
                let pranxm = model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value() - prcl;
                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time.clone().unwrap() >= pranxm));

                let ipanxm = EventFactory::create_event(
                    &Some(pranxm),
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.insert(ipanxm);

                let s = ScheduleFactory::<
                CycleAnchorDateOfPrincipalRedemption,
                    MaturityDate,
                    CycleOfPrincipalRedemption,
                    IsoDatetime
                >::create_schedule(
                    &model.cycle_anchor_date_of_principal_redemption,
                    &Some(maturity.clone()),
                    &model.cycle_of_principal_redemption,
                    &model.end_of_month_convention,
                    Some(true),
                );

                let es = EventFactory::create_events(
                    &s,
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.extend(es);
            }

            if let Some(capitalization_end_date) = model.capitalization_end_date.clone() {
                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time));
                interest_events.insert(capitalization_end.to_iso_datetime_event());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.extend(interest_events);
        }
        else if model.capitalization_end_date.is_some() {
            let e: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event(
                &model.capitalization_end_date.clone(),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

        }
        else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {

            let s = ScheduleFactory::<
                CycleAnchorDateOfPrincipalRedemption,
                MaturityDate,
                CycleOfPrincipalRedemption,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption,
                &Some(maturity.clone()),
                &model.cycle_of_principal_redemption,
                &model.end_of_month_convention,
                Some(true),
            );
            let interest_events = EventFactory::create_events(
                &s,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(interest_events);
        }

        // Interest calculation base (IPCB)
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if interest_calculation_base.clone() == InterestCalculationBase::NTL(NTL) {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_calculation_base.clone(),
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_calculation_base.clone(),
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s,
                    &EventType::IPCB,
                    &model.currency,
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    &model.clone().business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Rate reset events (RR)
        let s = ScheduleFactory::<CycleAnchorDateOfRateReset,
            MaturityDate,
            CycleOfRateReset,
            IsoDatetime
        >::create_schedule(
            &model.cycle_anchor_date_of_rate_reset,
            &Some(maturity.clone()),
            &model.cycle_of_rate_reset,
            &model.end_of_month_convention,
            Some(false),
        );
        let mut rate_reset_events = EventFactory::create_events(
            &s,
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            &model.clone().business_day_adjuster,
            &model.contract_id,
        );
        // adapt fixed rate reset event
        if let Some(next_reset_rate) = model.next_reset_rate.clone() {
            let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &model.status_date.clone(),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut fixed_eventa =
                rate_reset_events.clone().iter()
                    .find(|e| e > &&status_event.to_iso_datetime_event()).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM));
            fixed_eventa.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());


        }
        // add all rate reset events
        events.extend(rate_reset_events.clone());

        // add all rate reset events
        let prf_schedule: HashSet<_> = rate_reset_events.clone().iter()
            .map(|e| e.event_time.unwrap()).collect();
        if !prf_schedule.is_empty() {
            let es = EventFactory::create_events(
                &prf_schedule,
                &EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRD_ANN)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(es);
        }

        // scaling (if specified)
        if let Some(scaling_effect) = &model.scaling_effect {
            if scaling_effect.to_string().contains('I') || scaling_effect.to_string().contains('N') {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_scaling_index.clone(),
                    &Some(maturity.clone()),
                    &model.cycle_of_scaling_index,
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s,
                    &EventType::SC,
                    &model.currency,
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Termination event (TD)
        if let Some(termination_date) = model.termination_date.clone() {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e <= &termination.to_iso_datetime_event());
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e >= &status_event.to_iso_datetime_event());

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e <= &to_event);

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

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        // let model = &self.contract_terms;
        let _maturity = &self.contract_terms.maturity_date.clone();
        self.init_state_space(_maturity);
        let events = &mut self.contract_events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        if let Some(purchase_date) = &self.contract_terms.purchase_date.clone() {
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            events.retain(|e|
                !(e.event_type != EventType::AD && e.compare_to(&purchase_event.to_iso_datetime_event()) == -1) );
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
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
                &model.cycle_of_interest_payment,
                &model.end_of_month_convention,
                Some(true),
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

impl ANN {
    fn maturity(model: &ContractTerms) -> MaturityDate {
        if let Some(maturity_date) = model.maturity_date.clone() {
            let a = maturity_date.clone().deref().clone();
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

        let last_event = if pranx.value() >= t0.value() {
            pranx.value()
        } else if ied.value() + prcl.clone() > t0.value() {
            ied.value() + prcl.clone()
        } else {
            let mut previous_events: Vec<IsoDatetime> = ScheduleFactory::
            <CycleAnchorDateOfPrincipalRedemption,
                StatusDate,
                CycleOfPrincipalRedemption,
                IsoDatetime>::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption,
                &Some(t0.clone()),
                &model.cycle_of_principal_redemption,
                &model.end_of_month_convention,
                Some(false)
            ).into_iter().collect();

            previous_events.retain(|&d| d > t0.value());
            previous_events.sort();
            *previous_events.last().unwrap()
        };

        let time_from_last_event_plus_one_cycle = model.day_count_convention.as_ref().unwrap().day_count_fraction(last_event.value(), last_event + prcl.clone());
        let redemption_per_cycle = model.next_principal_redemption_payment.clone().unwrap().value() - (time_from_last_event_plus_one_cycle * model.nominal_interest_rate.clone().unwrap().value() * model.notional_principal.clone().unwrap().value());
        let remaining_periods = ((model.notional_principal.clone().unwrap().value() / redemption_per_cycle).ceil() - 1.0) as i32;

        MaturityDate::new(model.business_day_adjuster.clone().unwrap()
            .shift_bd( &(last_event.clone() + prcl.multiplied_by(remaining_periods))   )).ok().unwrap()
    }

}

impl fmt::Display for ANN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ANN")
    }
}


