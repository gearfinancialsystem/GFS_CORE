use std::collections::HashMap;
use std::rc::Rc;
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
use crate::terms::grp_notional_principal::ArrayCycleAnchorDateOfPrincipalRedemption::ArrayCycleAnchorDateOfPrincipalRedemtion;
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
use crate::types::isoDatetime::{TraitNaiveDateTimeExtension, IsoDatetime};
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
    pub array_cycle_anchor_date_of_principal_redemption: Option<ArrayCycleAnchorDateOfPrincipalRedemtion>,
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
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_type: Option<ContractType>,
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
    pub next_dividend_payment_amount: Option<NextDividendPaymentAmount>,
    pub next_principal_redemption_payment: Option<NextPrincipalRedemptionPayment>,
    pub next_reset_rate: Option<NextResetRate>,
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
            "calendar" => Some(FieldValue::vCalendar(self.calendar.clone().unwrap())) , // pas d'option, champs obligatoire
            "businessDayAdjuster" => Some(FieldValue::vBusinessDayAdjuster(self.businessDayAdjuster.clone().unwrap())),
            "endOfMonthConvention" => Some(FieldValue::vEndOfMonthConvention(self.endOfMonthConvention?)),
            "contractType" => Some(FieldValue::Vstring(self.contractType.clone().unwrap())), // obligatoire
            "contractID" => Some(FieldValue::Vstring(self.contractID.clone().unwrap())),
            "statusDate" => Some(FieldValue::vIsoDatetime(self.statusDate?)),
            "contractRole" => Some(FieldValue::vContractRole(self.contractRole.clone().unwrap())),
            "counterpartyID" => Some(FieldValue::Vstring(self.counterpartyID.clone().unwrap())),
            "creatorID" => Some(FieldValue::Vstring(self.creatorID.clone().unwrap())),
            "marketObjectCode" => Some(FieldValue::Vstring(self.marketObjectCode.clone().unwrap())),
            "cycleAnchorDateOfFee" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfFee?)),
            "cycleAnchorDateOfDividend" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfDividend?)),
            "cycleOfFee" => Some(FieldValue::Vstring(self.cycleOfFee.clone().unwrap())),
            "feeBasis" => Some(FieldValue::vFeeBasis(self.feeBasis.clone().unwrap())),
            "feeRate" => Some(FieldValue::vF64(self.feeRate?)),
            "feeAccrued" => Some(FieldValue::vF64(self.feeAccrued?)),
            "cycleAnchorDateOfInterestPayment" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfInterestPayment?)),
            "cycleOfInterestPayment" => Some(FieldValue::Vstring(self.cycleOfInterestPayment.clone().unwrap())),
            "nominalInterestRate" => Some(FieldValue::vF64(self.nominalInterestRate?)),
            "dayCountConvention" => Some(FieldValue::vDayCountConvention(self.dayCountConvention.clone().unwrap())),
            "accruedInterest" => Some(FieldValue::vF64(self.accruedInterest?)),
            "capitalizationEndDate" => Some(FieldValue::vIsoDatetime(self.capitalizationEndDate?)),
            "cyclePointOfInterestPayment" =>Some(FieldValue::vCyclePointOfInterestPayment(self.cyclePointOfInterestPayment.clone().unwrap())),
            "currency" => Some(FieldValue::Vstring(self.currency.clone().unwrap())), // obligatoire
            "initialExchangeDate" => Some(FieldValue::vIsoDatetime(self.initialExchangeDate?)),
            "premiumDiscountAtIED" => Some(FieldValue::vF64(self.premiumDiscountAtIED?)),
            "notionalPrincipal" => Some(FieldValue::vF64(self.notionalPrincipal?)),
            "purchaseDate" => Some(FieldValue::vIsoDatetime(self.purchaseDate?)),
            "priceAtPurchaseDate" => Some(FieldValue::vF64(self.priceAtPurchaseDate?)),
            "terminationDate" => Some(FieldValue::vIsoDatetime(self.terminationDate?)),
            "priceAtTerminationDate" => Some(FieldValue::vF64(self.priceAtTerminationDate?)),
            "marketObjectCodeOfScalingIndex" => Some(FieldValue::Vstring(self.marketObjectCodeOfScalingIndex.clone().unwrap())),
            "seniority" => Some(FieldValue::vSeniority(self.seniority.clone().unwrap())),
            "scalingIndexAtContractDealDate" => Some(FieldValue::vF64(self.scalingIndexAtContractDealDate?)),
            "notionalScalingMultiplier" => Some(FieldValue::vF64(self.notionalScalingMultiplier?)),
            "interestScalingMultiplier" => Some(FieldValue::vF64(self.interestScalingMultiplier?)),
            "cycleAnchorDateOfScalingIndex" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfScalingIndex?)),
            "cycleOfScalingIndex" => Some(FieldValue::Vstring(self.cycleOfScalingIndex.clone().unwrap())),
            "scalingEffect" => Some(FieldValue::vScalingEffect(self.scalingEffect.clone().unwrap())),
            // TODO=> review prepayment mechanism and attributes
            "cycleAnchorDateOfOptionality" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfOptionality?)),
            "cycleOfOptionality" => Some(FieldValue::Vstring(self.cycleOfOptionality.clone().unwrap())),
            "penaltyType" => Some(FieldValue::vPenaltyType(self.penaltyType.clone().unwrap())),
            "penaltyRate" => Some(FieldValue::vF64(self.penaltyRate?)),
            "objectCodeOfPrepaymentModel" => Some(FieldValue::Vstring(self.objectCodeOfPrepaymentModel.clone().unwrap())),
            "cycleAnchorDateOfRateReset" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfRateReset?)),
            "cycleOfRateReset" => Some(FieldValue::Vstring(self.cycleOfRateReset.clone().unwrap())),
            "rateSpread" => Some(FieldValue::vF64(self.rateSpread?)),
            "marketObjectCodeOfRateReset" => Some(FieldValue::Vstring(self.marketObjectCodeOfRateReset.clone().unwrap())),
            "lifeCap" => Some(FieldValue::vF64(self.lifeCap?)),
            "lifeFloor" => Some(FieldValue::vF64(self.lifeFloor?)),
            "periodCap" => Some(FieldValue::vF64(self.periodCap?)),
            "periodFloor" => Some(FieldValue::vF64(self.periodFloor?)),
            "cyclePointOfRateReset" => Some(FieldValue::vCyclePointOfRateReset(self.cyclePointOfRateReset.clone().unwrap())),
            "fixingPeriod" => Some(FieldValue::vIsoPeriod(self.fixingPeriod.clone().unwrap())),
            "nextResetRate" => Some(FieldValue::vF64(self.nextResetRate?)),
            "rateMultiplier" => Some(FieldValue::vF64(self.rateMultiplier?)),
            "maturityDate" =>Some(FieldValue::vMaturityDate(self.maturityDate.clone().unwrap())),
            "contractPerformance" => Some(FieldValue::vContractPerformance(self.contractPerformance?)),
            "deliverySettlement" => Some(FieldValue::vDeliverySettlement(self.deliverySettlement.clone().unwrap())),
            "quantity" => Some(FieldValue::vF64(self.quantity.clone().unwrap())),
            "marketValueObserved" => Some(FieldValue::vF64(self.marketValueObserved.clone().unwrap())),
            "cycleOfDividendPayment" => Some(FieldValue::Vstring(self.cycleOfDividendPayment.clone().unwrap())),
            "cycleAnchorDateOfDividendPayment" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfDividendPayment.clone().unwrap())),
            "marketObjectCodeOfDividends" => Some(FieldValue::Vstring(self.marketObjectCodeOfDividends.clone().unwrap())),
            "nonPerformingDate" => Some(FieldValue::vIsoDatetime(self.nonPerformingDate.clone().unwrap())),
            "prepaymentPeriod" => Some(FieldValue::vIsoPeriod(self.prepaymentPeriod.clone().unwrap())),
            "gracePeriod" => Some(FieldValue::vIsoPeriod(self.gracePeriod.clone().unwrap())),
            "delinquencyPeriod" => Some(FieldValue::vIsoPeriod(self.delinquencyPeriod.clone().unwrap())),
            "delinquencyRate" => Some(FieldValue::vF64(self.delinquencyRate.clone().unwrap())),
            "guaranteedExposure" => Some(FieldValue::vGuaranteedExposure(self.guaranteedExposure.clone().unwrap())),
            "coverageOfCreditEnhancement" =>Some(FieldValue::vF64(self.coverageOfCreditEnhancement.clone().unwrap())),
            "cycleOfDividend" =>Some(FieldValue::Vstring(self.cycleOfDividend.clone().unwrap())),
            "nextDividendPaymentAmount" =>Some(FieldValue::vF64(self.nextDividendPaymentAmount.clone().unwrap())),
            "exDividendDate" => Some(FieldValue::vIsoDatetime(self.exDividendDate.clone().unwrap())),
            "arrayCycleAnchorDateOfInterestPayment" =>Some(FieldValue::vVecIsoDatetime(self.arrayCycleAnchorDateOfInterestPayment.clone().unwrap())),
            "arrayCycleOfInterestPayment" =>Some(FieldValue::vVecString(self.arrayCycleOfInterestPayment.clone().unwrap())),
            "exerciseAmount" =>Some(FieldValue::vF64(self.exerciseAmount.clone().unwrap())),
            "settlementPeriod" =>Some(FieldValue::vIsoPeriod(self.settlementPeriod.clone().unwrap())),
            "exerciseDate" => Some(FieldValue::vIsoDatetime(self.exerciseDate.clone().unwrap())),
            "optionType" =>Some(FieldValue::vOptionType(self.optionType.clone().unwrap())),
            "optionStrike1" =>Some(FieldValue::vF64(self.optionStrike1.clone().unwrap())),
            "optionStrike2" =>Some(FieldValue::vF64(self.optionStrike2.clone().unwrap())),
            "xDayNotice"=>Some(FieldValue::vIsoPeriod(self.xDayNotice.clone().unwrap())),
            "cycleAnchorDateOfInterestCalculationBase"=>Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfInterestCalculationBase.clone().unwrap())),
            "cycleOfInterestCalculationBase"=>Some(FieldValue::Vstring(self.cycleOfInterestCalculationBase.clone().unwrap())),
            "interestCalculationBase"=>Some(FieldValue::vInterestCalculationBase(self.interestCalculationBase.clone().unwrap())),
            "interestCalculationBaseAmount" =>Some(FieldValue::vF64(self.interestCalculationBaseAmount.clone().unwrap())),
            "cycleAnchorDateOfPrincipalRedemption"=>Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfPrincipalRedemption.clone().unwrap())),
            "cycleOfPrincipalRedemption"=>Some(FieldValue::Vstring(self.cycleOfPrincipalRedemption.clone().unwrap())),
            "nextPrincipalRedemptionPayment" =>Some(FieldValue::vF64(self.nextPrincipalRedemptionPayment.clone().unwrap())),
            "amortizationDate"=>Some(FieldValue::vIsoDatetime(self.amortizationDate.clone().unwrap())),
            "boundaryValue" => Some(FieldValue::vF64(self.boundaryValue.clone().unwrap())),
            "boundaryDirection"=>Some(FieldValue::vBoundaryDirection(self.boundaryDirection.clone().unwrap())),
            "boundaryEffect"=>Some(FieldValue::vBoundaryEffect(self.boundaryEffect.clone().unwrap())),
            "boundaryLegInitiallyActive"=>Some(FieldValue::vBoundaryLegInitiallyActive(self.boundaryLegInitiallyActive.clone().unwrap())),
            "boundaryMonitoringAnchorDate" => Some(FieldValue::vIsoDatetime(self.boundaryMonitoringAnchorDate.clone().unwrap())),
            "boundaryMonitoringEndDate" => Some(FieldValue::vIsoDatetime(self.boundaryMonitoringEndDate.clone().unwrap())),
            "boundaryMonitoringCycle"=>Some(FieldValue::Vstring(self.boundaryMonitoringCycle.clone().unwrap())),
            "boundaryCrossedFlag"=>Some(FieldValue::vBool(self.boundaryCrossedFlag.clone().unwrap())),

            "arrayCycleAnchorDateOfPrincipalRedemption" => Some(FieldValue::vVecIsoDatetime(self.arrayCycleAnchorDateOfPrincipalRedemption.clone().unwrap())),
            "arrayCycleOfPrincipalRedemption" => Some(FieldValue::vVecString(self.arrayCycleOfPrincipalRedemption.clone().unwrap())),
            "arrayNextPrincipalRedemptionPayment" => Some(FieldValue::vVecF64(self.arrayNextPrincipalRedemptionPayment.clone().unwrap())),
            "arrayIncreaseDecrease" => Some(FieldValue::vArrayIncreaseDecrease(self.arrayIncreaseDecrease.clone().unwrap())),
            "arrayCycleAnchorDateOfRateReset" => Some(FieldValue::vVecIsoDatetime(self.arrayCycleAnchorDateOfRateReset.clone().unwrap())),
            "arrayCycleOfRateReset" => Some(FieldValue::vVecString(self.arrayCycleOfRateReset.clone().unwrap())),
            "arrayRate" => Some(FieldValue::vVecF64(self.arrayRate.clone().unwrap())),
            "arrayFixedVariable" => Some(FieldValue::vArrayFixedVariable(self.arrayFixedVariable.clone().unwrap())),
            "accruedInterest2" =>Some(FieldValue::vF64(self.accruedInterest2.clone().unwrap())),
            "nominalInterestRate2" =>Some(FieldValue::vF64(self.nominalInterestRate2.clone().unwrap())),
            "currency2" =>Some(FieldValue::Vstring(self.currency2.clone().unwrap())),
            "notionalPrincipal2" =>Some(FieldValue::vF64(self.notionalPrincipal2.clone().unwrap())),
            "creditEventTypeCovered"=>Some(FieldValue::vVecCreditEventTypeCovered(self.creditEventTypeCovered.clone().unwrap())),
            "futuresPrice" =>Some(FieldValue::vF64(self.futuresPrice.clone().unwrap())),
            "settlementCurrency"=>Some(FieldValue::Vstring(self.settlementCurrency.clone().unwrap())),
            _ => None,
        }
    }

    pub fn new(sm: &HashMap<String, Value>) -> Result<ContractModel, String> {
        let ct = sm.get("contractType").unwrap();
        match ct.as_string().unwrap().as_str() {
            "PAM" => {
                //let mut cm = ContractModel::init();
                let maturity_date = IsoDatetime::provide_rc(sm, "maturityDate");
                let calendar = Calendar::provide_rc(sm, "calendar");

                let business_day_adjuster = if let Some(calendar) = &calendar {
                    // Clone seulement l'Rc, pas le calendrier lui-même
                    let calendar_clone = Some(Rc::clone(calendar));
                    BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone.expect("te")
                    );
                };

                // Clonez simplement les Rc existantes
                let day_count_convention = if let (Some(maturity_date), Some(calendar)) = (&maturity_date, &calendar) {
                     DayCountConvention::provide(
                        sm,
                        "dayCountConvention",
                        Some(Rc::clone(maturity_date)),
                        Some(Rc::clone(calendar))
                    );
                };
                
                let cm = ContractModel {
                    accrued_interest:                       Some(CommonUtils::provide_f64default(sm, "accruedInterest", 0.0)),
                    capitalization_end_date:                Some(IsoDatetime::provide(sm, "capitalizationEndDate")),
                    contract_id:                        Some(CommonUtils::provide_string(sm, "contractID")),
                    contract_performance: Some(ContractPerformance::provide(sm, "contractPerformance")),
                    contract_role: Some(ContractRole::provide(sm, "contractRole")),
                    contract_type: Some(CommonUtils::provide_string(sm, "contractType")),
                    counterparty_id: Some(CommonUtils::provide_string(sm, "counterpartyID")),
                    currency: Some(CommonUtils::provide_string(sm, "currency")),
                    cycle_anchor_date_of_fee: Some(IsoDatetime::provide(sm, "cycleAnchorDateOfFee")),
                    cycle_anchor_date_of_interest_payment: Some(IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")),
                    cycle_anchor_date_of_optionality: Some(IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality")),
                    cycle_anchor_date_of_rate_reset: Some(IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")),
                    cycle_anchor_date_of_scaling_index: Some(IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex")),
                    cycle_of_fee: Some(CommonUtils::provide_string(sm, "cycleOfFee")),
                    cycle_of_interest_payment: Some(CommonUtils::provide_string(sm, "cycleOfInterestPayment")),
                    cycle_of_optionality: Some(CommonUtils::provide_string(sm, "cycleOfOptionality")),
                    cycle_of_rate_reset: Some(CommonUtils::provide_string(sm, "cycleOfRateReset")),
                    cycle_of_scaling_index: Some(CommonUtils::provide_string(sm, "cycleOfScalingIndex")),
                    cycle_point_of_interest_payment: Some(CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment")),
                    cycle_point_of_rate_reset: Some(CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")),
                    end_of_month_convention: Some(EndOfMonthConvention::provide(sm, "endOfMonthConvention")),
                    fee_accrued: Some(CommonUtils::provide_f64default(sm, "feeAccrued", 0.0)),
                    fee_basis: Some(FeeBasis::provide(sm, "feeBasis")),
                    fee_rate: Some(CommonUtils::provide_f64default(sm, "feeRate", 0.0)),
                    fixing_period: Some(IsoPeriod::provide(sm, "fixingPeriod")),
                    initial_exchange_date: Some(IsoDatetime::provide(sm, "initialExchangeDate")),
                    interest_scaling_multiplier: Some(CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0)),
                    life_cap: Some(CommonUtils::provide_f64(sm, "lifeCap")),
                    life_floor: Some(CommonUtils::provide_f64(sm, "lifeFloor")),
                    market_object_code: Some(CommonUtils::provide_string(sm, "marketObjectCode")),
                    market_object_code_of_rate_reset: Some(CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset")),
                    market_object_code_of_scaling_index: Some(CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex")),
                    next_reset_rate: Some(CommonUtils::provide_f64(sm, "nextResetRate")),
                    nominal_interest_rate: Some(CommonUtils::provide_f64(sm, "nominalInterestRate")),
                    notional_principal: Some(CommonUtils::provide_f64(sm, "notionalPrincipal")),
                    notional_scaling_multiplier: Some(CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0)),
                    object_code_of_prepayment_model: Some(CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel")),
                    penalty_rate: Some(CommonUtils::provide_f64default(sm, "penaltyRate", 0.0)),
                    penalty_type: Some(PenaltyType::provide(sm, "penaltyType")),
                    period_cap: Some(CommonUtils::provide_f64(sm, "periodCap")),
                    period_floor: Some(CommonUtils::provide_f64(sm, "periodFloor")),
                    premium_discount_at_ied: Some(CommonUtils::provide_f64(sm, "premiumDiscountAtIED")),
                    price_at_purchase_date: Some(CommonUtils::provide_f64(sm, "priceAtPurchaseDate")),
                    price_at_termination_date: Some(CommonUtils::provide_f64(sm, "priceAtTerminationDate")),
                    purchase_date: Some(IsoDatetime::provide(sm, "purchaseDate")),
                    rate_multiplier: Some(CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0)),
                    rate_spread: Some(CommonUtils::provide_f64default(sm, "rateSpread", 0.0)),
                    scaling_effect: Some(ScalingEffect::provide(sm, "scalingEffect")),
                    scaling_index_at_contract_deal_date: Some(CommonUtils::provide_f64(sm, "scalingIndexAtContractDealDate")),
                    status_date: Some(IsoDatetime::provide(sm, "statusDate")),
                    termination_date: Some(IsoDatetime::provide(sm, "terminationDate")),
                    ..Default::default()
                };

                Ok(cm)
            },
            "SWAPS" => {
                let mut cm = ContractModel::init();
                cm.contractID =CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode=CommonUtils::provide_string(sm, "marketObjectCode");
                cm.currency=CommonUtils::provide_string(sm, "currency");
                cm.purchaseDate=IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate=CommonUtils::provide_f64(sm, "priceAtPurchaseDate");
                cm.terminationDate=IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate=CommonUtils::provide_f64(sm, "priceAtTerminationDate");
                cm.deliverySettlement = DeliverySettlement::provide(sm, "deliverySettlement");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                
                let v = sm.get("contractStructure").unwrap().as_vec().unwrap() ;
                //let d1 = v.get(0).unwrap();
                //let r = ContractReference::new(&d1, &cm.contractRole.clone().unwrap());

                let a: Vec<ContractReference> = v.iter().map(|d| {
                    ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap())
                }).collect();
                println!("{:?}", a.get(0).unwrap().object.as_cm().unwrap().initialExchangeDate);
                println!("{:?}", a.get(1).unwrap().object.as_cm().unwrap().initialExchangeDate);
                cm.contractStructure = Some(a);
                
                
                //

                Ok(cm)
            },
            "STK" => {
                let mut cm = ContractModel::init();
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.quantity = CommonUtils::provide_f64default(sm, "quantity", 1.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.marketValueObserved = CommonUtils::provide_f64default(sm, "marketValueObserved", 0.0);

                // present for STK but not COM
                cm.calendar = Calendar::provide_rc(sm, "calendar");
                if let Some(calendar) = &cm.calendar {
                    // Clone seulement l'Rc, pas le calendrier lui-même
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone.expect("df")
                    );
                }
                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.cycleOfDividendPayment =  CommonUtils::provide_string(sm, "cycleOfDividendPayment");

                let a = if cm.cycleOfDividendPayment.is_none() {
                    None
                } else {
                    cm.purchaseDate.clone()
                };
                let b = IsoDatetime::provide(sm, "cycleAnchorDateOfDividendPayment");

                cm.cycleAnchorDateOfDividendPayment = if b.is_none() {
                    a
                } else {
                    b
                };

                cm.marketObjectCodeOfDividends =  CommonUtils::provide_string(sm, "marketObjectCodeOfDividends");
                Ok(cm)
            },
            "LAM" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.expect("ere"));
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    None
                } else {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new("B").unwrap()) {
                    Some(CyclePointOfRateReset::new("E").expect("f"))
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.premiumDiscountAtIED = CommonUtils::provide_f64default(sm, "premiumDiscountAtIED", 0.0);
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.marketObjectCodeOfScalingIndex = CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex");
                cm.scalingIndexAtContractDealDate = CommonUtils::provide_f64default(sm, "scalingIndexAtContractDealDate", 0.0);
                cm.notionalScalingMultiplier = CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0);
                cm.interestScalingMultiplier = CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0);

                cm.cycleAnchorDateOfScalingIndex = if cm.cycleOfScalingIndex.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex")
                };

                cm.cycleOfScalingIndex = CommonUtils::provide_string(sm, "cycleOfScalingIndex");
                cm.scalingEffect = ScalingEffect::provide(sm, "scalingEffect");

                cm.cycleAnchorDateOfOptionality = if cm.cycleOfOptionality.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality")
                };

                cm.cycleOfOptionality = CommonUtils::provide_string(sm, "cycleOfOptionality");
                cm.penaltyType = PenaltyType::provide(sm, "penaltyType");
                cm.penaltyRate = CommonUtils::provide_f64default(sm, "penaltyRate", 0.0);
                cm.objectCodeOfPrepaymentModel = CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                Ok(cm)
            },
            "ANN" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfFee")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new("B").unwrap()) {
                    Some(CyclePointOfRateReset::new("E").expect("fe"))
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.premiumDiscountAtIED = CommonUtils::provide_f64default(sm, "premiumDiscountAtIED", 0.0);
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.marketObjectCodeOfScalingIndex = CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex");
                cm.scalingIndexAtContractDealDate = CommonUtils::provide_f64default(sm, "scalingIndexAtContractDealDate", 0.0);
                cm.notionalScalingMultiplier = CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0);
                cm.interestScalingMultiplier = CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0);

                cm.cycleAnchorDateOfScalingIndex = if cm.cycleOfScalingIndex.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex")
                };

                cm.cycleOfScalingIndex = CommonUtils::provide_string(sm, "cycleOfScalingIndex");
                cm.scalingEffect = ScalingEffect::provide(sm, "scalingEffect");

                cm.cycleAnchorDateOfOptionality = if cm.cycleOfOptionality.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality")
                };

                cm.cycleOfOptionality = CommonUtils::provide_string(sm, "cycleOfOptionality");
                cm.penaltyType = PenaltyType::provide(sm, "penaltyType");
                cm.penaltyRate = CommonUtils::provide_f64default(sm, "penaltyRate", 0.0);
                cm.objectCodeOfPrepaymentModel = CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);

                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                cm.cycleAnchorDateOfInterestCalculationBase = if cm.cycleOfInterestCalculationBase.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestCalculationBase")
                };

                cm.cycleOfInterestCalculationBase = CommonUtils::provide_string(sm, "cycleOfInterestCalculationBase");

                cm.interestCalculationBase = if let Some(interestCalculationBase) = CommonUtils::provide_string(sm, "interestCalculationBase") {
                    if interestCalculationBase.is_empty() {
                        Some(InterestCalculationBase::new_NT())
                    } else {
                        InterestCalculationBase::provide(sm, "interestCalculationBase")
                    }
                } else {
                    Some(InterestCalculationBase::new_NT())
                };

                cm.interestCalculationBaseAmount = CommonUtils::provide_f64default(sm, "interestCalculationBaseAmount", 0.0);

                cm.cycleAnchorDateOfPrincipalRedemption = if let Some(initialExchangeDate) = IsoDatetime::provide(sm, "initialExchangeDate") {
                    Some(initialExchangeDate)
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfPrincipalRedemption")
                };

                cm.cycleOfPrincipalRedemption = CommonUtils::provide_string(sm, "cycleOfPrincipalRedemption");
                cm.nextPrincipalRedemptionPayment = CommonUtils::provide_f64(sm, "nextPrincipalRedemptionPayment");

                cm.amortizationDate = IsoDatetime::provide(sm, "amortizationDate");

                Ok(cm)
            },
            "LAX" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.premiumDiscountAtIED = CommonUtils::provide_f64default(sm, "premiumDiscountAtIED", 0.0);
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.arrayCycleAnchorDateOfPrincipalRedemption = IsoDatetime::provide_vec(sm, "arrayCycleAnchorDateOfPrincipalRedemption");
                cm.arrayCycleOfPrincipalRedemption = CommonUtils::provide_string_vec(sm, "arrayCycleOfPrincipalRedemption");
                cm.arrayNextPrincipalRedemptionPayment = CommonUtils::provide_f64_vec(sm, "arrayNextPrincipalRedemptionPayment");
                cm.arrayIncreaseDecrease = ArrayIncreaseDecrease::provide_vec(sm, "arrayIncreaseDecrease"); // to update
                cm.arrayCycleAnchorDateOfInterestPayment = IsoDatetime::provide_vec(sm, "arrayCycleAnchorDateOfInterestPayment");
                cm.arrayCycleOfInterestPayment = CommonUtils::provide_string_vec(sm, "arrayCycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.arrayCycleAnchorDateOfRateReset = IsoDatetime::provide_vec(sm, "arrayCycleAnchorDateOfRateReset");
                cm.arrayCycleOfRateReset = CommonUtils::provide_string_vec(sm, "arrayCycleOfRateReset");
                cm.arrayRate = CommonUtils::provide_f64_vec(sm, "arrayRate");
                cm.arrayFixedVariable = ArrayFixedVariable::provide_vec(sm, "arrayFixedVariable");
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");

                cm.cycleAnchorDateOfInterestCalculationBase = if cm.cycleOfInterestCalculationBase.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestCalculationBase")
                };

                cm.cycleOfInterestCalculationBase = CommonUtils::provide_string(sm, "cycleOfInterestCalculationBase");

                cm.interestCalculationBase = if let Some(interestCalculationBase) = CommonUtils::provide_string(sm, "interestCalculationBase") {
                    if interestCalculationBase.is_empty() {
                        Some(InterestCalculationBase::new_NT())
                    } else {
                        InterestCalculationBase::provide(sm, "interestCalculationBase")
                    }
                } else {
                    Some(InterestCalculationBase::new_NT())
                };

                cm.interestCalculationBaseAmount = CommonUtils::provide_f64default(sm, "interestCalculationBaseAmount", 0.0);

                cm.cycleAnchorDateOfPrincipalRedemption = if let Some(initialExchangeDate) = IsoDatetime::provide(sm, "initialExchangeDate") {
                    Some(initialExchangeDate)
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfPrincipalRedemption")
                };

                Ok(cm)
            },
            "SWPPV" => {
                let mut cm = ContractModel::init();

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.accruedInterest2 = CommonUtils::provide_f64default(sm, "accruedInterest2", 0.0);

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.creatorID = CommonUtils::provide_string(sm, "creatorID");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64(sm, "nominalInterestRate");
                cm.nominalInterestRate2 = CommonUtils::provide_f64(sm, "nominalInterestRate2");

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new("B").unwrap()) {
                    Some(CyclePointOfRateReset::new("E").expect("d"))
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.deliverySettlement = DeliverySettlement::provide(sm, "deliverySettlement");

                Ok(cm)
            },
            "FXOUT" => {
                let mut cm = ContractModel::init();
                cm.calendar = Calendar::provide_rc(sm, "calendar");
                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }
                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.currency2 = CommonUtils::provide_string(sm, "currency2");
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.notionalPrincipal2 = CommonUtils::provide_f64(sm, "notionalPrincipal2");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.deliverySettlement = DeliverySettlement::provide(sm, "deliverySettlement");
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");

                Ok(cm)
            },
            "COM" => {
                let mut cm = ContractModel::init();

                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.quantity = CommonUtils::provide_f64default(sm, "quantity", 1.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.marketValueObserved = CommonUtils::provide_f64default(sm, "marketValueObserved", 0.0);

                Ok(cm)
            },
            "CSH" => {
                let mut cm = ContractModel::init();

                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");

                Ok(cm)
            },
            "UMP" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfFee")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.xDayNotice = IsoPeriod::provide(sm, "xDayNotice");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");

                Ok(cm)
            },
            "CLM" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfFee")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.xDayNotice = IsoPeriod::provide(sm, "xDayNotice");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");

                Ok(cm)
            },
            "NAM" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfFee")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new("B").unwrap()) {
                    Some(CyclePointOfRateReset::new("E").expect("r"))
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.premiumDiscountAtIED = CommonUtils::provide_f64default(sm, "premiumDiscountAtIED", 0.0);
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.marketObjectCodeOfScalingIndex = CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex");
                cm.scalingIndexAtContractDealDate = CommonUtils::provide_f64default(sm, "scalingIndexAtContractDealDate", 0.0);
                cm.notionalScalingMultiplier = CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0);
                cm.interestScalingMultiplier = CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0);

                cm.cycleAnchorDateOfScalingIndex = if cm.cycleOfScalingIndex.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex")
                };

                cm.cycleOfScalingIndex = CommonUtils::provide_string(sm, "cycleOfScalingIndex");
                cm.scalingEffect = ScalingEffect::provide(sm, "scalingEffect");

                cm.cycleAnchorDateOfOptionality = if cm.cycleOfOptionality.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality")
                };

                cm.cycleOfOptionality = CommonUtils::provide_string(sm, "cycleOfOptionality");
                cm.penaltyType = PenaltyType::provide(sm, "penaltyType");
                cm.penaltyRate = CommonUtils::provide_f64default(sm, "penaltyRate", 0.0);
                cm.objectCodeOfPrepaymentModel = CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
                cm.fixingPeriod = IsoPeriod::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                cm.cycleAnchorDateOfInterestCalculationBase = if cm.cycleOfInterestCalculationBase.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestCalculationBase")
                };

                cm.cycleOfInterestCalculationBase = CommonUtils::provide_string(sm, "cycleOfInterestCalculationBase");

                cm.interestCalculationBase = if let Some(interestCalculationBase) = CommonUtils::provide_string(sm, "interestCalculationBase") {
                    if interestCalculationBase.is_empty() {
                        Some(InterestCalculationBase::new_NT())
                    } else {
                        InterestCalculationBase::provide(sm, "interestCalculationBase")
                    }
                } else {
                    Some(InterestCalculationBase::new_NT())
                };

                cm.interestCalculationBaseAmount = CommonUtils::provide_f64default(sm, "interestCalculationBaseAmount", 0.0);

                cm.cycleAnchorDateOfPrincipalRedemption = if let Some(initialExchangeDate) = IsoDatetime::provide(sm, "initialExchangeDate") {
                    Some(initialExchangeDate)
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfPrincipalRedemption")
                };

                cm.cycleOfPrincipalRedemption = CommonUtils::provide_string(sm, "cycleOfPrincipalRedemption");
                cm.nextPrincipalRedemptionPayment = CommonUtils::provide_f64(sm, "nextPrincipalRedemptionPayment");

                Ok(cm)
            },
            "CEC" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                };
                cm.creditEventTypeCovered = Some(b);

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap()))
                            .collect();
                        cm.contractStructure = Some(contract_structure);
                    }
                }

                Ok(cm)
            },
            "CEG" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.creatorID = CommonUtils::provide_string(sm, "creatorID");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                cm.nonPerformingDate = IsoDatetime::provide(sm, "nonPerformingDate");
                cm.gracePeriod = IsoPeriod::provide(sm, "gracePeriod");
                cm.delinquencyPeriod = IsoPeriod::provide(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc")  );
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                };
                cm.creditEventTypeCovered = Some(b);

                cm.cycleAnchorDateOfFee = if cm.cycleOfFee.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfFee")
                };

                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(calendar)))
                } else {
                    None
                };

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap()))
                            .collect();
                        cm.contractStructure = Some(contract_structure);
                    }
                }

                Ok(cm)
            },
            "FUTUR" => {
                let mut cm = ContractModel::init();

                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.creatorID = CommonUtils::provide_string(sm, "creatorID");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                cm.seniority = Seniority::provide(sm, "seniority");
                cm.nonPerformingDate = IsoDatetime::provide(sm, "nonPerformingDate");
                cm.prepaymentPeriod = IsoPeriod::provide(sm, "prepaymentPeriod");
                cm.gracePeriod = IsoPeriod::provide(sm, "gracePeriod");
                cm.delinquencyPeriod = IsoPeriod::provide(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                };
                cm.creditEventTypeCovered = Some(b);


                cm.cycleAnchorDateOfDividend = IsoDatetime::provide(sm, "cycleAnchorDateOfDividend");
                cm.cycleOfDividend = CommonUtils::provide_string(sm, "cycleOfDividend");
                cm.nextDividendPaymentAmount = CommonUtils::provide_f64default(sm, "nextDividendPaymentAmount", 0.0);
                cm.exDividendDate = IsoDatetime::provide(sm, "exDividendDate");
                cm.cycleAnchorDateOfFee = IsoDatetime::provide(sm, "cycleAnchorDateOfFee");
                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);
                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.futuresPrice = CommonUtils::provide_f64(sm, "futuresPrice");
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");

                Ok(cm)
            },
            "BCS" => {
                let mut cm = ContractModel::init();

                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.creatorID = CommonUtils::provide_string(sm, "creatorID");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                cm.seniority = Seniority::provide(sm, "seniority");
                cm.nonPerformingDate = IsoDatetime::provide(sm, "nonPerformingDate");
                cm.prepaymentPeriod = IsoPeriod::provide(sm, "prepaymentPeriod");
                cm.gracePeriod = IsoPeriod::provide(sm, "gracePeriod");
                cm.delinquencyPeriod = IsoPeriod::provide(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                };
                cm.creditEventTypeCovered = Some(b);


                cm.cycleAnchorDateOfDividend = IsoDatetime::provide(sm, "cycleAnchorDateOfDividend");
                cm.cycleOfDividend = CommonUtils::provide_string(sm, "cycleOfDividend");
                cm.nextDividendPaymentAmount = CommonUtils::provide_f64default(sm, "nextDividendPaymentAmount", 0.0);
                cm.exDividendDate = IsoDatetime::provide(sm, "exDividendDate");
                cm.cycleAnchorDateOfFee = IsoDatetime::provide(sm, "cycleAnchorDateOfFee");
                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.boundaryValue = CommonUtils::provide_f64(sm, "priceAtPurchaseDate");
                cm.boundaryDirection = BoundaryDirection::provide(sm, "boundaryDirection");
                cm.boundaryEffect = BoundaryEffect::provide(sm, "boundaryEffect");
                cm.boundaryLegInitiallyActive = BoundaryLegInitiallyActive::provide(sm, "boundaryLegInitiallyActive");

                cm.boundaryMonitoringAnchorDate = if let Some(boundaryMonitoringAnchorDate) = IsoDatetime::provide(sm, "boundaryMonitoringAnchorDate") {
                    Some(boundaryMonitoringAnchorDate)
                } else {
                    cm.purchaseDate.clone()
                };

                cm.boundaryMonitoringEndDate = if let Some(boundaryMonitoringEndDate) = IsoDatetime::provide(sm, "boundaryMonitoringEndDate") {
                    Some(boundaryMonitoringEndDate)
                } else {
                    //cm.maturityDate.clone()
                    cm.maturityDate.clone().map(|rc| (*rc).clone())
                };

                cm.boundaryMonitoringCycle = CommonUtils::provide_string(sm, "boundaryMonitoringCycle");
                cm.boundaryCrossedFlag = CommonUtils::provide_bool(sm, "boundaryCrossedFlag");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap()))
                            .collect();
                        cm.contractStructure = Some(contract_structure);
                    }
                }

                Ok(cm)
            },
            "OPTNS" => {
                let mut cm = ContractModel::init();

                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Some(Rc::clone(calendar));
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone.unwrap());
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.creatorID = CommonUtils::provide_string(sm, "creatorID");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                cm.seniority = Seniority::provide(sm, "seniority");
                cm.nonPerformingDate = IsoDatetime::provide(sm, "nonPerformingDate");
                cm.prepaymentPeriod = IsoPeriod::provide(sm, "prepaymentPeriod");
                cm.gracePeriod = IsoPeriod::provide(sm, "gracePeriod");
                cm.delinquencyPeriod = IsoPeriod::provide(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);


                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new("DF").expect("good cetc"));
                    w
                };
                cm.creditEventTypeCovered = Some(b);


                cm.cycleAnchorDateOfDividend = IsoDatetime::provide(sm, "cycleAnchorDateOfDividend");
                cm.cycleOfDividend = CommonUtils::provide_string(sm, "cycleOfDividend");
                cm.nextDividendPaymentAmount = CommonUtils::provide_f64default(sm, "nextDividendPaymentAmount", 0.0);
                cm.exDividendDate = IsoDatetime::provide(sm, "exDividendDate");
                cm.cycleAnchorDateOfFee = IsoDatetime::provide(sm, "cycleAnchorDateOfFee");
                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);

                cm.cycleAnchorDateOfInterestPayment = if cm.cycleOfInterestPayment.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment")
                };

                cm.arrayCycleAnchorDateOfInterestPayment = IsoDatetime::provide_vec(sm, "arrayCycleAnchorDateOfInterestPayment");
                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.arrayCycleOfInterestPayment = CommonUtils::provide_string_vec(sm, "arrayCycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.settlementPeriod = IsoPeriod::provide(sm, "settlementPeriod");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.optionType = OptionType::provide(sm, "optionType");
                cm.optionStrike1 = CommonUtils::provide_f64(sm, "optionStrike1");
                cm.optionStrike2 = CommonUtils::provide_f64(sm, "optionStrike2");
                cm.currency = CommonUtils::provide_string(sm, "currency");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap()))
                            .collect();
                        cm.contractStructure = Some(contract_structure);
                    }
                }

                Ok(cm)
            },
            "CAPFL" => {
                let mut cm = ContractModel::init();

                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.as_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &cm.contractRole.clone().unwrap()))
                            .collect();
                        cm.contractStructure = Some(contract_structure);
                    }
                }

                Ok(cm)
            },
            _ => Err("test erreur".to_string()),
        }
    }
}
