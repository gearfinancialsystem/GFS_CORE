use std::{rc::Rc, collections::HashSet, fmt};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::time::ScheduleFactory::ScheduleFactory;

use crate::util::RedemptionUtils::RedemptionUtils;

use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_calendar::Calendar::Calendar;
use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use gfs_lib_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use gfs_lib_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use gfs_lib_terms::terms::grp_fees::FeeBasis::FeeBasis;
use gfs_lib_terms::terms::grp_fees::FeeRate::FeeRate;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use gfs_lib_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
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
use gfs_lib_types::traits::TraitConvert::{IsoCycleConvertToOption, IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::IsoPeriod::IsoPeriod;
use gfs_lib_types::types::Value::Value;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType::RRF;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub struct ANN {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub curr_event_index: i32,
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
    pub first_event_date: Option<PhantomIsoDatetimeW>,
    pub last_event_date: Option<PhantomIsoDatetimeW>,
}

impl TraitContractModel for ANN {
    fn new() -> Self {
        Self {
            contract_id: ContractID::new("init".to_string()).expect("init contract ID"),
            contract_terms: ContractTerms::default(),
            risk_factor_external_data: None,
            risk_factor_external_event: None,
            related_contracts: None,
            event_timeline: Vec::new(),
            curr_event_index: -1,
            states_space: StatesSpace::default(),
            status_date: None,
            first_event_date: None,
            last_event_date: None,
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

        // Champs qui dépendent d'autres champs
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

        let day_count_convention =
            if let Some(maturity_date) = &maturity_date {
                DayCountConvention::provide_from_input_dict(&sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
            } else {
                DayCountConvention::provide_from_input_dict(&sm, "dayCountConvention", None, Some(Rc::clone(&calendar)))
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


        let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict (&sm, "cycleOfOptionality");
        let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfOptionality::from_str(&a).ok()
        } else {
            CycleAnchorDateOfOptionality::provide_from_input_dict(&sm, "cycleAnchorDateOfOptionality")
        };



        let cycle_anchor_date_of_rate_reset = CycleAnchorDateOfRateReset::provide_from_input_dict(&sm, "cycleAnchorDateOfRateReset");
        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_reset.is_some() {
            cycle_anchor_date_of_rate_reset
        } else {
            if cycle_of_rate_reset.is_none() {
                None
            }
            else {
                let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value();
                CycleAnchorDateOfRateReset::new(a).ok()
            }

        };

        let cycle_of_interest_calculation_base = CycleOfInterestCalculationBase::provide_from_input_dict(&sm, "cycleOfInterestCalculationBase");
        let cycle_anchor_date_of_interest_calculation_base = if cycle_of_interest_calculation_base.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestCalculationBase::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestCalculationBase::provide_from_input_dict(&sm, "cycleAnchorDateOfInterestCalculationBase3")
        };


        let interest_calculation_base_tmp = InterestCalculationBase::provide_from_input_dict(&sm, "interestCalculationBase");
        let interest_calculation_base = if interest_calculation_base_tmp.is_none() {
            InterestCalculationBase::new("NT").ok()
        } else {
            interest_calculation_base_tmp
        };

        let cycle_of_principal_redemption = CycleOfPrincipalRedemption::provide_from_input_dict (&sm, "cycleOfPrincipalRedemption");

        // let b = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate");
        let mut cycle_anchor_date_of_principal_redemption = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(&sm, "cycleAnchorDateOfPrincipalRedemption");
        if cycle_anchor_date_of_principal_redemption.is_none() {
            cycle_anchor_date_of_principal_redemption = {
                let init = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value();
                Some(init.convert::<CycleAnchorDateOfPrincipalRedemption>())
            }
        }

        // let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
        //     let a = initial_exchange_date.value().to_string();
        //     CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
        // } else {
        //     CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(&sm, "cycleAnchorDateOfPrincipalRedemption")
        // };

        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                &sm,
                "businessDayConvention",
                calendar_clone.unwrap()
            )
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

        let mut fee_accrued = FeeAccrued::provide_from_input_dict(&sm, "feeAccrued");
        if fee_accrued.is_none() {
            fee_accrued = FeeAccrued::new(0.0).ok();
        }

        let mut nominal_interest_rate = NominalInterestRate::provide_from_input_dict(&sm, "nominalInterestRate");
        if nominal_interest_rate.is_none() {
            nominal_interest_rate = NominalInterestRate::new(0.0).ok();
        }

        let mut premium_discount_at_ied = PremiumDiscountAtIED::provide_from_input_dict(&sm, "premiumDiscountAtIED");
        if premium_discount_at_ied.is_none() {
            premium_discount_at_ied = PremiumDiscountAtIED::new(0.0).ok();
        }

        let mut price_at_purchase_date = PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate");
        if price_at_purchase_date.is_none() {
            price_at_purchase_date = PriceAtPurchaseDate::new(0.0).ok();
        }

        let mut price_at_termination_date = PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate");
        if price_at_termination_date.is_none() {
            price_at_termination_date = PriceAtTerminationDate::new(0.0).ok();
        }

        let mut scaling_index_at_contract_deal_date = ScalingIndexAtContractDealDate::provide_from_input_dict(&sm, "scalingIndexAtContractDealDate");
        if scaling_index_at_contract_deal_date.is_none() {
            scaling_index_at_contract_deal_date = ScalingIndexAtContractDealDate::new(0.0).ok();
        }
        let mut notional_scaling_multiplier = NotionalScalingMultiplier::provide_from_input_dict(&sm, "notionalScalingMultiplier");
        if notional_scaling_multiplier.is_none() {
            notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        }
        let mut interest_scaling_multiplier = InterestScalingMultiplier::provide_from_input_dict(&sm, "interestScalingMultiplier");
        if interest_scaling_multiplier.is_none() {
            interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        }
        let mut scaling_effect = ScalingEffect::provide_from_input_dict(&sm, "scalingEffect");
        if scaling_effect.is_none() {
            scaling_effect = ScalingEffect::new("OOO").ok();
        }

        let mut penalty_type = PenaltyType::provide_from_input_dict(&sm, "penaltyType");
        if penalty_type.is_none() {
            penalty_type = PenaltyType::new("N").ok();
        }

        let mut penalty_rate = PenaltyRate::provide_from_input_dict(&sm, "penaltyRate");
        if penalty_rate.is_none() {
            penalty_rate = PenaltyRate::new(0.0).ok();
        }

        let mut rate_spread = RateSpread::provide_from_input_dict(&sm, "rateSpread");
        if rate_spread.is_none() {
            rate_spread = RateSpread::new(0.0).ok();
        }

        let mut rate_multiplier = RateMultiplier::provide_from_input_dict(&sm, "rateMultiplier");
        if rate_multiplier.is_none() {
            rate_multiplier = RateMultiplier::new(1.0).ok();
        }

        let mut interest_calculation_base_amount = InterestCalculationBaseAmount::provide_from_input_dict(&sm, "interestCalculationBaseAmount");
        if interest_calculation_base_amount.is_none() {
            interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok();
        }

        let ct = ContractTerms {
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
            fee_accrued: fee_accrued,
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment"),
            nominal_interest_rate: nominal_interest_rate,
            day_count_convention: day_count_convention,
            accrued_interest: accrued_interest,
            capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(&sm, "capitalizationEndDate"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(&sm, "cyclePointOfInterestPayment"),
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate"),
            premium_discount_at_ied: premium_discount_at_ied,
            notional_principal: NotionalPrincipal::provide_from_input_dict(&sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(&sm, "purchaseDate"),
            price_at_purchase_date: price_at_purchase_date,
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            price_at_termination_date: price_at_termination_date,
            market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(&sm, "marketObjectCodeOfScalingIndex"),
            scaling_index_at_contract_deal_date: scaling_index_at_contract_deal_date,
            notional_scaling_multiplier: notional_scaling_multiplier,
            interest_scaling_multiplier: interest_scaling_multiplier,
            cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
            cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(&sm, "cycleOfScalingIndex"),
            scaling_effect: scaling_effect,
            cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
            cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(&sm, "cycleOfOptionality"),
            penalty_type: penalty_type,
            penalty_rate: penalty_rate,
            object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(&sm, "objectCodeOfPrepaymentModel"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset"),
            rate_spread: rate_spread,
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(&sm, "marketObjectCodeOfRateReset"),
            life_cap: life_cap,
            life_floor: life_floor,
            period_cap: period_cap,
            period_floor: period_floor,
            fixing_period: FixingPeriod::provide_from_input_dict(&sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(&sm, "nextResetRate"),
            rate_multiplier: rate_multiplier,
            maturity_date: maturity_date,
            cycle_anchor_date_of_interest_calculation_base: cycle_anchor_date_of_interest_calculation_base,
            cycle_of_interest_calculation_base: CycleOfInterestCalculationBase::provide_from_input_dict(&sm, "cycleOfInterestCalculationBase"),
            interest_calculation_base: interest_calculation_base,
            interest_calculation_base_amount: interest_calculation_base_amount,
            cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
            cycle_of_principal_redemption: cycle_of_principal_redemption,
            next_principal_redemption_payment: NextPrincipalRedemptionPayment::provide_from_input_dict(&sm, "nextPrincipalRedemptionPayment"),
            amortization_date: AmortizationDate::provide_from_input_dict(&sm, "amortizationDate"),
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
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();
        states.maturity_date = Some(Self::maturity(model));

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok(); //Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();// Some(0.0);
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok(); //Some(0.0);
        }
        else {
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
        }
        else if model.accrued_interest.is_some() {
            states.accrued_interest = AccruedInterest::new(&model.contract_role.clone().unwrap().role_sign() * model.accrued_interest.clone().unwrap().value()).ok();
        }
        else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.clone().business_day_adjuster.unwrap();

            let mut ip_schedule: Vec<PhantomIsoDatetimeW> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone().convert_option::<StartTime>(),
                &states.maturity_date.clone().convert_option::<EndTime>(),
                &model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<_> = ip_schedule.iter().filter(|&&date| date < states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = AccruedInterest::new(day_counter.day_count_fraction(
                time_adjuster.shift_sc(*t_minus),
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()),
            ) * states.notional_principal.clone().unwrap().value() * states.nominal_interest_rate.clone().unwrap().value()).ok();
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        }
        else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }

        if model.next_principal_redemption_payment.is_none() {
            if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
                // Fixed at initial PRF event
            } else {
                states.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(RedemptionUtils::redemptionAmount(model, &states)).ok();
            }
        }
        else {
            states.next_principal_redemption_payment = model.next_principal_redemption_payment.clone();
        }

        self.states_space = states;
    }

    fn init_contract_event_timeline(&mut self, to : Option<PhantomIsoDatetimeW>) {
        let mut events : Vec<ContractEvent> = Vec::new(); // A revoir
        let model = &self.contract_terms;
        let maturity = Self::maturity(model);
        let model = &self.contract_terms;
        // Initial exchange (IED)
        // ::<InitialExchangeDate, InitialExchangeDate>
        let e : ContractEvent = EventFactory::create_event(
            &model.initial_exchange_date.clone().convert_option::<ScheduleTime>(),
            &EventType::IED,
            &model.currency,
            Some(PayOffFunction::from_str("POF_IED_PAM")),
            Some(StatesTransitionFunction::from_str("STF_IED_LAM")),
            &None,
            &model.contract_id,
        );
        events.push(e);

        // Principal redemption (MD)
        // ::<MaturityDate, MaturityDate>
        let e: ContractEvent = EventFactory::create_event(
            &Some(maturity.clone().convert::<ScheduleTime>()),
            &EventType::MD,
            &model.currency,
            Some(PayOffFunction::from_str("POF_MD_PAM")),
            Some(StatesTransitionFunction::from_str("STF_MD_LAM")),
            &None,
            &model.contract_id,
        );
        events.push(e);



        // Principal redemption schedule (PR)
        let stf =
        if model.interest_calculation_base.clone().unwrap() != InterestCalculationBase::NT(NT) {
            StatesTransitionFunction::from_str("STF_PR_NAM")
        }
        else {
            StatesTransitionFunction::from_str("STF_PR2_NAM")
        };
        
        let a = &ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
            &Some(maturity.clone().convert::<EndTime>()),
            &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
            &model.end_of_month_convention.clone(),
            Some(false),
        );
        let mut test3: Vec<String> = vec![];

        for e in a.iter() {
            test3.push(e.to_string());
        }
        // let rr = &model.cycle_anchor_date_of_principal_redemption.unwrap().to_string();
        // let rrw = maturity.clone().to_string();
        // let ww = a.iter().map(|e| e.convert::<ScheduleTime>().to_string()).collect::<HashSet<String>>();
        let es = EventFactory::create_events(
            &a.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
            &EventType::PR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_PR_NAM")),
            Some(stf),
            &model.business_day_adjuster.clone(),
            &model.contract_id,
        );
        events.extend(es);

        let mut test2: Vec<String> = vec![];

        for e in events.clone() {
            test2.push(e.to_string());
        }

        // Initial principal redemption fixing event (PRF)
        if model.next_principal_redemption_payment.is_none() {
            let e = EventFactory::create_event(
                &CycleAnchorDateOfPrincipalRedemption::new((model.cycle_anchor_date_of_principal_redemption.clone().map(|d|
                    d.value() - IsoPeriod::of_days(1))).unwrap()).ok().convert_option::<ScheduleTime>(),
                &EventType::PRF,
                &model.currency,
                Some(PayOffFunction::from_str("POF_RR_PAM")),
                Some(StatesTransitionFunction::from_str("STF_PRF_ANN")),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e);
        }

        // Fees (FP)
        if model.cycle_of_fee.is_some() {
            events.extend(EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee.convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_fee.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(true),
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::FP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_FP_PAM")),
                Some(StatesTransitionFunction::from_str("STF_FP_LAM")),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            ));
        }

        // Purchase (PRD)
        if let Some(purchase_date) = model.purchase_date.clone() {
            let e: ContractEvent = EventFactory::create_event(
                &Some(purchase_date.convert::<ScheduleTime>()),
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_LAM")),
                Some(StatesTransitionFunction::from_str("STF_PRD_LAM")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }



        // Interest payment related events (IP)
        let stf_ipci = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            StatesTransitionFunction::from_str("STF_IPCI_LAM")
        } else {
            StatesTransitionFunction::from_str("STF_IPCI2_LAM")
        };

        if model.nominal_interest_rate.is_some() &&
            (model.cycle_of_interest_payment.is_some() ||
                model.cycle_anchor_date_of_interest_payment.is_some()) {
            let mut interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment.convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(true),
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_LAM")),
                Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
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
                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time.clone().unwrap() >= pranxm.convert::<EventTime>()));

                let ipanxm = EventFactory::create_event(
                    &Some(pranxm.convert::<ScheduleTime>()),
                    &EventType::IP,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IP_LAM")),
                    Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.insert(ipanxm);

                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(true),
                );

                let es = EventFactory::create_events(
                    &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                    &EventType::IP,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IP_LAM")),
                    Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.extend(es);
            }

            if let Some(capitalization_end_date) = model.capitalization_end_date.clone() {
                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date.convert::<ScheduleTime>()),
                    &EventType::IPCI,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time));
                interest_events.insert(capitalization_end.clone());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(PayOffFunction::from_str("POF_IPCI_PAM"));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.extend(interest_events);
        }
        else if model.capitalization_end_date.is_some() {
            let e: ContractEvent = EventFactory::create_event(
                &model.capitalization_end_date.clone().convert_option::<ScheduleTime>(),
                &EventType::IPCI,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                Some(stf_ipci),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e);

        }
        else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {

            let s = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
                &Some(maturity.clone().convert::<EndTime>()),
                &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            );
            let interest_events = EventFactory::create_events(
                &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_LAM")),
                Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(interest_events);
        }



        // Interest calculation base (IPCB)
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if interest_calculation_base.clone() == InterestCalculationBase::NTL(NTL) {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_calculation_base.clone().convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_interest_calculation_base.clone().convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                    &EventType::IPCB,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IPCB_LAM")),
                    Some(StatesTransitionFunction::from_str("STF_IPCB_LAM")),
                    &model.clone().business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Rate reset events (RR)
        let s = ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_rate_reset.convert_option::<StartTime>(),
            &Some(maturity.clone().convert::<EndTime>()),
            &model.cycle_of_rate_reset.convert_option::<PhantomIsoCycleW>(),
            &model.end_of_month_convention,
            Some(false),
        );
        let rate_reset_events = EventFactory::create_events(
            &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
            &EventType::RR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_RR_PAM")),
            Some(StatesTransitionFunction::from_str("STF_RR_LAM")),
            &model.clone().business_day_adjuster,
            &model.contract_id,
        );
        let mut rate_reset_events3: Vec<ContractEvent> = Vec::new();
        // adapt fixed rate reset event
        if let Some(_) = model.next_reset_rate.clone() {
            let status_event = EventFactory::create_event(
                &model.status_date.convert_option::<ScheduleTime>(),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            // let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            let mut fixed_event: Option<ContractEvent> = None;
            let mut rate_reset_events2 = rate_reset_events.clone().iter().map(|e| e.clone()).collect::<Vec<ContractEvent>>();

            rate_reset_events2.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));
            rate_reset_events3 = rate_reset_events2.clone();
            rate_reset_events3.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));

            for (u, e) in rate_reset_events2.iter().enumerate() {
                if e.compare_to(&status_event) == 1 {
                    fixed_event = Some(e.clone());
                    rate_reset_events3.remove(u);
                    break;
                }
            }

            if fixed_event.is_some() {
                let mut fixed_e = fixed_event.clone().unwrap();

                fixed_e.fstate = Some(StatesTransitionFunction::from_str("STF_RRF_LAM"));
                fixed_e.event_type = RRF;
                rate_reset_events3.push(fixed_e.clone());
                // rate_reset_events.insert(fixed_e);
                println!("ok");
            }

        }
        // add all rate reset events
        if model.next_reset_rate.is_some() {
            events.extend(rate_reset_events3.clone());
        }
        else {
            events.extend(rate_reset_events.clone());
        }

        // add all rate reset events
        let prf_schedule: HashSet<_> = rate_reset_events.clone().iter()
            .map(|e| e.event_time.unwrap()).collect();
        if !prf_schedule.is_empty() {
            let es = EventFactory::create_events(
                &prf_schedule.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                &EventType::PRF,
                &model.currency,
                Some(PayOffFunction::from_str("POF_RR_PAM")),
                Some(StatesTransitionFunction::from_str("STF_PRF_ANN")),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(es);
        }

        // scaling (if specified)
        if let Some(scaling_effect) = &model.scaling_effect {
            if scaling_effect.to_string().contains('I') || scaling_effect.to_string().contains('N') {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_scaling_index.clone().convert_option::<StartTime>(),
                    &Some(maturity.clone().convert::<EndTime>()),
                    &model.cycle_of_scaling_index.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s.iter().map(|e| e.convert::<ScheduleTime>()).collect::<HashSet<ScheduleTime>>(),
                    &EventType::SC,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_SC_PAM")),
                    Some(StatesTransitionFunction::from_str("STF_SC_LAM")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Termination event (TD)
        if let Some(termination_date) = model.termination_date.clone() {
            let termination: ContractEvent = EventFactory::create_event(
                &Some(termination_date.convert::<ScheduleTime>()),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_LAM")),
                Some(StatesTransitionFunction::from_str("STF_TD_PAM")),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event: ContractEvent = EventFactory::create_event(
            &model.status_date.convert_option::<ScheduleTime>(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e >= &status_event);

        // Remove all post to-date events
        if to.is_some() {
            let to_event = EventFactory::create_event(
                &Some(to.clone().unwrap().convert::<ScheduleTime>()),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e <= &to_event);
        }


        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        self.event_timeline = events.clone();
        self.sort_events_timeline();
        let mut test: Vec<String> = vec![];
        for e in self.event_timeline.clone() {
            test.push(e.to_string());
        }
        println!("ok");
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

            self.event_timeline[id_ce].payoff = Some(a.expect("ok"));
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

    fn next_day(&mut self, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {
        // ici on met Vec<TestResult> car il peut y avoir plusieur event le meme jour
        // itere un jour apres lautre
        let period1d = *PhantomIsoPeriodW::new(0,0,1);
        let next_status_date = self.status_date.convert_option::<PhantomIsoDatetimeW>().unwrap().value()
            + period1d;
        let next_event_index = (self.curr_event_index + 1) as usize;
        let mut next_event_date = self.event_timeline.get(next_event_index).unwrap().get_schedule_time();

        if next_status_date < next_event_date.value() {
            self.status_date = StatusDate::new(next_status_date).ok();
            let oo = self.status_date.clone()?.to_string();
            None
        }
        else { // case >=, seul = doit etre matche
            let mut result_vec: Vec<TestResult> = Vec::new();
            let mut curr_next_event_index = next_event_index;
            while next_status_date == next_event_date.value() {
                let ww = next_status_date.to_string();
                let www = next_event_date.to_string();

                result_vec.push(self.next_event(extract_results).expect("ok").expect("ok"));
                curr_next_event_index += 1;
                if curr_next_event_index == self.event_timeline.len() {
                    break;
                }
                next_event_date = self.event_timeline.get(curr_next_event_index).unwrap().get_schedule_time();
            }
            self.status_date = StatusDate::new(next_status_date).ok();
            Some(Ok(result_vec))
        }

    }
    
    fn next_event(&mut self, extract_results: bool) -> Option<Result<TestResult, String>> {


        let next_event_index = (self.curr_event_index + 1) as usize;
        if next_event_index < self.event_timeline.len() {

            self.eval_pof_contract_event(next_event_index);
            self.eval_stf_contract_event(next_event_index);
            self.curr_event_index += 1;
            if extract_results == true {
                let curr_testresult = TestResult {
                    eventDate: self.event_timeline[next_event_index].event_time.expect("fe").to_string(),
                    eventType: self.event_timeline[next_event_index].event_type.to_string(),
                    payoff: self.event_timeline[next_event_index].payoff.clone().expect("ok").to_string(),
                    currency: self.event_timeline[next_event_index].currency.clone().expect("ef").0,
                    notionalPrincipal: self.states_space.notional_principal.clone().expect("ok").to_string(),
                    nominalInterestRate: self.states_space.nominal_interest_rate.clone().expect("ok").to_string(),
                    accruedInterest: self.states_space.accrued_interest.clone().expect("ok").to_string(),
                };
                Some(Ok(curr_testresult))
            }
            else {
                Some(Err("Err ave TestResult".to_string()))
            }

        } else {
            None
        }
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
        self.sort_events_timeline();
        let events_len = self.event_timeline.len();
        let mut result_vec: Vec<TestResult> = Vec::new();

        while self.curr_event_index + 1 < events_len as i32 { // i < events_len {
            if self.curr_event_index > -1 {
                if date.is_some() {
                    if self.event_timeline[self.curr_event_index as usize].event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                        break
                    }
                }
            }

            let curr_testresult: Option<Result<TestResult, String>> = self.next_event(extract_results);
            if extract_results == true {
                if curr_testresult.clone().unwrap().is_ok() {
                    result_vec.push(curr_testresult.clone().unwrap().unwrap());
                }
            }
        }

        if let Some(purchase_date) = &self.contract_terms.purchase_date.clone() {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            self.event_timeline.retain(|e|
                !(e.event_type != EventType::AD && e.compare_to(&purchase_event) == -1) );
        }

        // self.event_timeline = events.clone();
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
            let mut previous_events: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption.convert_option::<StartTime>(),
                &Some(t0.clone().convert::<EndTime>()),
                &model.cycle_of_principal_redemption.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(false)
            ).iter().map(|e| e.value()).collect::<Vec<IsoDatetime>>();

            previous_events.retain(|&d| d > t0.value());
            previous_events.sort();
            *previous_events.last().unwrap()
        };

        let time_from_last_event_plus_one_cycle = model.day_count_convention.as_ref().unwrap().day_count_fraction(PhantomIsoDatetimeW::new(last_event).expect("okj"), PhantomIsoDatetimeW::new(last_event + prcl.clone()).expect("okj") );
        let redemption_per_cycle = model.next_principal_redemption_payment.clone().unwrap().value() - (time_from_last_event_plus_one_cycle * model.nominal_interest_rate.clone().unwrap().value() * model.notional_principal.clone().unwrap().value());
        let remaining_periods = ((model.notional_principal.clone().unwrap().value() / redemption_per_cycle).ceil() - 1.0) as i32;

        MaturityDate::new(model.business_day_adjuster.clone().unwrap()
            .shift_bd( &  PhantomIsoDatetimeW::new(last_event.clone() + prcl.multiplied_by(remaining_periods)).expect("ok")   ).value()
        ).ok().unwrap()
    }

}




impl fmt::Display for ANN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ANN")
    }
}


impl fmt::Debug for ANN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ANN")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for ANN {
    fn clone(&self) -> Self {
        ANN {
            contract_id: self.contract_id.clone(),
            contract_terms: self.contract_terms.clone(),
            risk_factor_external_data: None, // faire qqchose specifique ici ?
            risk_factor_external_event: None, // faire qqchose specifique ici ?
            related_contracts: None, // faire qqchose specifique ici ?
            event_timeline: self.event_timeline.clone(),
            curr_event_index: self.curr_event_index.clone(),
            states_space: self.states_space.clone(),
            status_date: self.status_date.clone(),
            first_event_date: self.first_event_date.clone(),
            last_event_date: self.last_event_date.clone(),
        }
    }
}

// Implémentation manuelle de PartialEq
impl PartialEq for ANN {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for ANN {}

impl Hash for ANN {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}

