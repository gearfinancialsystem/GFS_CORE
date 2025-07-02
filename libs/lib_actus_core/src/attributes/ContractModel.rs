use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use crate::attributes::ContractReference::ContractReference;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use crate::terms::grp_boundary::BoundaryMonitoringAnchorDate::BoundaryMonitoringAnchorDate;
use crate::terms::grp_boundary::BoundaryMonitoringCycle::BoundaryMonitoringCycle;
use crate::terms::grp_boundary::BoundaryMonitoringEndDate::BoundaryMonitoringEndDate;
use crate::terms::grp_boundary::BoundaryValue::BoundaryValue;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractStructure::ContractStructure;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::CreatorID::CreatorID;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use crate::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use crate::terms::grp_counterparty::GracePeriod::GracePeriod;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_counterparty::PrepaymentPeriod::PrepaymentPeriod;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use crate::terms::grp_dividend::CycleAnchorDateOfDividendPayment::CycleAnchorDateOfDividendPayment;
use crate::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use crate::terms::grp_dividend::CycleOfDividendPayment;
use crate::terms::grp_dividend::ExDividendDate::ExDividendDate;
use crate::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::ArrayCycleAnchorDateOfInterestPayment::ArrayCycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::ArrayCycleOfInterestPayment::ArrayCycleOfInterestPayment;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use crate::terms::grp_notional_principal::ArrayCycleAnchorDateOfPrincipalRedemption::ArrayCycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::ArrayCycleOfPrincipalRedemption::ArrayCycleOfPrincipalRedemption;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::terms::grp_notional_principal::ArrayNextPrincipalRedemptionPayment::ArrayNextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::Currency2::Currency2;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use crate::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use crate::terms::grp_notional_principal::MarketValueObserved::MarketValueObserved;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::Quantity::Quantity;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_notional_principal::XDayNotice::XDayNotice;
use crate::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use crate::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use crate::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use crate::terms::grp_optionality::OptionStrike1::OptionStrike1;
use crate::terms::grp_optionality::OptionStrike2::OptionStrike2;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::PenaltyRate::PenaltyRate;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::ArrayCycleAnchorDateOfRateReset::ArrayCycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::ArrayCycleOfRateReset::ArrayCycleOfRateReset;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::terms::grp_reset_rate::ArrayRate::ArrayRate;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use crate::terms::grp_reset_rate::LifeCap::LifeCap;
use crate::terms::grp_reset_rate::LifeFloor::LifeFloor;
use crate::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use crate::terms::grp_reset_rate::NextResetRate::NextResetRate;
use crate::terms::grp_reset_rate::PeriodCap::PeriodCap;
use crate::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use crate::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use crate::terms::grp_reset_rate::RateSpread::RateSpread;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::terms::grp_settlement::FuturesPrice::FuturesPrice;
use crate::terms::grp_settlement::SettlementCurrency::SettlementCurrency;
use crate::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use crate::types::IsoDatetime::{TraitNaiveDateTimeExtension, IsoDatetime};
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::CommonUtils::CommonUtils;
use crate::util::Value::Value;

#[derive(PartialEq, Debug, Clone)]
pub enum FieldValue {
    Vstring(String),
    vF64(f64),
    vIsoDatetime(IsoDatetime),
    vIsoPeriod(IsoPeriod),
    vBusinessDayAdjuster(BusinessDayAdjuster),
    vDayCountConvention(DayCountConvention),
    vEndOfMonthConvention(EndOfMonthConvention),
    vContractRole(ContractRole),
    vContractStructure(ContractStructure),
    vFeeBasis(FeeBasis),
    vCyclePointOfInterestPayment(CyclePointOfInterestPayment),
    vContractPerformance(ContractPerformance),
    vScalingEffect(ScalingEffect),
    vPenaltyType(PenaltyType),
    vCyclePointOfRateReset(CyclePointOfRateReset),
    vMaturityDate(Rc<IsoDatetime>),
    vCalendar(Rc<Calendar>),
    vDeliverySettlement(DeliverySettlement),
    vSeniority(Seniority),
    vGuaranteedExposure(GuaranteedExposure),
    vOptionType(OptionType),
    vInterestCalculationBase(InterestCalculationBase),
    vBoundaryDirection(BoundaryDirection),
    vBoundaryEffect(BoundaryEffect),
    vBoundaryLegInitiallyActive(BoundaryLegInitiallyActive),
    vBool(bool),
    vVecIsoDatetime(Vec<IsoDatetime>),
    vVecF64(Vec<f64>),
    vVecString(Vec<String>),
    vArrayIncreaseDecrease(Vec<ArrayIncreaseDecrease>),
    vArrayFixedVariable(Vec<ArrayFixedVariable>),
    vVecCreditEventTypeCovered(Vec<CreditEventTypeCovered>),
}

