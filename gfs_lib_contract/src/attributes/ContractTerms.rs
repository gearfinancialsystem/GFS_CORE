use std::rc::Rc;
use gfs_lib_terms::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use gfs_lib_terms::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use gfs_lib_terms::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use gfs_lib_terms::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use gfs_lib_terms::terms::grp_boundary::BoundaryMonitoringAnchorDate::BoundaryMonitoringAnchorDate;
use gfs_lib_terms::terms::grp_boundary::BoundaryMonitoringCycle::BoundaryMonitoringCycle;
use gfs_lib_terms::terms::grp_boundary::BoundaryMonitoringEndDate::BoundaryMonitoringEndDate;
use gfs_lib_terms::terms::grp_boundary::BoundaryValue::BoundaryValue;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_calendar::Calendar::Calendar;
use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::CreatorID::CreatorID;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use gfs_lib_terms::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use gfs_lib_terms::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use gfs_lib_terms::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use gfs_lib_terms::terms::grp_counterparty::GracePeriod::GracePeriod;
use gfs_lib_terms::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use gfs_lib_terms::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use gfs_lib_terms::terms::grp_counterparty::PrepaymentPeriod::PrepaymentPeriod;
use gfs_lib_terms::terms::grp_counterparty::Seniority::Seniority;
use gfs_lib_terms::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use gfs_lib_terms::terms::grp_dividend::CycleAnchorDateOfDividendPayment::CycleAnchorDateOfDividendPayment;
use gfs_lib_terms::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use gfs_lib_terms::terms::grp_dividend::CycleOfDividendPayment::CycleOfDividendPayment;
use gfs_lib_terms::terms::grp_dividend::ExDividendDate::ExDividendDate;
use gfs_lib_terms::terms::grp_dividend::MarketObjectCodeOfDividends::MarketObjectCodeOfDividends;
use gfs_lib_terms::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use gfs_lib_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use gfs_lib_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use gfs_lib_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use gfs_lib_terms::terms::grp_fees::FeeBasis::FeeBasis;
use gfs_lib_terms::terms::grp_fees::FeeRate::FeeRate;
use gfs_lib_terms::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::ArrayCycleAnchorDateOfInterestPayment::ArrayCycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::ArrayCycleOfInterestPayment::ArrayCycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use gfs_lib_terms::terms::grp_notional_principal::ArrayCycleAnchorDateOfPrincipalRedemption::ArrayCycleAnchorDateOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::ArrayCycleOfPrincipalRedemption::ArrayCycleOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use gfs_lib_terms::terms::grp_notional_principal::ArrayNextPrincipalRedemptionPayment::ArrayNextPrincipalRedemptionPayment;
use gfs_lib_terms::terms::grp_notional_principal::Currency2::Currency2;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use gfs_lib_terms::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use gfs_lib_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::MarketValueObserved::MarketValueObserved;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::Quantity::Quantity;
use gfs_lib_terms::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use gfs_lib_terms::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::XDayNotice::XDayNotice;
use gfs_lib_terms::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use gfs_lib_terms::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use gfs_lib_terms::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use gfs_lib_terms::terms::grp_optionality::OptionStrike1::OptionStrike1;
use gfs_lib_terms::terms::grp_optionality::OptionStrike2::OptionStrike2;
use gfs_lib_terms::terms::grp_optionality::OptionType::OptionType;
use gfs_lib_terms::terms::grp_optionality::PenaltyRate::PenaltyRate;
use gfs_lib_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use gfs_lib_terms::terms::grp_reset_rate::ArrayCycleAnchorDateOfRateReset::ArrayCycleAnchorDateOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::ArrayCycleOfRateReset::ArrayCycleOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use gfs_lib_terms::terms::grp_reset_rate::ArrayRate::ArrayRate;
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
use gfs_lib_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use gfs_lib_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use gfs_lib_terms::terms::grp_settlement::ExerciseDate::ExerciseDate;
use gfs_lib_terms::terms::grp_settlement::FuturesPrice::FuturesPrice;
use gfs_lib_terms::terms::grp_settlement::SettlementCurrency::SettlementCurrency;
use gfs_lib_terms::terms::grp_settlement::SettlementPeriod::SettlementPeriod;


