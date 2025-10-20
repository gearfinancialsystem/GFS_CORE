use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::Payoff;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::contracts::Lam::LAM;
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
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_calendar::Calendar::Calendar;
use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use gfs_lib_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use gfs_lib_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use gfs_lib_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use gfs_lib_terms::terms::grp_fees::FeeBasis::FeeBasis;
use gfs_lib_terms::terms::grp_fees::FeeRate::FeeRate;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use gfs_lib_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use gfs_lib_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use gfs_lib_terms::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use gfs_lib_terms::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use gfs_lib_terms::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use gfs_lib_terms::terms::grp_optionality::PenaltyRate::PenaltyRate;
use gfs_lib_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use gfs_lib_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use gfs_lib_terms::terms::grp_reset_rate::LifeCap::LifeCap;
use gfs_lib_terms::terms::grp_reset_rate::LifeFloor::LifeFloor;
use gfs_lib_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::NextResetRate::NextResetRate;
use gfs_lib_terms::terms::grp_reset_rate::PeriodCap::PeriodCap;
use gfs_lib_terms::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use gfs_lib_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use gfs_lib_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::{IsoCycleConvertTo, IsoCycleConvertToOption, IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::Value::Value;
use crate::events::EventSequence::EventSequence;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::util::ResultsStruct::TestResult;

pub struct NAM {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
}

impl TraitContractModel for NAM {
    fn new() -> Self {
        Self {
            contract_id: ContractID::new("init".to_string()).expect("init contract ID"),
            contract_terms: ContractTerms::default(),
            risk_factor_external_data: None,
            risk_factor_external_event: None,
            related_contracts: None,
            event_timeline: Vec::new(),
            states_space: StatesSpace::default(),
            status_date: None,
        }
    }

    fn init_contract_terms(&mut self, sm: HashMap<String, Value>) {
        let calendar = Calendar::provide_rc(&sm, "calendar");

        let maturity_date_tmp = MaturityDate::provide_from_input_dict(&sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };


        let cycle_of_fee = CycleOfFee::provide_from_input_dict(&sm, "cycleOfFee");
        let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfFee::from_str(&a).ok()
        } else {
            CycleAnchorDateOfFee::provide_from_input_dict(&sm, "cycleAnchorDateOfFee")
        };

        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestPayment::provide_from_input_dict(&sm, "cycleAnchorDateOfInterestPayment")
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(&sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };

        let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(&sm, "cyclePointOfInterestPayment");
        let cycle_point_of_rate_reset =
            if let Some(point) = &cycle_point_of_interest_payment {
                if point.to_string() == "B" {
                    CyclePointOfRateReset::from_str("E").ok()
                } else {
                    CyclePointOfRateReset::provide_from_input_dict(&sm, "cyclePointOfRateReset")
                }
            } else {
                None
            };


        let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(&sm, "cycleOfScalingIndex");
        let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfScalingIndex::from_str(&a).ok()
        } else {
            CycleAnchorDateOfScalingIndex::provide_from_input_dict(&sm, "cycleAnchorDateOfScalingIndex")
        };


        let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict(&sm, "cycleOfOptionality");
        let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfOptionality::from_str(&a).ok()
        } else {
            CycleAnchorDateOfOptionality::provide_from_input_dict(&sm, "cycleAnchorDateOfOptionality")
        };

        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfRateReset::from_str(&a).ok()
        } else {
            CycleAnchorDateOfRateReset::provide_from_input_dict(&sm,"cycleAnchorDateOfRateReset" )
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                &sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };

        let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(&sm, "creditEventTypeCovered");
        let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
            Some(CreditEventTypeCovered::default())
        } else {
            credit_event_type_covered_tmp
        };
        let eomc = EndOfMonthConvention::provide_from_input_dict(&sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};


        let w = AccruedInterest::provide_from_input_dict(&sm, "accruedInterest");
        let accrued_interest = if w.is_some() {
            w
        }
        else {
            AccruedInterest::new(0.0).ok()
        };


        let w = FeeRate::provide_from_input_dict(&sm, "feeRate");
        let fee_rate = if w.is_some() {
            w
        }
        else {
            FeeRate::new(0.0).ok()
        };


        let w = PeriodCap::provide_from_input_dict(&sm, "periodCap");
        let period_cap = if w.is_some() {
            w
        }
        else {
            PeriodCap::new(f64::INFINITY).ok()
        };

        let w = PeriodFloor::provide_from_input_dict(&sm, "periodFloor");
        let period_floor = if w.is_some() {
            w
        }
        else {
            PeriodFloor::new(f64::NEG_INFINITY).ok()
        };


        let w = LifeCap::provide_from_input_dict(&sm, "lifeCap");
        let life_cap = if w.is_some() {
            w
        }
        else {
            LifeCap::new(f64::INFINITY).ok()
        };

        let w = LifeFloor::provide_from_input_dict(&sm, "lifeFloor");
        let life_floor = if w.is_some() {
            w
        }
        else {
            LifeFloor::new(f64::NEG_INFINITY).ok()
        };

        let mut cycle_anchor_date_of_principal_redemption = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(&sm, "cycleAnchorDateOfPrincipalRedemption");
        cycle_anchor_date_of_principal_redemption = if cycle_anchor_date_of_principal_redemption.is_some() {
            cycle_anchor_date_of_principal_redemption
        }
        else {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate");
            CycleAnchorDateOfPrincipalRedemption::new(a.unwrap().value()).ok()
        };


        let mut scaling_effect = ScalingEffect::provide_from_input_dict(&sm, "scalingEffect");
        scaling_effect = if scaling_effect.is_some() {
            scaling_effect
        }
        else {
            Some(ScalingEffect::new("OOO").unwrap())
        };

        let mut premium_discount_at_ied= PremiumDiscountAtIED::provide_from_input_dict(&sm, "premiumDiscountAtIED");
        premium_discount_at_ied = if premium_discount_at_ied.is_some() {
            premium_discount_at_ied
        }
        else {
            PremiumDiscountAtIED::new(0.0).ok()
        };


        let mut next_principal_redemption_payment= NextPrincipalRedemptionPayment::provide_from_input_dict(&sm, "nextPrincipalRedemptionPayment");
        next_principal_redemption_payment = if next_principal_redemption_payment.is_some() {
            next_principal_redemption_payment
        }
        else {
            None
        };

        let ct = ContractTerms {
            next_principal_redemption_payment: next_principal_redemption_payment,
            premium_discount_at_ied: premium_discount_at_ied,
            cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
            cycle_of_principal_redemption: CycleOfPrincipalRedemption::provide_from_input_dict(&sm, "cycleOfPrincipalRedemption"),
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(&sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(&sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(&sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(&sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(&sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(&sm, "marketObjectCode"),
            cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
            cycle_of_fee: CycleOfFee::provide_from_input_dict(&sm, "cycleOfFee"),
            fee_basis: FeeBasis::provide_from_input_dict(&sm, "feeBasis"),
            fee_rate: fee_rate,
            fee_accrued: FeeAccrued::provide_from_input_dict(&sm, "feeAccrued"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(&sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            accrued_interest: accrued_interest,
            capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(&sm, "capitalizationEndDate"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(&sm, "cyclePointOfInterestPayment"),
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(&sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(&sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate"),
            market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(&sm, "marketObjectCodeOfScalingIndex"),
            scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(&sm, "scalingIndexAtContractDealDate"),
            notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(&sm, "notionalScalingMultiplier"),
            interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(&sm, "interestScalingMultiplier"),
            cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
            cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(&sm, "cycleOfScalingIndex"),
            scaling_effect: scaling_effect,
            cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
            cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(&sm, "cycleOfOptionality"),
            penalty_type: PenaltyType::provide_from_input_dict(&sm, "penaltyType"),
            penalty_rate: PenaltyRate::provide_from_input_dict(&sm, "penaltyRate"),
            object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(&sm, "objectCodeOfPrepaymentModel"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(&sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(&sm, "marketObjectCodeOfRateReset"),
            life_cap: life_cap,
            life_floor: life_floor,
            period_cap: period_cap,
            period_floor: period_floor,
            fixing_period: FixingPeriod::provide_from_input_dict(&sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(&sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(&sm, "rateMultiplier"),
            maturity_date: maturity_date,
            credit_event_type_covered: credit_event_type_covered,
            ..Default::default()
        };


        self.contract_terms = ct;
    }

    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Arc<dyn TraitExternalData>>) {
        self.risk_factor_external_data = risk_factor_external_data;
    }

    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>) {
        self.risk_factor_external_event = risk_factor_external_event;
    }

    fn init_related_contracts(&mut self, _sm: HashMap<String, Value>) {
        self.related_contracts = None;
    }

    fn init_status_date(&mut self) {
        self.status_date = self.contract_terms.status_date;
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(self.contract_terms.notional_scaling_multiplier.clone().unwrap().value()).ok();
        states.contract_performance = model.contract_performance.clone();
        states.status_date = model.status_date.clone();
        states.next_principal_redemption_payment = model.next_principal_redemption_payment.clone();

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();//Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();;
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok();
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
            } else {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(role_sign * &self.contract_terms.interest_calculation_base_amount.clone().unwrap().value()).ok();
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();// Some(0.0);
        } else if model.accrued_interest.is_some() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
        } else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();
            let mut ip_schedule: Vec<PhantomIsoDatetimeW> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.convert_option::<StartTime>(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                &model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();
            let date_earlier_than_t0: Vec<PhantomIsoDatetimeW> = ip_schedule.into_iter().filter(|date| date.clone() < states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = AccruedInterest::new(day_counter.day_count_fraction(
                time_adjuster.shift_sc(t_minus),
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()),
            ) * states.notional_principal.clone().unwrap().value() * states.nominal_interest_rate.clone().unwrap().value()).ok();

        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();// Some(0.0);
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }

        self.states_space = states;
    }
    
    fn init_contract_event_timeline(&mut self, _to : Option<PhantomIsoDatetimeW>) {
        let model = &self.contract_terms;
        let mut events: Vec<ContractEvent> = Vec::new();
        let maturity = Self::maturity(self);

        // Initial exchange
        let e: ContractEvent = EventFactory::create_event(
            &model.initial_exchange_date.convert_option::<ScheduleTime>(),
            &EventType::IED,
            &model.currency,
            Some(PayOffFunction::from_str("POF_IED_PAM")),
            Some(StatesTransitionFunction::from_str("STF_IED_LAM")),
            &None,
            &model.contract_id,
        );
        events.push(e);

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
            &Some(maturity.clone().convert::<EndTime>()),
            &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
            &model.end_of_month_convention,
            Some(false),
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf = if model.interest_calculation_base != Some(InterestCalculationBase::NT(NT)) {
            StatesTransitionFunction::from_str("STF_PR_NAM")
        } else {
            StatesTransitionFunction::from_str("STF_PR2_NAM")
        };

        // Regular principal redemption events
        let pr_events = EventFactory::create_events(
            &pr_schedule.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
            &EventType::PR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_PR_NAM")),
            Some(stf),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(pr_events);

        // Maturity event
        let e: ContractEvent = EventFactory::create_event(
            &Some(maturity.clone().convert::<ScheduleTime>()),
            &EventType::MD,
            &model.currency,
            Some(PayOffFunction::from_str("POF_MD_PAM")),
            Some(StatesTransitionFunction::from_str("STF_MD_LAM")),
            &model.business_day_adjuster,
            &model.contract_id,
        );
        events.push(e);

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_LAM")),
                Some(StatesTransitionFunction::from_str("STF_PRD_LAM")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }

        // Choose the right state transition function for IPCI depending on ipcb attributes
        let stf_ipci = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            StatesTransitionFunction::from_str("STF_IPCI_LAM")
        } else {
            StatesTransitionFunction::from_str("STF_IPCI2_LAM")
        };

        // Interest payment related events
        if model.cycle_of_interest_payment.is_some() || model.cycle_anchor_date_of_interest_payment.is_some() {
            let s = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone().convert_option::<StartTime>(),
                &Some(maturity.clone().convert::<EndTime>()),
                &model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            );
            let mut interest_events = EventFactory::create_events(
                &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_LAM")),
                Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            // Check if the cycle anchor dates and cycle periods for interest payments and principal payments are different
            if model.cycle_anchor_date_of_interest_payment.clone().unwrap().value() != model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value()
                || model.cycle_of_interest_payment.clone().unwrap().value() != model.cycle_of_principal_redemption.clone().unwrap().value() {
                // Calculate the next principal redemption date by subtracting the cycle period from the anchor date
                //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap());
                let prcl = &model.cycle_of_principal_redemption.clone().unwrap().value().extract_period();
                let mut pranxm = PhantomIsoDatetimeW::new(model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value() - prcl.clone().unwrap()).expect("ioj");

                // Remove any interest payment events that occur on or after the calculated next principal redemption date
                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && (e.event_time.clone().unwrap() > pranxm.convert::<EventTime>() || e.event_time.clone().unwrap() == pranxm.convert::<EventTime>()))
                });

                // Create a new interest payment event at the adjusted principal redemption date
                let ipanxm: ContractEvent = EventFactory::create_event(
                    &Some(pranxm.convert::<ScheduleTime>()),
                    &EventType::IP,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IP_LAM")),
                    Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.insert(ipanxm);

                // Generate new interest payment events based on the updated principal redemption schedule
                let new_interest_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_principal_redemption.clone().convert_option::<StartTime>(),
                        &Some(maturity.clone().convert::<EndTime>()),
                        &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                        &model.end_of_month_convention,
                        Some(true),
                    ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                    &EventType::IP,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IP_LAM")),
                    Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.extend(new_interest_events);
            }

            // Adapt if interest capitalization set
            if let Some(capitalization_end_date) = &model.capitalization_end_date {
                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date.clone().convert::<ScheduleTime>()),
                    &EventType::IPCI,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time)
                });

                interest_events.insert(capitalization_end.clone());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time.clone() {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(PayOffFunction::from_str("POF_IPCI_PAM"));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events.clone().into_iter().collect());
        } else if model.capitalization_end_date.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            let e: ContractEvent = EventFactory::create_event(
                &model.capitalization_end_date.clone().convert_option::<ScheduleTime>(),
                &EventType::IPCI,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                Some(stf_ipci),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e);
        } else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {
            // If no IPCL or IPANX is provided, IP events are set to PR cycle
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(true),
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_LAM")),
                Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset.clone().convert_option::<StartTime>(),
                &Some(maturity.clone().convert::<EndTime>()),
                &model.cycle_of_rate_reset.clone().convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(false),
            ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
            &EventType::RR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_RR_PAM")),
            Some(StatesTransitionFunction::from_str("STF_RR_LAM")),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                &model.status_date.clone().convert_option::<ScheduleTime>(),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

            let mut fixed_eventa = sorted_events.iter_mut().find(|e| e.event_time.clone().unwrap() > status_event.event_time.clone().unwrap()).unwrap().clone();
            fixed_eventa.fstate = Some(StatesTransitionFunction::from_str("STF_RRF_LAM")); // Ensure the field name is correct

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
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee.convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &Some(cycle_of_fee.clone().convert::<PhantomIsoCycleW>()),
                    &model.end_of_month_convention.clone(),
                    Some(true),
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::FP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_FP_PAM")),
                Some(StatesTransitionFunction::from_str("STF_FP_LAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Scaling events (if specified)
        if let scaling_effect = &model.scaling_effect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_scaling_index.clone().convert_option::<StartTime>(),
                        &Some(maturity.clone().convert::<EndTime>()),
                        &model.cycle_of_scaling_index.clone().convert_option::<PhantomIsoCycleW>(),
                        &model.end_of_month_convention.clone(),
                        Some(false),
                    ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                    &EventType::SC,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_SC_PAM")),
                    Some(StatesTransitionFunction::from_str("STF_SC_LAM")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.extend(scaling_events);
            }
        }

        // Interest calculation base events (if specified)
        if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            let icb_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_calculation_base.clone().convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_interest_calculation_base.clone().convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention.clone(),
                    Some(false),
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::IPCB,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IPCB_PAM")),
                Some(StatesTransitionFunction::from_str("STF_IPCB_LAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(icb_events);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone().convert::<ScheduleTime>()),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_LAM")),
                Some(StatesTransitionFunction::from_str("STF_TD_PAM")),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            &model.status_date.clone().convert_option::<ScheduleTime>(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_date = maturity.clone(); //to.unwrap_or(maturity);
        let post_date = EventFactory::create_event(
            &Some(to_date.convert::<ScheduleTime>()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= post_date.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        self.event_timeline = events.clone();
    }

    fn set_status_date(&mut self, status_date: Option<StatusDate>) {
        self.status_date = status_date;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            //println!("{:?}\n", a);
            self.event_timeline[id_ce].payoff = Some(Payoff::new(a).expect("ok"));
            //println!("payoff0{:?}\n", self.event_timeline[id_ce].payoff);
        }

        // on peut la retravailler pour etre plus direct et efficace
    }

    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &mut self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

    fn compute_payoff(&mut self) {
        let id_ce: usize = 0;
        self.eval_pof_contract_event(id_ce);
    }

    fn next(&mut self) {
        let id_ce: usize = 0;
        self.eval_pof_contract_event(id_ce);
    }

    fn add_event_to_contract_event_timeline(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        // reflechir a quoi pourrait bien servir reset
        self.contract_terms = ContractTerms::default();
        self.risk_factor_external_data = None;
        self.risk_factor_external_event = None;
        self.related_contracts = None;
        self.event_timeline = Vec::new();
        self.states_space = StatesSpace::default();
        self.status_date = None;
    }
    
    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {

        let events = &mut self.event_timeline.clone();

        events.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));

        let mut result_vec: Vec<TestResult> = Vec::new();

        let mut i: usize = 0;
        for event in events.iter_mut() {
            // let a = event.event_time.expect("fd");
            // let b = EventTime::new(date.expect("fo").value()).expect("ok");

            if date.is_some() {
                if event.event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                    break
                }
            }
            self.eval_pof_contract_event(i);
            //println!("nominalprincipal{:?}", self.states_space.notional_principal);
            //println!("payoff{:?}", self.event_timeline[i].payoff);
            self.eval_stf_contract_event(i);
            // let a = self.event_timeline[i].payoff.clone().expect("ok").to_string();
            if extract_results == true {
                let curr_testresult = TestResult {
                    eventDate: event.event_time.expect("fe").to_string(),
                    eventType: event.event_type.to_string(),
                    payoff: self.event_timeline[i].payoff.clone().expect("ok").to_string(),
                    currency: event.currency.clone().expect("ef").0,
                    notionalPrincipal: self.states_space.notional_principal.clone().expect("ok").to_string(),
                    nominalInterestRate: self.states_space.nominal_interest_rate.clone().expect("ok").to_string(),
                    accruedInterest: self.states_space.accrued_interest.clone().expect("ok").to_string(),
                };
                result_vec.push(curr_testresult)
            }

            i += 1;
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &self.contract_terms.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        self.event_timeline = events.clone();
        if extract_results == false {

            return None;
        }
        else {
            ////////////////////////////////////////////////////////
            // Remove pre-purchase events if purchase date is set //
            ////////////////////////////////////////////////////////
            result_vec.retain(|e| {
                if self.contract_terms.purchase_date.is_some() {
                    let purchase_event: ContractEvent = EventFactory::create_event(
                        &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
                        &EventType::PRD,
                        &self.contract_terms.currency,
                        None,
                        None,
                        &None,
                        &self.contract_terms.contract_id,
                    );
                    let epoch_millis = IsoDatetime::from_str(e.eventDate.as_str()).clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
                    let epoch_offset = epoch_millis + EventSequence::time_offset(&EventType::from_str(e.eventType.as_str()).expect("exist"));
                    EventType::from_str(e.eventType.as_str()).expect("exist") == EventType::AD || epoch_offset as f64 >= purchase_event.epoch_offset.unwrap().value()
                } else { true }
            });
            return Some(Ok(result_vec));
        }

    }

    fn sort_events_timeline(&mut self) {
        self.event_timeline.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));
    }

}

impl NAM {
    fn maturity(&self) -> MaturityDate {
        if let maturity = &self.contract_terms.maturity_date.clone().unwrap() {
            return maturity.as_ref().clone();
        }

        let t0 = &self.contract_terms.status_date.clone().unwrap();
        let pranx = &self.contract_terms.cycle_anchor_date_of_principal_redemption.clone().unwrap();
        let ied = &self.contract_terms.initial_exchange_date.clone().unwrap();
        //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
        let prcl = &self.contract_terms.cycle_of_principal_redemption.clone().unwrap().value().extract_period().unwrap();
        let last_event: PhantomIsoDatetimeW;

        if pranx.value() >= t0.value() || pranx.value() == t0.value() {
            last_event = pranx.convert::<PhantomIsoDatetimeW>();
        } else if (ied.clone().value() + prcl.clone()) > t0.value() || (ied.clone().value() + prcl.clone()) == t0.value() {
            last_event = PhantomIsoDatetimeW::new( ied.clone().value() + prcl.clone()).expect("sf");
        } else {
            let mut previous_events = ScheduleFactory::create_schedule(
                &self.contract_terms.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
                &self.contract_terms.status_date.convert_option::<EndTime>(),
                &self.contract_terms.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                &self.contract_terms.end_of_month_convention,
                Some(true)
            );

            previous_events.retain(|d| d.clone() < t0.convert::<PhantomIsoDatetimeW>());
            previous_events.remove(&t0.convert::<PhantomIsoDatetimeW>());

            let mut prev_events_list: Vec<_> = previous_events.into_iter().collect();
            prev_events_list.sort();

            last_event = prev_events_list.last().unwrap().clone();
        }

        let time_from_last_event_plus_one_cycle = &self.contract_terms.day_count_convention.as_ref().unwrap().day_count_fraction(
            last_event,
            PhantomIsoDatetimeW::new(last_event.value() + prcl.clone()).expect("fs"),
        );

        let redemption_per_cycle = &self.contract_terms.next_principal_redemption_payment.clone().unwrap().value();
        - (time_from_last_event_plus_one_cycle * &self.contract_terms.nominal_interest_rate.clone().unwrap().value()
            * &self.contract_terms.notional_principal.clone().unwrap().value());

        let remaining_periods = ((&self.contract_terms.notional_principal.clone().unwrap().value() / redemption_per_cycle).ceil() - 1.0) as i32;

        let new_mat_date = last_event.value() + prcl.multiplied_by(remaining_periods);
        let zz = &PhantomIsoDatetimeW::new(new_mat_date).expect("oijk");
        MaturityDate::new(
                self.contract_terms.business_day_adjuster.clone().as_ref().unwrap().shift_bd(
                zz
            ).value()
        ).ok().unwrap()
    }

}

impl fmt::Display for NAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NAM")
    }
}


impl fmt::Debug for NAM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NAM")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for NAM {
    fn clone(&self) -> Self {
        NAM {
            contract_id: self.contract_id.clone(),
            contract_terms: self.contract_terms.clone(),
            risk_factor_external_data: None, // faire qqchose specifique ici ?
            risk_factor_external_event: None, // faire qqchose specifique ici ?
            related_contracts: None, // faire qqchose specifique ici ?
            event_timeline: self.event_timeline.clone(),
            states_space: self.states_space.clone(),
            status_date: self.status_date.clone(),
        }
    }
}

// Implémentation manuelle de PartialEq
impl PartialEq for NAM {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for NAM {}

impl Hash for NAM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}