impl FieldValue {
    pub fn extract_vString(&self) -> Option<String> {
        match self {
            Self::Vstring(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn extract_vF64(&self) -> Option<f64> {
        match self {
            Self::vF64(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn extract_vIsoDatetime(&self) -> Option<IsoDatetime> {
        match self { 
            Self::vIsoDatetime(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn extract_vBusinessDayAdjuster(&self) -> Option<BusinessDayAdjuster> {
        match self {
            Self::vBusinessDayAdjuster(s) => Some(s.clone()),
            _ => None,
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
#[derive(Default)] // Toutes les options sont None
pub struct ContractModel {
    pub accrued_interest: Option<AccruedInterest>,
    pub accrued_interest2: Option<AccruedInterest2>,
    pub amortization_date: Option<AmortizationDate>,
    pub array_cycle_anchor_date_of_interest_payment: Option<ArrayCycleAnchorDateOfInterestPayment>,
    pub array_cycle_anchor_date_of_principal_redemption: Option<ArrayCycleAnchorDateOfPrincipalRedemption>,
    pub array_cycle_anchor_date_of_rate_reset: Option<ArrayCycleAnchorDateOfRateReset>,
    pub array_cycle_of_interest_payment: Option<ArrayCycleOfInterestPayment>,
    pub array_cycle_of_principal_redemption: Option<ArrayCycleOfPrincipalRedemption>,
    pub array_cycle_of_rate_reset: Option<ArrayCycleOfRateReset>,
    pub array_fixed_variable: Option<ArrayFixedVariable>,
    pub array_increase_decrease: Option<Vec<ArrayIncreaseDecrease>>,
    pub array_next_principal_redemption_payment: Option<ArrayNextPrincipalRedemptionPayment>,
    pub array_rate: Option<ArrayRate>,
    pub boundary_crossed_flag: Option<BoundaryCrossedFlag>,
    pub boundary_direction: Option<BoundaryDirection>,
    pub boundary_effect: Option<BoundaryEffect>,
    pub boundary_leg_initially_active: Option<BoundaryLegInitiallyActive>,
    pub boundary_monitoring_anchor_date: Option<BoundaryMonitoringAnchorDate>,
    pub boundary_monitoring_cycle: Option<BoundaryMonitoringCycle>,
    pub boundary_monitoring_end_date: Option<BoundaryMonitoringEndDate>,
    pub boundary_value: Option<BoundaryValue>,
    pub business_day_adjuster: Option<BusinessDayAdjuster>,
    pub calendar: Option<Rc<Calendar>>,
    pub capitalization_end_date: Option<CapitalizationEndDate>,
    pub contract_id: Option<ContractID>,
    pub contract_performance: Option<ContractPerformance>,
    pub contract_role: Option<ContractRole>,
    pub contract_structure: Option<ContractStructure>,
    pub contract_type: String, // le seul term different de la specification
    pub counterparty_id: Option<CounterpartyID>,
    pub coverage_of_credit_enhancement: Option<CoverageOfCreditEnhancement>,
    pub creator_id: Option<CreatorID>,
    pub credit_event_type_covered: Option<CreditEventTypeCovered>,
    pub currency: Option<Currency>,
    pub currency2: Option<Currency2>,
    pub cycle_anchor_date_of_dividend: Option<CycleAnchorDateOfDividend>,
    pub cycle_anchor_date_of_dividend_payment: Option<CycleAnchorDateOfDividendPayment>,
    pub cycle_anchor_date_of_fee: Option<CycleAnchorDateOfFee>,
    pub cycle_anchor_date_of_interest_calculation_base: Option<CycleAnchorDateOfInterestCalculationBase>,
    pub cycle_anchor_date_of_interest_payment: Option<CycleAnchorDateOfInterestPayment>,
    pub cycle_anchor_date_of_optionality: Option<CycleAnchorDateOfOptionality>,
    pub cycle_anchor_date_of_principal_redemption: Option<CycleAnchorDateOfPrincipalRedemption>,
    pub cycle_anchor_date_of_rate_reset: Option<CycleAnchorDateOfRateReset>,
    pub cycle_anchor_date_of_scaling_index: Option<CycleAnchorDateOfScalingIndex>,
    pub cycle_of_dividend: Option<CycleOfDividend>,
    pub cycle_of_fee: Option<CycleOfFee>,
    pub cycle_of_interest_calculation_base: Option<CycleOfInterestCalculationBase>,
    pub cycle_of_interest_payment: Option<CycleOfInterestPayment>,
    pub cycle_of_optionality: Option<CycleOfOptionality>,
    pub cycle_of_principal_redemption: Option<CycleOfPrincipalRedemption>,
    pub cycle_of_rate_reset: Option<CycleOfRateReset>,
    pub cycle_of_scaling_index: Option<CycleOfScalingIndex>,
    pub cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>,
    pub cycle_point_of_rate_reset: Option<CyclePointOfRateReset>,
    pub day_count_convention: Option<DayCountConvention>,
    pub delinquency_period: Option<DelinquencyPeriod>,
    pub delinquency_rate: Option<DelinquencyRate>,
    pub delivery_settlement: Option<DeliverySettlement>,
    pub end_of_month_convention: Option<EndOfMonthConvention>,
    pub ex_dividend_date: Option<ExDividendDate>,
    pub exercise_amount: Option<ExerciseAmount>,
    pub exercise_date: Option<ExerciseDate>,
    pub fee_accrued: Option<FeeAccrued>,
    pub fee_basis: Option<FeeBasis>,
    pub fee_rate: Option<FeeRate>,
    pub fixing_period: Option<FixingPeriod>,
    pub futures_price: Option<FuturesPrice>,
    pub grace_period: Option<GracePeriod>,
    pub guaranteed_exposure: Option<GuaranteedExposure>,
    pub initial_exchange_date: Option<InitialExchangeDate>,
    pub interest_calculation_base: Option<InterestCalculationBase>,
    pub interest_calculation_base_amount: Option<InterestCalculationBaseAmount>,
    pub interest_scaling_multiplier: Option<InterestScalingMultiplier>,
    pub life_cap: Option<LifeCap>,
    pub life_floor: Option<LifeFloor>,
    pub market_object_code: Option<MarketObjectCode>,
    pub market_object_code_of_rate_reset: Option<MarketObjectCodeOfRateReset>,
    pub market_object_code_of_scaling_index: Option<MarketObjectCodeOfScalingIndex>,
    pub market_value_observed: Option<MarketValueObserved>,
    pub maturity_date: Option<Rc<MaturityDate>>,
    pub nominal_interest_rate: Option<NominalInterestRate>,
    pub nominal_interest_rate2: Option<NominalInterestRate2>,
    pub non_performing_date: Option<NonPerformingDate>,
    pub notional_principal: Option<NotionalPrincipal>,
    pub notional_principal2: Option<NotionalPrincipal2>,
    pub notional_scaling_multiplier: Option<NotionalScalingMultiplier>,
    pub next_dividend_payment_amount: Option<NextDividendPaymentAmount>,
    pub next_principal_redemption_payment: Option<NextPrincipalRedemptionPayment>,
    pub next_reset_rate: Option<NextResetRate>,
    pub object_code_of_prepayment_model: Option<ObjectCodeOfPrepaymentModel>, // nest pas dans la liste des champs sur le site, mais dans le code
    pub option_strike1: Option<OptionStrike1>,
    pub option_strike2: Option<OptionStrike2>,
    pub option_type: Option<OptionType>,
    pub penalty_rate: Option<PenaltyRate>,
    pub penalty_type: Option<PenaltyType>,
    pub period_cap: Option<PeriodCap>,
    pub period_floor: Option<PeriodFloor>,
    pub prepayment_period: Option<PrepaymentPeriod>,
    pub premium_discount_at_ied: Option<PremiumDiscountAtIED>,
    pub price_at_purchase_date: Option<PriceAtPurchaseDate>,
    pub price_at_termination_date: Option<PriceAtTerminationDate>,
    pub purchase_date: Option<PurchaseDate>,
    pub quantity: Option<Quantity>,
    pub rate_multiplier: Option<RateMultiplier>,
    pub rate_spread: Option<RateSpread>,
    pub scaling_effect: Option<ScalingEffect>,
    pub scaling_index_at_contract_deal_date: Option<ScalingIndexAtContractDealDate>,
    pub seniority: Option<Seniority>,
    pub settlement_currency: Option<SettlementCurrency>,
    pub settlement_period: Option<SettlementPeriod>,
    pub status_date: Option<StatusDate>,
    pub termination_date: Option<TerminationDate>,
    pub x_day_notice: Option<XDayNotice>,
}


impl ContractModel {

    pub fn get_field(&self, field_name: &str) -> Option<FieldValue> {
        match field_name {
            "accrued_interest" => Some(FieldValue::vF64(self.accrued_interest?)),
            "accrued_interest2" => Some(FieldValue::vF64(self.accrued_interest2.clone().unwrap())),
            "amortization_date" => Some(FieldValue::vIsoDatetime(self.amortization_date.clone().unwrap())),
            "array_cycle_anchor_date_of_interest_payment" => Some(FieldValue::vVecIsoDatetime(self.array_cycle_anchor_date_of_interest_payment.clone().unwrap())),
            "array_cycle_anchor_date_of_principal_redemption" => Some(FieldValue::vVecIsoDatetime(self.array_cycle_anchor_date_of_principal_redemption.clone().unwrap())),
            "array_cycle_anchor_date_of_rate_reset" => Some(FieldValue::vVecIsoDatetime(self.array_cycle_anchor_date_of_rate_reset.clone().unwrap())),
            "array_cycle_of_interest_payment" => Some(FieldValue::vVecString(self.array_cycle_of_interest_payment.clone().unwrap())),
            "array_cycle_of_principal_redemption" => Some(FieldValue::vVecString(self.array_cycle_of_principal_redemption.clone().unwrap())),
            "array_cycle_of_rate_reset" => Some(FieldValue::vVecString(self.array_cycle_of_rate_reset.clone().unwrap())),
            "array_fixed_variable" => Some(FieldValue::vArrayFixedVariable(self.array_fixed_variable.clone().unwrap())),
            "array_increase_decrease" => Some(FieldValue::vArrayIncreaseDecrease(self.array_increase_decrease.clone().unwrap())),
            "array_next_principal_redemption_payment" => Some(FieldValue::vVecF64(self.array_next_principal_redemption_payment.clone().unwrap())),
            "array_rate" => Some(FieldValue::vVecF64(self.array_rate.clone().unwrap())),
            "boundary_crossed_flag" => Some(FieldValue::vBool(self.boundary_crossed_flag.clone().unwrap())),
            "boundary_direction" => Some(FieldValue::vBoundaryDirection(self.boundary_direction.clone().unwrap())),
            "boundary_effect" => Some(FieldValue::vBoundaryEffect(self.boundary_effect.clone().unwrap())),
            "boundary_leg_initially_active" => Some(FieldValue::vBoundaryLegInitiallyActive(self.boundary_leg_initially_active.clone().unwrap())),
            "boundary_monitoring_anchor_date" => Some(FieldValue::vIsoDatetime(self.boundary_monitoring_anchor_date.clone().unwrap())),
            "boundary_monitoring_cycle" => Some(FieldValue::Vstring(self.boundary_monitoring_cycle.clone().unwrap())),
            "boundary_monitoring_end_date" => Some(FieldValue::vIsoDatetime(self.boundary_monitoring_end_date.clone().unwrap())),
            "boundary_value" => Some(FieldValue::vF64(self.boundary_value.clone().unwrap())),
            "business_day_adjuster" => Some(FieldValue::vBusinessDayAdjuster(self.business_day_adjuster.clone().unwrap())),
            "calendar" => Some(FieldValue::vCalendar(self.calendar.clone().unwrap())),
            "capitalization_end_date" => Some(FieldValue::vIsoDatetime(self.capitalization_end_date?)),
            "contract_id" => Some(FieldValue::Vstring(self.contract_id.clone().unwrap())),
            "contract_performance" => Some(FieldValue::vContractPerformance(self.contract_performance?)),
            "contract_role" => Some(FieldValue::vContractRole(self.contract_role.clone().unwrap())),
            "contract_type" => Some(FieldValue::Vstring(self.contract_type.clone())),
            "counterparty_id" => Some(FieldValue::Vstring(self.counterparty_id.clone().unwrap())),
            "coverage_of_credit_enhancement" => Some(FieldValue::vF64(self.coverage_of_credit_enhancement.clone().unwrap())),
            "creator_id" => Some(FieldValue::Vstring(self.creator_id.clone().unwrap())),
            "credit_event_type_covered" => Some(FieldValue::vVecCreditEventTypeCovered(self.credit_event_type_covered.clone().unwrap())),
            "currency" => Some(FieldValue::Vstring(self.currency.clone().unwrap())),
            "currency2" => Some(FieldValue::Vstring(self.currency2.clone().unwrap())),
            "cycle_anchor_date_of_dividend" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_dividend?)),
            "cycle_anchor_date_of_dividend_payment" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_dividend_payment.clone().unwrap())),
            "cycle_anchor_date_of_fee" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_fee?)),
            "cycle_anchor_date_of_interest_calculation_base" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_interest_calculation_base.clone().unwrap())),
            "cycle_anchor_date_of_interest_payment" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_interest_payment?)),
            "cycle_anchor_date_of_optionality" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_optionality?)),
            "cycle_anchor_date_of_principal_redemption" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_principal_redemption.clone().unwrap())),
            "cycle_anchor_date_of_rate_reset" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_rate_reset?)),
            "cycle_anchor_date_of_scaling_index" => Some(FieldValue::vIsoDatetime(self.cycle_anchor_date_of_scaling_index?)),
            "cycle_of_dividend" => Some(FieldValue::Vstring(self.cycle_of_dividend.clone().unwrap())),
            "cycle_of_dividend_payment" => Some(FieldValue::Vstring(self.cycle_of_dividend_payment.clone().unwrap())),
            "cycle_of_fee" => Some(FieldValue::Vstring(self.cycle_of_fee.clone().unwrap())),
            "cycle_of_interest_calculation_base" => Some(FieldValue::Vstring(self.cycle_of_interest_calculation_base.clone().unwrap())),
            "cycle_of_interest_payment" => Some(FieldValue::Vstring(self.cycle_of_interest_payment.clone().unwrap())),
            "cycle_of_optionality" => Some(FieldValue::Vstring(self.cycle_of_optionality.clone().unwrap())),
            "cycle_of_principal_redemption" => Some(FieldValue::Vstring(self.cycle_of_principal_redemption.clone().unwrap())),
            "cycle_of_rate_reset" => Some(FieldValue::Vstring(self.cycle_of_rate_reset.clone().unwrap())),
            "cycle_of_scaling_index" => Some(FieldValue::Vstring(self.cycle_of_scaling_index.clone().unwrap())),
            "cycle_point_of_interest_payment" => Some(FieldValue::vCyclePointOfInterestPayment(self.cycle_point_of_interest_payment.clone().unwrap())),
            "cycle_point_of_rate_reset" => Some(FieldValue::vCyclePointOfRateReset(self.cycle_point_of_rate_reset.clone().unwrap())),
            "day_count_convention" => Some(FieldValue::vDayCountConvention(self.day_count_convention.clone().unwrap())),
            "delinquency_period" => Some(FieldValue::vIsoPeriod(self.delinquency_period.clone().unwrap())),
            "delinquency_rate" => Some(FieldValue::vF64(self.delinquency_rate.clone().unwrap())),
            "delivery_settlement" => Some(FieldValue::vDeliverySettlement(self.delivery_settlement.clone().unwrap())),
            "exercise_amount" => Some(FieldValue::vF64(self.exercise_amount.clone().unwrap())),
            "exercise_date" => Some(FieldValue::vIsoDatetime(self.exercise_date.clone().unwrap())),
            "ex_dividend_date" => Some(FieldValue::vIsoDatetime(self.ex_dividend_date.clone().unwrap())),
            "fee_accrued" => Some(FieldValue::vF64(self.fee_accrued?)),
            "fee_basis" => Some(FieldValue::vFeeBasis(self.fee_basis.clone().unwrap())),
            "fee_rate" => Some(FieldValue::vF64(self.fee_rate?)),
            "fixing_period" => Some(FieldValue::vIsoPeriod(self.fixing_period.clone().unwrap())),
            "futures_price" => Some(FieldValue::vF64(self.futures_price.clone().unwrap())),
            "guaranteed_exposure" => Some(FieldValue::vGuaranteedExposure(self.guaranteed_exposure.clone().unwrap())),
            "grace_period" => Some(FieldValue::vIsoPeriod(self.grace_period.clone().unwrap())),
            "interest_calculation_base" => Some(FieldValue::vInterestCalculationBase(self.interest_calculation_base.clone().unwrap())),
            "interest_calculation_base_amount" => Some(FieldValue::vF64(self.interest_calculation_base_amount.clone().unwrap())),
            "interest_scaling_multiplier" => Some(FieldValue::vF64(self.interest_scaling_multiplier?)),
            "initial_exchange_date" => Some(FieldValue::vIsoDatetime(self.initial_exchange_date?)),
            "life_cap" => Some(FieldValue::vF64(self.life_cap?)),
            "life_floor" => Some(FieldValue::vF64(self.life_floor?)),
            "market_object_code" => Some(FieldValue::Vstring(self.market_object_code.clone().unwrap())),
            "market_object_code_of_dividends" => Some(FieldValue::Vstring(self.market_object_code_of_dividends.clone().unwrap())),
            "market_object_code_of_rate_reset" => Some(FieldValue::Vstring(self.market_object_code_of_rate_reset.clone().unwrap())),
            "market_object_code_of_scaling_index" => Some(FieldValue::Vstring(self.market_object_code_of_scaling_index.clone().unwrap())),
            "market_value_observed" => Some(FieldValue::vF64(self.market_value_observed.clone().unwrap())),
            "maturity_date" => Some(FieldValue::vMaturityDate(self.maturity_date.clone().unwrap())),
            "next_dividend_payment_amount" => Some(FieldValue::vF64(self.next_dividend_payment_amount.clone().unwrap())),
            "next_principal_redemption_payment" => Some(FieldValue::vF64(self.next_principal_redemption_payment.clone().unwrap())),
            "next_reset_rate" => Some(FieldValue::vF64(self.next_reset_rate?)),
            "nominal_interest_rate" => Some(FieldValue::vF64(self.nominal_interest_rate?)),
            "nominal_interest_rate2" => Some(FieldValue::vF64(self.nominal_interest_rate2.clone().unwrap())),
            "non_performing_date" => Some(FieldValue::vIsoDatetime(self.non_performing_date.clone().unwrap())),
            "notional_principal" => Some(FieldValue::vF64(self.notional_principal?)),
            "notional_principal2" => Some(FieldValue::vF64(self.notional_principal2.clone().unwrap())),
            "notional_scaling_multiplier" => Some(FieldValue::vF64(self.notional_scaling_multiplier?)),
            "object_code_of_prepayment_model" => Some(FieldValue::Vstring(self.object_code_of_prepayment_model.clone().unwrap())),
            "option_strike1" => Some(FieldValue::vF64(self.option_strike1.clone().unwrap())),
            "option_strike2" => Some(FieldValue::vF64(self.option_strike2.clone().unwrap())),
            "option_type" => Some(FieldValue::vOptionType(self.option_type.clone().unwrap())),
            "penalty_rate" => Some(FieldValue::vF64(self.penalty_rate?)),
            "penalty_type" => Some(FieldValue::vPenaltyType(self.penalty_type.clone().unwrap())),
            "prepayment_period" => Some(FieldValue::vIsoPeriod(self.prepayment_period.clone().unwrap())),
            "premium_discount_at_ied" => Some(FieldValue::vF64(self.premium_discount_at_ied?)),
            "price_at_purchase_date" => Some(FieldValue::vF64(self.price_at_purchase_date?)),
            "price_at_termination_date" => Some(FieldValue::vF64(self.price_at_termination_date?)),
            "purchase_date" => Some(FieldValue::vIsoDatetime(self.purchase_date?)),
            "quantity" => Some(FieldValue::vF64(self.quantity.clone().unwrap())),
            "rate_multiplier" => Some(FieldValue::vF64(self.rate_multiplier?)),
            "rate_spread" => Some(FieldValue::vF64(self.rate_spread?)),
            "scaling_effect" => Some(FieldValue::vScalingEffect(self.scaling_effect.clone().unwrap())),
            "scaling_index_at_contract_deal_date" => Some(FieldValue::vF64(self.scaling_index_at_contract_deal_date?)),
            "seniority" => Some(FieldValue::vSeniority(self.seniority.clone().unwrap())),
            "settlement_currency" => Some(FieldValue::Vstring(self.settlement_currency.clone().unwrap())),
            "settlement_period" => Some(FieldValue::vIsoPeriod(self.settlement_period.clone().unwrap())),
            "status_date" => Some(FieldValue::vIsoDatetime(self.status_date?)),
            "termination_date" => Some(FieldValue::vIsoDatetime(self.termination_date?)),
            "x_day_notice" => Some(FieldValue::vIsoPeriod(self.x_day_notice.clone().unwrap())),
            _ => None,
        }
    }

    pub fn new(sm: &HashMap<String, Value>) -> Result<ContractModel, String> {
        let ct = sm.get("contractType").unwrap();
        let ct_str = ct.as_string().unwrap().as_str()
        match ct_str {
            "PAM" => {
                //let mut cm = ContractModel::init();
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
                let calendar = Calendar::provide_rc(sm, "calendar");

                let business_day_adjuster = if let Some(calendar) = &calendar {

                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone.expect("te")
                    )
                };

                // Clonez simplement les Rc existantes
                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                     DayCountConvention::provide(
                        sm,
                        "dayCountConvention",
                        Some(Rc::clone(maturity_date)),
                        Some(Rc::clone(calendar))
                    )
                };
                
                let cm = ContractModel {
                    accrued_interest:                       AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    business_day_adjuster:                  business_day_adjuster,
                    calendar:                               calendar,
                    capitalization_end_date:                CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
                    contract_id:                            ContractID::provide_from_input_dict(sm, "contractID"),
                    contract_performance:                   ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    contract_role:                          ContractRole::provide_from_input_dict(sm, "contractRole"),
                    contract_type:                          ct_str.clone().to_string(),
                    counterparty_id:                        CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    currency:                               Currency::provide_from_input_dict(sm, "currency"),
                    cycle_anchor_date_of_fee:               CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_anchor_date_of_interest_payment:  CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
                    cycle_anchor_date_of_optionality:       CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality"),
                    cycle_anchor_date_of_rate_reset:        CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset"),
                    cycle_anchor_date_of_scaling_index:     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex"),
                    cycle_of_fee:                           CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    cycle_of_interest_payment:              CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    cycle_of_optionality:                   CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
                    cycle_of_rate_reset:                    CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
                    cycle_of_scaling_index:                 CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
                    cycle_point_of_interest_payment:        CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
                    cycle_point_of_rate_reset:              CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset"),
                    day_count_convention:                   day_count_convention,
                    end_of_month_convention:                EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    fee_accrued:                            FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    fee_basis:                              FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate:                               FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fixing_period:                          FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    initial_exchange_date:                  InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    interest_scaling_multiplier:            InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
                    life_cap:                               LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor:                             LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    market_object_code:                     MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    market_object_code_of_rate_reset:       MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
                    market_object_code_of_scaling_index:    MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
                    maturity_date:                          maturity_date,
                    next_reset_rate:                        NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    nominal_interest_rate:                  NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    notional_principal:                     NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    notional_scaling_multiplier:            NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
                    object_code_of_prepayment_model:        ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
                    penalty_rate:                           PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
                    penalty_type:                           PenaltyType::provide_from_input_dict(sm, "penaltyType"),
                    period_cap:                             PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor:                           PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
                    premium_discount_at_ied:                PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIed"),
                    price_at_purchase_date:                 PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    price_at_termination_date:              PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    purchase_date:                          PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    rate_multiplier:                        RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    rate_spread:                            RateSpread::provide_from_input_dict(sm, "rateSpread"),
                    scaling_effect:                         ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
                    scaling_index_at_contract_deal_date:    ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
                    status_date:                            StatusDate::provide_from_input_dict(sm, "statusDate"),
                    termination_date:                       TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    ..Default::default()
                };

                Ok(cm)
            },
            "SWAPS" => {

                let contract_role = ContractRole::provide(sm, "contractRole");

                let v = sm.get("contractStructure").unwrap().as_vec().unwrap();
                let contract_structure: Vec<ContractReference> = v.iter().map(|d| {
                    ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap())
                }).collect();

                let cm = ContractModel {
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
                    contract_type: ct_str.clone().to_string(),
                    contract_structure: contract_structure,
                    ..Default::default()
                };

                Ok(cm)
            },
            "STK" => {

                //let purchase_date = IsoDatetime::provide(sm, "purchaseDate");
                let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");

                let calendar = Calendar::provide_rc(sm, "calendar");

                //VERIFIER PAS PRESENT DANS LA LISTE DES TERMES
                let cycle_of_dividend_payment = CommonUtils::provide_string(sm, "cycleOfDividendPayment");


                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone.expect("df")
                    )
                } else {
                    None
                };

                let cycle_anchor_date_of_dividend_payment = {
                    let a = if cycle_of_dividend_payment.is_none() {
                        None
                    } else {
                        let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
                        CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).ok()
                    };
                    let b = CycleAnchorDateOfDividendPayment::provide_from_input_dict(sm, "CycleAnchorDateOfDividendPayment");;
                    if b.is_none() { a } else { b }
                };

                let cm = ContractModel {
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    quantity: Quantity::provide_from_input_dict(sm, "quantity"),
                    purchase_date: purchase_date,
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    cycle_of_dividend_payment: cycle_of_dividend_payment,
                    cycle_anchor_date_of_dividend_payment: cycle_anchor_date_of_dividend_payment,
                    market_object_code_of_dividends: MarketObjectCodeOfDividends::provide_from_input_dict(sm, "marketObjectCodeOfDividends"),
                    ..Default::default()
                };

                Ok(cm)
            },
            "LAM" => {
                let calendar = Calendar::provide_rc(sm, "calendar");
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");

                let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
                let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
                    None
                } else {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfFee::from_str(&a).ok()
                };


                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    //IsoDatetime::provide(sm, "initialExchangeDate")
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()

                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention =
                    if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
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

                let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality");
                let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfOptionality::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
                };

                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset")
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.expect("ere")
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
                    cycle_point_of_rate_reset: cycle_point_of_rate_reset,
                    cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    premium_discount_at_ied: PremiumDiscountAtIed::provide_from_input_dict(sm, "premiumDiscountAtIed"),
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
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
                    fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    maturity_date: maturity_date,
                    ..Default::default()
                };

                Ok(cm)
            },
            "ANN" => {
                // Déclarations simples sans dépendances
                let calendar = Calendar::provide_rc(sm, "calendar");
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");

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

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
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



                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
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
                let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
                    let a = initial_exchange_date.value().to_string();
                    CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: DayCountConvention::provide_from_input_dict(sm, "dayCountConvention"),
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
                    cycle_point_of_rate_reset: CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset"),
                    cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIed"),
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
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
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

                Ok(cm)
            },
            "LAX" => {
                // Déclarations simples sans dépendances
                let calendar = Calendar::provide_rc(sm, "calendar");

                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");

                // Champs qui dépendent d'autres champs
                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
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


                let b = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
                let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
                    let a = initial_exchange_date.value().to_string();
                    CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIed"),
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
                    contract_type: ct_str.clone().to_string(),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
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


                Ok(cm)
            },
            "SWPPV" => {
     
                let calendar = Calendar::provide_rc(sm, "calendar");

                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
 

                // Champs qui dépendent d'autres champs
                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };


                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
                };

                let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment");
                let cycle_point_of_rate_reset = if let Some(point) = &cycle_point_of_interest_payment {
                    if point.to_string() == "B" {
                        Some(CyclePointOfRateReset::new("E").expect("d"))
                    } else {
                        CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                    }
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    accrued_interest2: AccruedInterest2::provide_from_input_dict(sm, "accruedInterest2"),
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    nominal_interest_rate2: NominalInterestRate2::provide_from_input_dict(sm, "nominalInterestRate2"),
                    day_count_convention: day_count_convention,
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    maturity_date: MaturityDate::provide_from_input_dict(sm, "maturityDate"),
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
                    cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
                    rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
                    market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
                    cycle_point_of_rate_reset: cycle_point_of_rate_reset,
                    cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
                    fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
                    ..Default::default()
                };


                Ok(cm)
            },
            "FXOUT" => {
                // Déclarations simples sans dépendances
                let calendar = Calendar::provide_rc(sm, "calendar");

                // Gestion des dépendances
                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    currency2: Currency2::provide_from_input_dict(sm, "currency2"),
                    maturity_date: MaturityDate::provide_from_input_dict(sm, "maturityDate"),
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    notional_principal2: NotionalPrincipal2::provide_from_input_dict(sm, "notionalPrincipal2"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    ..Default::default()
                };


                Ok(cm)
            },
            "COM" => {

                let cm = ContractModel {
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    quantity: Quantity::provide_from_input_dict(sm, "quantity"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
                    ..Default::default()
                };


                Ok(cm)
            },
            "CSH" => {

                let cm = ContractModel {
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    ..Default::default()
                };

                Ok(cm)
            },
            "UMP" => {
                // Déclarations simples sans dépendances
                let calendar = Calendar::provide_rc(sm, "calendar");



                let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
                let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfFee::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
                };


                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&IsoDatetime::provide_rc(sm, "maturityDate"), &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");

                let cm = ContractModel {
                    calendar: Calendar::provide_from_input_dict(sm, "calendar"),
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
                    cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
                    cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
                    rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
                    market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
                    fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
                    maturity_date: maturity_date,
                    ..Default::default()
                };


                Ok(cm)
            },
            "CLM" => {
                let calendar = Calendar::provide_rc(sm, "calendar");

                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");


                let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
                let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfFee::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
                };

                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };


                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    maturity_date: maturity_date,
                    x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
                    cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
                    cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
                    rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
                    market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
                    fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
                    ..Default::default()
                };


                Ok(cm)
            },
            "NAM" => {
                let calendar = Calendar::provide_rc(sm, "calendar");

                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");


                let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
                let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfFee::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
                };

                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
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

                let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
                let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfRateReset::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
                    cycle_point_of_rate_reset: cycle_point_of_rate_reset,
                    cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
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
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
                    period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
                    fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
                    next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
                    rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
                    maturity_date: maturity_date,
                    credit_event_type_covered: credit_event_type_covered,
                    ..Default::default()
                };


                Ok(cm)
            },
            "CEC" => {
                let calendar = Calendar::provide_rc(sm, "calendar");

                let contract_role = ContractRole::provide(sm, "contractRole");
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");


                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };

                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contract_structure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
                            .collect();
                        Some(contract_structure)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: contract_role,
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
                    grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
                    delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
                    delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
                    guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
                    coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
                    credit_event_type_covered: credit_event_type_covered,
                    cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    maturity_date: maturity_date,
                    notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
                    exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
                    cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
                    next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
                    ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
                    contract_structure: contract_structure,
                    ..Default::default()
                };


                Ok(cm)
            },
            "CEG" => {
                // Déclarations simples sans dépendances
                let calendar = Calendar::provide_rc(sm, "calendar");

                let contract_role = ContractRole::provide(sm, "contractRole");


                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };

                let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
                let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
                    let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                    CycleAnchorDateOfInterestPayment::from_str(&a).ok()
                } else {
                    CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
                };

                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&IsoDatetime::provide_rc(sm, "maturityDate"), &calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contract_structure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
                            .collect();
                        Some(contract_structure)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let cm = ContractModel {
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: contract_role,
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
                    grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
                    delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
                    delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
                    guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
                    coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
                    credit_event_type_covered: credit_event_type_covered,
                    cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
                    cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
                    next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
                    ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
                    cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
                    fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
                    fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    day_count_convention: day_count_convention,
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
                    exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    contract_structure: contract_structure,
                    ..Default::default()
                };


                Ok(cm)
            },
            "FUTUR" => {
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
                let calendar = Calendar::provide_rc(sm, "calendar");


                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };


                let cycle_anchor_date_of_interest_payment = if CommonUtils::provide_string(sm, "cycleOfInterestPayment").is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let cm = ContractModel {
                    maturity_date: maturity_date,
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    seniority: Seniority::provide_from_input_dict(sm, "seniority"),
                    non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
                    prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
                    grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
                    delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
                    delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
                    guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
                    coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
                    credit_event_type_covered: credit_event_type_covered,
                    cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
                    cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
                    next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
                    ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
                    cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
                    exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    futures_price: FuturesPrice::provide_from_input_dict(sm, "futuresPrice"),
                    ..Default::default()
                };


                Ok(cm)
            },
            "BCS" => {
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
                let calendar = Calendar::provide_rc(sm, "calendar");
                let contract_role = ContractRole::provide(sm, "contractRole");
                let purchase_date = IsoDatetime::provide(sm, "purchaseDate");

                let boundary_monitoring_anchor_date = if let Some(boundary_monitoring_anchor_date) = IsoDatetime::provide(sm, "boundaryMonitoringAnchorDate") {
                    Some(boundary_monitoring_anchor_date)
                } else {
                    purchase_date.clone()
                };
                let boundary_monitoring_end_date = if let Some(boundary_monitoring_end_date) = IsoDatetime::provide(sm, "boundaryMonitoringEndDate") {
                    Some(boundary_monitoring_end_date)
                } else {
                    maturity_date.clone().map(|rc| (*rc).clone())
                };


                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };


                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contract_structure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
                            .collect();
                        Some(contract_structure)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let cm = ContractModel {
                    maturity_date: maturity_date,
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: contract_role,
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    seniority: Seniority::provide_from_input_dict(sm, "seniority"),
                    non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
                    prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
                    grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
                    delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
                    delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
                    guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
                    coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
                    credit_event_type_covered: credit_event_type_covered,
                    cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
                    cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
                    next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
                    ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
                    cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    boundary_value: BoundaryValue::provide_from_input_dict(sm, "boundaryValue"),
                    boundary_direction: BoundaryDirection::provide_from_input_dict(sm, "boundaryDirection"),
                    boundary_effect: BoundaryEffect::provide_from_input_dict(sm, "boundaryEffect"),
                    boundary_leg_initially_active: BoundaryLegInitiallyActive::provide_from_input_dict(sm, "boundaryLegInitiallyActive"),
                    boundary_monitoring_anchor_date: boundary_monitoring_anchor_date,
                    boundary_monitoring_end_date: boundary_monitoring_end_date,
                    boundary_monitoring_cycle: BoundaryMonitoringCycle::provide_from_input_dict(sm, "boundaryMonitoringCycle"),
                    boundary_crossed_flag: BoundaryCrossedFlag::provide_from_input_dict(sm, "boundaryCrossedFlag"),
                    contract_structure: contract_structure,
                    ..Default::default()
                };


                Ok(cm)
            },
            "OPTNS" => {
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
                let calendar = Calendar::provide_rc(sm, "calendar");
                let contract_role = ContractRole::provide(sm, "contractRole");


                let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
                let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
                    Some(CreditEventTypeCovered::default())
                } else {
                    credit_event_type_covered_tmp
                };

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                        BusinessDayAdjuster::provide(
                        sm,
                        "businessDayAdjuster",
                        calendar_clone.unwrap()
                    )
                } else {
                    None
                };

                let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contract_structure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
                            .collect();
                        Some(contract_structure)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let cm = ContractModel {
                    maturity_date: maturity_date,
                    calendar: calendar,
                    business_day_adjuster: business_day_adjuster,
                    end_of_month_convention: EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention"),
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
                    creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
                    seniority: Seniority::provide_from_input_dict(sm, "seniority"),
                    non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
                    prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
                    grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
                    delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
                    delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
                    guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
                    coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
                    credit_event_type_covered: credit_event_type_covered,
                    cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
                    cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
                    next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
                    ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
                    cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
                    cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
                    cycle_anchor_date_of_interest_payment: CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
                    array_cycle_anchor_date_of_interest_payment: ArrayCycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "arrayCycleAnchorDateOfInterestPayment"),
                    cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
                    array_cycle_of_interest_payment: ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment"),
                    nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
                    exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
                    exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    option_type: OptionType::provide_from_input_dict(sm, "optionType"),
                    option_strike1: OptionStrike1::provide_from_input_dict(sm, "optionStrike1"),
                    option_strike2: OptionStrike2::provide_from_input_dict(sm, "optionStrike2"),
                    contract_structure: contract_structure,
                    ..Default::default()
                };


                Ok(cm)
            },
            "CAPFL" => {

                let contract_role = ContractRole::provide(sm, "contractRole");

                let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contract_structure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
                            .collect();
                        Some(contract_structure)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let cm = ContractModel {
                    contract_type: ct_str.clone().to_string(),
                    status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
                    contract_role: contract_role,
                    contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
                    counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
                    market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
                    currency: Currency::provide_from_input_dict(sm, "currency"),
                    purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
                    price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
                    termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
                    price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
                    life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
                    life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
                    contract_structure: contract_structure,
                    ..Default::default()
                };

                Ok(cm)
            },
            _ => Err("test erreur".to_string()),
        }
    }
}