#[derive(PartialEq, Debug, Clone, Default)]  // Toutes les options sont None
pub struct ContractTerms {
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
    pub array_increase_decrease: Option<ArrayIncreaseDecrease>,
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
    pub calendar: Rc<Calendar>,
    pub capitalization_end_date: Option<CapitalizationEndDate>,
    pub contract_id: Option<ContractID>,
    pub contract_performance: Option<ContractPerformance>,
    pub contract_role: Option<ContractRole>,
    // pub contract_structure: Option<ContractStructure>,
    pub contract_type: Option<ContractType>, // le seul term different de la specification
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
    pub cycle_of_dividend_payment: Option<CycleOfDividendPayment>,
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
    pub end_of_month_convention: EndOfMonthConvention,
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
    pub market_object_code_of_dividends: Option<MarketObjectCodeOfDividends>, // nest pas dans la liste des champs sur le site, mais dans le code
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


// impl ContractModel {
//
//     pub fn new(sm: &HashMap<String, Value>) -> Result<ContractModel, String> {
//         let ct = sm.get("contractType").unwrap();
//         let ct_str = ct.as_string().unwrap().as_str();
//         match ct_str {
//             "PAM" => {
//                 //let mut cm = ContractModel::init();
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let business_day_adjuster: Option<BusinessDayAdjuster> =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "BusinessDayAdjuster",
//                         calendar_clone.expect("te")
//                     )
//                 };
//
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(
//                         sm,
//                         "dayCountConvention",
//                         Some(Rc::clone(maturity_date)),
//                         Some(Rc::clone(&calendar))
//                     )
//                 } else {
//                     None
//                 };
//
//                 //map.put("cycleAnchorDateOfRateReset", (CommonUtils.isNull(attributes.get("cycleAnchorDateOfRateReset"))) ?
//                 //  ((CommonUtils.isNull(attributes.get("cycleOfRateReset"))) ? null : LocalDateTime.parse(attributes.get("initialExchangeDate"))) : LocalDateTime.parse(attributes.get("cycleAnchorDateOfRateReset")));
//
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_resetxx = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_resetxx.is_none() {
//                     if cycle_of_rate_reset.is_none() {
//                         None
//                     }
//                     else {
//                         let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                         CycleAnchorDateOfRateReset::from_str(&a).ok()
//                     }
//                 } else {
//                     cycle_anchor_date_of_rate_resetxx
//                 };
//                 // une valeur par default non specifier dans la norme mais dans la base de code
//                 let mut accrued_interest = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
//                 if accrued_interest.is_none() {
//                     accrued_interest = AccruedInterest::new(0.0).ok();
//                 }
//
//                 let mut fee_rate = FeeRate::provide_from_input_dict(sm, "feeRate");
//                 if fee_rate.is_none() {
//                     fee_rate = FeeRate::new(0.0).ok();
//                 }
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let mut rate_multiplier = RateMultiplier::provide_from_input_dict(sm, "rateMultiplier");
//                 if rate_multiplier.is_none() {
//                     rate_multiplier = RateMultiplier::new(1.0).ok();
//                 }
//
//                 let mut notional_scaling_multiplier = NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier");
//                 if notional_scaling_multiplier.is_none() {
//                     notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
//                 }
//
//                 let mut interest_scaling_multiplier = InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier");
//                 if interest_scaling_multiplier.is_none() {
//                     interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
//                 }
//
//                 let cm = ContractModel {
//                     accrued_interest:                       accrued_interest,
//                     business_day_adjuster:                  business_day_adjuster,
//                     calendar:                               calendar,
//                     capitalization_end_date:                CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
//                     contract_id:                            ContractID::provide_from_input_dict(sm, "contractID"),
//                     contract_performance:                   ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     contract_role:                          ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     contract_type:                          ct_str.to_string(),
//                     counterparty_id:                        CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     currency:                               Currency::provide_from_input_dict(sm, "currency"),
//                     cycle_anchor_date_of_fee:               CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     cycle_anchor_date_of_interest_payment:  CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
//                     cycle_anchor_date_of_optionality:       CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality"),
//                     cycle_anchor_date_of_rate_reset:        cycle_anchor_date_of_rate_reset,
//                     cycle_anchor_date_of_scaling_index:     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex"),
//                     cycle_of_fee:                           CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     cycle_of_interest_payment:              CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     cycle_of_optionality:                   CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
//                     cycle_of_rate_reset:                    cycle_of_rate_reset,
//                     cycle_of_scaling_index:                 CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
//                     cycle_point_of_interest_payment:        CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
//                     cycle_point_of_rate_reset:              CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset"),
//                     day_count_convention:                   day_count_convention,
//                     end_of_month_convention:                end_of_month_convention,
//                     fee_accrued:                            FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     fee_basis:                              FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate:                               fee_rate,
//                     fixing_period:                          FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     initial_exchange_date:                  InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     interest_scaling_multiplier:            interest_scaling_multiplier,
//                     life_cap:                               LifeCap::provide_from_input_dict(sm, "lifeCap"),
//                     life_floor:                             LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
//                     market_object_code:                     MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     market_object_code_of_rate_reset:       MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     market_object_code_of_scaling_index:    MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
//                     maturity_date:                          maturity_date,
//                     next_reset_rate:                        NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     nominal_interest_rate:                  NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     notional_principal:                     NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     notional_scaling_multiplier:            notional_scaling_multiplier,
//                     object_code_of_prepayment_model:        ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
//                     penalty_rate:                           PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
//                     penalty_type:                           PenaltyType::provide_from_input_dict(sm, "penaltyType"),
//                     period_cap:                             PeriodCap::provide_from_input_dict(sm, "periodCap"),
//                     period_floor:                           PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
//                     premium_discount_at_ied:                PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
//                     price_at_purchase_date:                 PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     price_at_termination_date:              PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     purchase_date:                          PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     rate_multiplier:                        rate_multiplier,
//                     rate_spread:                            RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     scaling_effect:                         ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
//                     scaling_index_at_contract_deal_date:    ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
//                     status_date:                            StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     termination_date:                       TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "SWAPS" => {
//
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//
//                 let cm = ContractModel {
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_structure: contract_structure,
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "STK" => {
//
//                 //let purchase_date = IsoDatetime::provide(sm, "purchaseDate");
//                 let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");
//
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 //VERIFIER PAS PRESENT DANS LA LISTE DES TERMES
//                 let cycle_of_dividend_payment = CycleOfDividendPayment::provide_from_input_dict(sm, "cycleOfDividendPayment");
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "BusinessDayAdjuster",
//                         calendar_clone.expect("df")
//                     )
//                 };
//
//                 let cycle_anchor_date_of_dividend_payment = {
//                     let a = if cycle_of_dividend_payment.is_none() {
//                         None
//                     } else {
//                         let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
//                         CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).ok()
//                     };
//                     let b = CycleAnchorDateOfDividendPayment::provide_from_input_dict(sm, "CycleAnchorDateOfDividendPayment");
//                     if b.is_none() { a } else { b }
//                 };
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     quantity: Quantity::provide_from_input_dict(sm, "quantity"),
//                     purchase_date: purchase_date,
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     cycle_of_dividend_payment: cycle_of_dividend_payment,
//                     cycle_anchor_date_of_dividend_payment: cycle_anchor_date_of_dividend_payment,
//                     market_object_code_of_dividends: MarketObjectCodeOfDividends::provide_from_input_dict(sm, "marketObjectCodeOfDividends"),
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "LAM" => {
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
//                 let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
//                     None
//                 } else {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfFee::from_str(&a).ok()
//                 };
//
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     //IsoDatetime::provide(sm, "initialExchangeDate")
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention =
//                     if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//                 let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment");
//                 let cycle_point_of_rate_reset =
//                     if let Some(point) = &cycle_point_of_interest_payment {
//                     if point.to_string() == "B" {
//                         CyclePointOfRateReset::from_str("E").ok()
//                     } else {
//                         CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset")
//                     }
//                 } else {
//                     None
//                 };
//
//                 let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex");
//                 let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfScalingIndex::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex")
//                 };
//
//                 let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality");
//                 let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfOptionality::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
//                 };
//
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfRateReset::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset")
//                 };
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.expect("ere")
//                     )
//                 };
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
//                     fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
//                     capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
//                     cycle_point_of_rate_reset: cycle_point_of_rate_reset,
//                     cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
//                     scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
//                     notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
//                     interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
//                     cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
//                     cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
//                     scaling_effect: ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
//                     cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
//                     cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
//                     penalty_type: PenaltyType::provide_from_input_dict(sm, "penaltyType"),
//                     penalty_rate: PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
//                     object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
//                     life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
//                     period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
//                     period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     maturity_date: maturity_date,
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "ANN" => {
//                 // Déclarations simples sans dépendances
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 // Champs qui dépendent d'autres champs
//                 let cycle_of_fee = CycleOfFee::provide_from_input_dict (sm, "cycleOfFee");
//                 let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfFee::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
//                 };
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict (sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//                 let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment");
//                 let cycle_point_of_rate_reset =
//                     if let Some(point) = &cycle_point_of_interest_payment {
//                         if point.to_string() == "B" {
//                             CyclePointOfRateReset::from_str("E").ok()
//                         } else {
//                             CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset")
//                         }
//                     } else {
//                         None
//                     };
//
//
//                 let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex");
//                 let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfScalingIndex::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex")
//                 };
//
//
//                 let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict (sm, "cycleOfOptionality");
//                 let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfOptionality::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
//                 };
//
//
//
//                 let cycle_anchor_date_of_rate_reset = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_reset.is_some() {
//                     cycle_anchor_date_of_rate_reset
//                 } else {
//                     if cycle_of_rate_reset.is_none() {
//                         None
//                     }
//                     else {
//                         let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
//                         CycleAnchorDateOfRateReset::new(a).ok()
//                     }
//
//                 };
//
//                 let cycle_of_interest_calculation_base = CycleOfInterestCalculationBase::provide_from_input_dict (sm, "cycleOfInterestCalculationBase");
//                 let cycle_anchor_date_of_interest_calculation_base = if cycle_of_interest_calculation_base.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict (sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestCalculationBase::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestCalculationBase::provide_from_input_dict(sm, "cycleAnchorDateOfInterestCalculationBase3")
//                 };
//
//
//                 let interest_calculation_base_tmp = InterestCalculationBase::provide_from_input_dict(sm, "interestCalculationBase");
//                 let interest_calculation_base = if interest_calculation_base_tmp.is_none() {
//                     InterestCalculationBase::new("NT").ok()
//                 } else {
//                     interest_calculation_base_tmp
//                 };
//
//                 let cycle_of_principal_redemption = CycleOfPrincipalRedemption::provide_from_input_dict (sm, "cycleOfPrincipalRedemption");
//
//                 let b = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
//                 let z = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption");
//                 let cycle_anchor_date_of_principal_redemption = if z.is_some() {
//                     z
//                 }
//                 else { CycleAnchorDateOfPrincipalRedemption::new(b.unwrap().value()).ok() };
//
//                 // let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
//                 //     let a = initial_exchange_date.value().to_string();
//                 //     CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
//                 // } else {
//                 //     CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
//                 // };
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let w = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
//                 let accrued_interest = if w.is_some() {
//                     w
//                 }
//                 else {
//                     AccruedInterest::new(0.0).ok()
//                 };
//
//
//                 let w = FeeRate::provide_from_input_dict(sm, "feeRate");
//                 let fee_rate = if w.is_some() {
//                     w
//                 }
//                 else {
//                     FeeRate::new(0.0).ok()
//                 };
//
//
//                 let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
//                 let period_cap = if w.is_some() {
//                     w
//                 }
//                 else {
//                     PeriodCap::new(f64::INFINITY).ok()
//                 };
//
//                 let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
//                 let period_floor = if w.is_some() {
//                     w
//                 }
//                 else {
//                     PeriodFloor::new(f64::NEG_INFINITY).ok()
//                 };
//
//
//                 let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
//                 let life_cap = if w.is_some() {
//                     w
//                 }
//                 else {
//                     LifeCap::new(f64::INFINITY).ok()
//                 };
//
//                 let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
//                 let life_floor = if w.is_some() {
//                     w
//                 }
//                 else {
//                     LifeFloor::new(f64::NEG_INFINITY).ok()
//                 };
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: fee_rate,
//                     fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     accrued_interest: accrued_interest,
//                     capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
//                     cycle_point_of_rate_reset: cycle_point_of_rate_reset,
//                     cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
//                     scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
//                     notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
//                     interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
//                     cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
//                     cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
//                     scaling_effect: ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
//                     cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
//                     cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
//                     penalty_type: PenaltyType::provide_from_input_dict(sm, "penaltyType"),
//                     penalty_rate: PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
//                     object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     life_cap: life_cap,
//                     life_floor: life_floor,
//                     period_cap: period_cap,
//                     period_floor: period_floor,
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     maturity_date: maturity_date,
//                     cycle_anchor_date_of_interest_calculation_base: cycle_anchor_date_of_interest_calculation_base,
//                     cycle_of_interest_calculation_base: CycleOfInterestCalculationBase::provide_from_input_dict(sm, "cycleOfInterestCalculationBase"),
//                     interest_calculation_base: interest_calculation_base,
//                     interest_calculation_base_amount: InterestCalculationBaseAmount::provide_from_input_dict(sm, "interestCalculationBaseAmount"),
//                     cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
//                     cycle_of_principal_redemption: cycle_of_principal_redemption,
//                     next_principal_redemption_payment: NextPrincipalRedemptionPayment::provide_from_input_dict(sm, "nextPrincipalRedemptionPayment"),
//                     amortization_date: AmortizationDate::provide_from_input_dict(sm, "amortizationDate"),
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "LAX" => {
//                 // Déclarations simples sans dépendances
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 // Champs qui dépendent d'autres champs
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//                 let cycle_of_interest_calculation_base = CycleOfInterestCalculationBase::provide_from_input_dict (sm, "cycleOfInterestCalculationBase");
//                 let cycle_anchor_date_of_interest_calculation_base = if cycle_of_interest_calculation_base.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict (sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestCalculationBase::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestCalculationBase::provide_from_input_dict(sm, "cycleAnchorDateOfInterestCalculationBase3")
//                 };
//
//                 let interest_calculation_base_tmp = InterestCalculationBase::provide_from_input_dict(sm, "interestCalculationBase");
//                 let interest_calculation_base = if interest_calculation_base_tmp.is_none() {
//                     InterestCalculationBase::new("NT").ok()
//                 } else {
//                     interest_calculation_base_tmp
//                 };
//
//                 let mut fee_rate = FeeRate::provide_from_input_dict(sm, "feeRate");
//                 if fee_rate.is_none() {
//                     fee_rate = FeeRate::new(0.0).ok();
//                 }
//
//                 let b = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
//                 let cycle_anchor_date_of_principal_redemption = if let Some(initial_exchange_date) = b {
//                     let a = initial_exchange_date.value().to_string();
//                     CycleAnchorDateOfPrincipalRedemption::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption")
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let b = eomc.unwrap();
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {
//                     eomc.unwrap()
//                 };
//                 let z = ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment");
//
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
//                     maturity_date: maturity_date,
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     array_cycle_anchor_date_of_principal_redemption: ArrayCycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "arrayCycleAnchorDateOfPrincipalRedemption"),
//                     array_cycle_of_principal_redemption: ArrayCycleOfPrincipalRedemption::provide_from_input_dict(sm, "arrayCycleOfPrincipalRedemption"),
//                     array_next_principal_redemption_payment: ArrayNextPrincipalRedemptionPayment::provide_from_input_dict(sm, "arrayNextPrincipalRedemptionPayment"),
//                     array_increase_decrease: ArrayIncreaseDecrease::provide_from_input_dict(sm, "arrayIncreaseDecrease"),
//                     array_cycle_anchor_date_of_interest_payment: ArrayCycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "arrayCycleAnchorDateOfInterestPayment"),
//                     array_cycle_of_interest_payment: ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     array_cycle_anchor_date_of_rate_reset: ArrayCycleAnchorDateOfRateReset::provide_from_input_dict(sm, "arrayCycleAnchorDateOfRateReset"),
//                     array_cycle_of_rate_reset: ArrayCycleOfRateReset::provide_from_input_dict(sm, "arrayCycleOfRateReset"),
//                     array_rate: ArrayRate::provide_from_input_dict(sm, "arrayRate"),
//                     array_fixed_variable: ArrayFixedVariable::provide_from_input_dict(sm, "arrayFixedVariable"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     fee_rate: fee_rate,
//                     end_of_month_convention: end_of_month_convention,
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
//                     period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
//                     life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
//                     life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
//                     cycle_anchor_date_of_interest_calculation_base: cycle_anchor_date_of_interest_calculation_base,
//                     cycle_of_interest_calculation_base: CycleOfInterestCalculationBase::provide_from_input_dict(sm, "cycleOfInterestCalculationBase"),
//                     interest_calculation_base: interest_calculation_base,
//                     interest_calculation_base_amount: InterestCalculationBaseAmount::provide_from_input_dict(sm, "interestCalculationBaseAmount"),
//                     cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "SWPPV" => {
//
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//
//                 // Champs qui dépendent d'autres champs
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfRateReset::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
//                 };
//
//                 let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment");
//                 let cycle_point_of_rate_reset = if let Some(point) = &cycle_point_of_interest_payment {
//                     if point.to_string() == "B" {
//                         Some(CyclePointOfRateReset::new("E").expect("d"))
//                     } else {
//                         CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
//                     }
//                 } else {
//                     CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
//                     accrued_interest2: AccruedInterest2::provide_from_input_dict(sm, "accruedInterest2"),
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     nominal_interest_rate2: NominalInterestRate2::provide_from_input_dict(sm, "nominalInterestRate2"),
//                     day_count_convention: day_count_convention,
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     maturity_date: maturity_date,
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     cycle_point_of_rate_reset: cycle_point_of_rate_reset,
//                     cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "FXOUT" => {
//                 // Déclarations simples sans dépendances
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 // Gestion des dépendances
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     currency2: Currency2::provide_from_input_dict(sm, "currency2"),
//                     maturity_date: maturity_date,
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     notional_principal2: NotionalPrincipal2::provide_from_input_dict(sm, "notionalPrincipal2"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
//                     settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "COM" => {
//
//                 let cm = ContractModel {
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     quantity: Quantity::provide_from_input_dict(sm, "quantity"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "CSH" => {
//
//                 let cm = ContractModel {
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             "UMP" => {
//                 // Déclarations simples sans dépendances
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 // doit etre None a priori
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
//                 let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfFee::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
//                 };
//
//                 let mut cycle_anchor_date_of_interest_payment = CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment");
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict (sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_anchor_date_of_interest_payment.is_some() {
//                     cycle_anchor_date_of_interest_payment
//                 } else {
//                     if cycle_of_interest_payment.is_some() {
//                         let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
//                         CycleAnchorDateOfInterestPayment::new(a).ok()
//                     }
//                     else {
//                         None
//                     }
//                 };
//
//                 let day_count_convention =
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", maturity_date.clone(), Some(Rc::clone(&calendar)));
//                 // let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                 //
//                 // } else {
//                 //     None
//                 // };
//
//                 let mut cycle_anchor_date_of_rate_reset = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict (sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_reset.is_some() {
//                     cycle_anchor_date_of_rate_reset
//                 } else {
//                     if cycle_of_rate_reset.is_some() {
//                         let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
//                         CycleAnchorDateOfRateReset::new(a).ok()
//                     }
//                     else {
//                         None
//                     }
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//                 let w = NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate");
//                 let nominal_interest_rate = if w.is_some() {
//                     w
//                 }
//                 else {
//                     NominalInterestRate::new(0.0).ok()
//                 };
//
//                 let w = FeeRate::provide_from_input_dict(sm, "feeRate");
//                 let fee_rate = if w.is_some() {
//                     w
//                 }
//                 else {
//                     FeeRate::new(0.0).ok()
//                 };
//
//                 let w = FeeAccrued::provide_from_input_dict(sm, "feeAccrued");
//                 let fee_accrued = if w.is_some() { w } else { FeeAccrued::new(0.0).ok() };
//
//
//                 let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
//                 let period_cap = if w.is_some() { w } else { PeriodCap::new(f64::INFINITY).ok() };
//
//                 let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
//                 let period_floor = if w.is_some() { w } else { PeriodFloor::new(f64::NEG_INFINITY).ok() };
//
//                 let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
//                 let life_cap = if w.is_some() { w } else { LifeCap::new(f64::INFINITY).ok() };
//
//                 let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
//                 let life_floor = if w.is_some() { w } else { LifeFloor::new(f64::NEG_INFINITY).ok() };
//
//
//
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: fee_rate,
//                     fee_accrued: fee_accrued,
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment:cycle_of_interest_payment,
//                     nominal_interest_rate: nominal_interest_rate,
//                     day_count_convention: day_count_convention,
//                     accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     life_cap: life_cap,
//                     life_floor: life_floor,
//                     period_cap: period_cap,
//                     period_floor: period_floor,
//                     maturity_date: maturity_date,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "CLM" => {
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//
//                 let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
//                 let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfFee::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
//                 };
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfRateReset::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
//                 };
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
//                     fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     maturity_date: maturity_date,
//                     x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
//                     life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
//                     period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
//                     period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "NAM" => {
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//
//                 let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
//                 let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfFee::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
//                 };
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//                 let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment");
//                 let cycle_point_of_rate_reset =
//                     if let Some(point) = &cycle_point_of_interest_payment {
//                         if point.to_string() == "B" {
//                             CyclePointOfRateReset::from_str("E").ok()
//                         } else {
//                             CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset")
//                         }
//                     } else {
//                         None
//                     };
//
//
//                 let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex");
//                 let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfScalingIndex::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex")
//                 };
//
//
//                 let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict (sm, "cycleOfOptionality");
//                 let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfOptionality::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
//                 };
//
//                 let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
//                 let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfRateReset::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//
//                 let w = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
//                 let accrued_interest = if w.is_some() {
//                     w
//                 }
//                 else {
//                     AccruedInterest::new(0.0).ok()
//                 };
//
//
//                 let w = FeeRate::provide_from_input_dict(sm, "feeRate");
//                 let fee_rate = if w.is_some() {
//                     w
//                 }
//                 else {
//                     FeeRate::new(0.0).ok()
//                 };
//
//
//                 let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
//                 let period_cap = if w.is_some() {
//                     w
//                 }
//                 else {
//                     PeriodCap::new(f64::INFINITY).ok()
//                 };
//
//                 let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
//                 let period_floor = if w.is_some() {
//                     w
//                 }
//                 else {
//                     PeriodFloor::new(f64::NEG_INFINITY).ok()
//                 };
//
//
//                 let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
//                 let life_cap = if w.is_some() {
//                     w
//                 }
//                 else {
//                     LifeCap::new(f64::INFINITY).ok()
//                 };
//
//                 let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
//                 let life_floor = if w.is_some() {
//                     w
//                 }
//                 else {
//                     LifeFloor::new(f64::NEG_INFINITY).ok()
//                 };
//
//                 let mut cycle_anchor_date_of_principal_redemption = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption");
//                 cycle_anchor_date_of_principal_redemption = if cycle_anchor_date_of_principal_redemption.is_some() {
//                     cycle_anchor_date_of_principal_redemption
//                 }
//                 else {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
//                     CycleAnchorDateOfPrincipalRedemption::new(a.unwrap().value()).ok()
//                 };
//
//
//                 let mut scaling_effect = ScalingEffect::provide_from_input_dict(sm, "scalingEffect");
//                 scaling_effect = if scaling_effect.is_some() {
//                     scaling_effect
//                 }
//                 else {
//                     Some(ScalingEffect::new("OOO").unwrap())
//                 };
//
//                 let mut premium_discount_at_ied= PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED");
//                 premium_discount_at_ied = if premium_discount_at_ied.is_some() {
//                     premium_discount_at_ied
//                 }
//                 else {
//                     PremiumDiscountAtIED::new(0.0).ok()
//                 };
//
//
//                 let mut next_principal_redemption_payment= NextPrincipalRedemptionPayment::provide_from_input_dict(sm, "nextPrincipalRedemptionPayment");
//                 next_principal_redemption_payment = if next_principal_redemption_payment.is_some() {
//                     next_principal_redemption_payment
//                 }
//                 else {
//                     None
//                 };
//
//                 let cm = ContractModel {
//                     next_principal_redemption_payment: next_principal_redemption_payment,
//                     premium_discount_at_ied: premium_discount_at_ied,
//                     cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
//                     cycle_of_principal_redemption: CycleOfPrincipalRedemption::provide_from_input_dict(sm, "cycleOfPrincipalRedemption"),
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: fee_rate,
//                     fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     accrued_interest: accrued_interest,
//                     capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
//                     cycle_point_of_rate_reset: cycle_point_of_rate_reset,
//                     cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
//                     notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
//                     scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
//                     notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
//                     interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
//                     cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
//                     cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
//                     scaling_effect: scaling_effect,
//                     cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
//                     cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
//                     penalty_type: PenaltyType::provide_from_input_dict(sm, "penaltyType"),
//                     penalty_rate: PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
//                     object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
//                     cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
//                     cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
//                     rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
//                     market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
//                     life_cap: life_cap,
//                     life_floor: life_floor,
//                     period_cap: period_cap,
//                     period_floor: period_floor,
//                     fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
//                     next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
//                     rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
//                     maturity_date: maturity_date,
//                     credit_event_type_covered: credit_event_type_covered,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "CEC" => {
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//
//                 // let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 // let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                 //     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                 //     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 // } else {
//                 //     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 // };
//
//                 // let day_count_convention = if let Some(maturity_date) = &maturity_date {
//                 //     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 // } else {
//                 //     None
//                 // };
//
//                 let calendar_clone = Some(Rc::clone(&calendar));
//                 let a = BusinessDayAdjuster::provide(
//                     sm,
//                     "businessDayAdjuster",
//                     calendar_clone.unwrap()
//                 );
//                 let mut business_day_adjuster = if a.is_some() {
//                     a
//                 } else {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::new("NOS", calendar_clone.unwrap()).ok()
//                 };
//
//
//                 //this.bdConvention = new Same();
//                 //this.scConvention = new ShiftCalc();
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let mut guaranteed_exposure = GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure");
//                 guaranteed_exposure = if guaranteed_exposure.is_none() {
//                     GuaranteedExposure::new(Some("NO")).ok()
//                 } else {
//                     guaranteed_exposure
//                 };
//
//                 let mut coverage_of_credit_enhancement = CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement");
//                 coverage_of_credit_enhancement = if coverage_of_credit_enhancement.is_none() {
//                     CoverageOfCreditEnhancement::new(1.0).ok()
//                 } else {
//                     coverage_of_credit_enhancement
//                 };
//
//                 let mut settlement_period =  SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod");
//                 settlement_period = if settlement_period.is_none() {
//                     let a = SettlementPeriod::parse_from_string("P0D").unwrap();
//                     Some(SettlementPeriod::new(a.years, a.months, a.days))
//                 } else {
//                     settlement_period
//                 };
//
//                 let mut exercise_amount = ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount");
//                 exercise_amount = if exercise_amount.is_none() {
//                     ExerciseAmount::new(0.0).ok()
//                 } else {
//                     exercise_amount
//                 };
//
//
//
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: contract_role,
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//
//                     //contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     //non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
//                     //grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
//                     //delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
//                     //delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
//                     guaranteed_exposure: guaranteed_exposure,
//                     coverage_of_credit_enhancement: coverage_of_credit_enhancement,
//                     credit_event_type_covered: credit_event_type_covered,
//                     //cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     //cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     //fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     //fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
//                     //fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     //cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     //cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     //nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     //day_count_convention: day_count_convention,
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     //maturity_date: maturity_date,
//                     //notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
//                     //purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     //price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     //termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     //price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
//                     exercise_amount: exercise_amount,
//                     settlement_period: settlement_period,
//                     //cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
//                     //cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
//                     //next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
//                     //ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
//                     contract_structure: contract_structure,
//                     //market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "CEG" => {
//                 // Déclarations simples sans dépendances
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let day_count_convention = if let (Some(maturity_date)) = (&maturity_date) {
//                     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
//                 } else {
//                     None
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//                 let cm = ContractModel {
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: contract_role,
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
//                     grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
//                     delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
//                     delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
//                     guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
//                     coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
//                     credit_event_type_covered: credit_event_type_covered,
//                     cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
//                     cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
//                     next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
//                     ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
//                     cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
//                     fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
//                     fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     day_count_convention: day_count_convention,
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
//                     exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
//                     settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     contract_structure: contract_structure,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "FUTUR" => {
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//
//
//                 let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
//                 let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
//                     //IsoDatetime::provide(sm, "initialExchangeDate")
//                     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
//                     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
//
//                 } else {
//                     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
//                 };
//
//                 let business_day_adjuster =  {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     maturity_date: maturity_date,
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     seniority: Seniority::provide_from_input_dict(sm, "seniority"),
//                     non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
//                     prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
//                     grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
//                     delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
//                     delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
//                     guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
//                     coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
//                     credit_event_type_covered: credit_event_type_covered,
//                     cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
//                     cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
//                     next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
//                     ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
//                     cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
//                     exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
//                     exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     futures_price: FuturesPrice::provide_from_input_dict(sm, "futuresPrice"),
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "BCS" => {
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp.clone() {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//                 let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");
//
//
//                 let w = BoundaryMonitoringAnchorDate::provide_from_input_dict(sm, "boundaryMonitoringAnchorDate");
//                 let boundary_monitoring_anchor_date = if let Some(boundary_monitoring_anchor_date) = w {
//                     Some(boundary_monitoring_anchor_date)
//                 } else {
//                     let aa = purchase_date.clone().unwrap().value().to_string();
//                     BoundaryMonitoringAnchorDate::from_str(&aa).ok()
//                 };
//
//                 let a = BoundaryMonitoringEndDate::provide_from_input_dict(sm, "BoundaryMonitoringEndDate");
//                 let boundary_monitoring_end_date = if let Some(boundary_monitoring_end_date) = a {
//                     Some(boundary_monitoring_end_date)
//                 } else {
//
//                     Some(BoundaryMonitoringEndDate::from_str(maturity_date_tmp.unwrap().value().to_string().as_str()).unwrap())
//                 };
//
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                     BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//                 let cm = ContractModel {
//                     maturity_date: maturity_date,
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: contract_role,
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     seniority: Seniority::provide_from_input_dict(sm, "seniority"),
//                     non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
//                     prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
//                     grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
//                     delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
//                     delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
//                     guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
//                     coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
//                     credit_event_type_covered: credit_event_type_covered,
//                     cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
//                     cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
//                     next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
//                     ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
//                     cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
//                     exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     boundary_value: BoundaryValue::provide_from_input_dict(sm, "boundaryValue"),
//                     boundary_direction: BoundaryDirection::provide_from_input_dict(sm, "boundaryDirection"),
//                     boundary_effect: BoundaryEffect::provide_from_input_dict(sm, "boundaryEffect"),
//                     boundary_leg_initially_active: BoundaryLegInitiallyActive::provide_from_input_dict(sm, "boundaryLegInitiallyActive"),
//                     boundary_monitoring_anchor_date: boundary_monitoring_anchor_date,
//                     boundary_monitoring_end_date: boundary_monitoring_end_date,
//                     boundary_monitoring_cycle: BoundaryMonitoringCycle::provide_from_input_dict(sm, "boundaryMonitoringCycle"),
//                     boundary_crossed_flag: BoundaryCrossedFlag::provide_from_input_dict(sm, "boundaryCrossedFlag"),
//                     contract_structure: contract_structure,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "OPTNS" => {
//                 let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
//                 let maturity_date = if let Some(a) = maturity_date_tmp {
//                     Some(Rc::new(a))
//                 } else {
//                     None
//                 };
//                 let calendar = Calendar::provide_rc(sm, "calendar");
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//
//
//                 let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
//                 let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
//                     Some(CreditEventTypeCovered::default())
//                 } else {
//                     credit_event_type_covered_tmp
//                 };
//
//                 let business_day_adjuster = {
//                     let calendar_clone = Some(Rc::clone(&calendar));
//                         BusinessDayAdjuster::provide(
//                         sm,
//                         "businessDayAdjuster",
//                         calendar_clone.unwrap()
//                     )
//                 };
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//                 let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
//                 let end_of_month_convention = if eomc.is_none() {
//                     EndOfMonthConvention::default()
//                 } else {eomc.unwrap()};
//
//                 let cm = ContractModel {
//                     maturity_date: maturity_date,
//                     calendar: calendar,
//                     business_day_adjuster: business_day_adjuster,
//                     end_of_month_convention: end_of_month_convention,
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
//                     creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
//                     seniority: Seniority::provide_from_input_dict(sm, "seniority"),
//                     non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
//                     prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
//                     grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
//                     delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
//                     delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
//                     guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
//                     coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
//                     credit_event_type_covered: credit_event_type_covered,
//                     cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
//                     cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
//                     next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
//                     ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
//                     cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
//                     cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
//                     cycle_anchor_date_of_interest_payment: CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
//                     array_cycle_anchor_date_of_interest_payment: ArrayCycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "arrayCycleAnchorDateOfInterestPayment"),
//                     cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
//                     array_cycle_of_interest_payment: ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment"),
//                     nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
//                     exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
//                     exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     option_type: OptionType::provide_from_input_dict(sm, "optionType"),
//                     option_strike1: OptionStrike1::provide_from_input_dict(sm, "optionStrike1"),
//                     option_strike2: OptionStrike2::provide_from_input_dict(sm, "optionStrike2"),
//                     contract_structure: contract_structure,
//                     ..Default::default()
//                 };
//
//
//                 Ok(cm)
//             },
//             "CAPFL" => {
//
//                 let contract_role = ContractRole::provide(sm, "contractRole");
//
//                 let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
//                     if let Some(structure_vec) = contract_structure.as_vec() {
//                         let contract_structure: Vec<ContractReference> = structure_vec.iter()
//                             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
//                             .collect();
//                         Some(ContractStructure::new(contract_structure))
//                     } else {
//                         None
//                     }
//
//                 } else {None};
//
//                 let cm = ContractModel {
//                     contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
//                     status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
//                     contract_role: contract_role,
//                     contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
//                     counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
//                     market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
//                     currency: Currency::provide_from_input_dict(sm, "currency"),
//                     purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
//                     price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
//                     termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
//                     price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
//                     life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
//                     life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
//                     contract_structure: contract_structure,
//                     ..Default::default()
//                 };
//
//                 Ok(cm)
//             },
//             _ => Err("test error".to_string()),
//         }
//     }
// }
