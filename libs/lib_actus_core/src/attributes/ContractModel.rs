use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::attributes::ContractReference::ContractReference;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractStructure::ContractStructure;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};
use crate::util::CommonUtils::{CommonUtils, Value};

#[derive(PartialEq, Debug, Clone)]
pub enum FieldValue {
    vString(String),
    vF64(f64),
    vIsoDatetime(IsoDatetime),
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
            Self::vString(s) => Some(s.clone()),
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
pub struct ContractModel {
    pub calendar: Option<Rc<Calendar>>, // pas d'option, champs obligatoire
    pub businessDayAdjuster: Option<BusinessDayAdjuster>,
    pub endOfMonthConvention: Option<EndOfMonthConvention>,
    pub contractType: Option<String>, // obligatoire
    pub contractID: Option<String>,
    pub statusDate: Option<IsoDatetime>,
    pub contractRole: Option<ContractRole>,
    pub counterpartyID: Option<String>,
    pub creatorID: Option<String>,
    pub marketObjectCode: Option<String>,
    pub cycleAnchorDateOfFee: Option<IsoDatetime>,
    pub cycleOfFee: Option<String>,
    pub feeBasis: Option<FeeBasis>,
    pub feeRate: Option<f64>,
    pub feeAccrued: Option<f64>,
    pub cycleAnchorDateOfInterestPayment: Option<IsoDatetime>,
    pub cycleAnchorDateOfDividend: Option<IsoDatetime>,
    pub cycleOfInterestPayment: Option<String>,
    pub nominalInterestRate: Option<f64>,
    pub dayCountConvention: Option<DayCountConvention>,
    pub deliverySettlement: Option<DeliverySettlement>,
    pub accruedInterest: Option<f64>, // obligatoire
    pub capitalizationEndDate: Option<IsoDatetime>,
    pub cyclePointOfInterestPayment: Option<CyclePointOfInterestPayment>,
    pub currency: Option<String>, // obligatoire
    pub initialExchangeDate: Option<IsoDatetime>,
    pub premiumDiscountAtIED: Option<f64>,
    pub notionalPrincipal: Option<f64>,
    pub purchaseDate: Option<IsoDatetime>,
    pub priceAtPurchaseDate: Option<f64>,
    pub terminationDate: Option<IsoDatetime>,
    pub priceAtTerminationDate: Option<f64>,
    pub marketObjectCodeOfScalingIndex: Option<String>,
    pub seniority: Option<Seniority>,
    pub scalingIndexAtContractDealDate: Option<f64>,
    pub notionalScalingMultiplier: Option<f64>,
    pub interestScalingMultiplier: Option<f64>,
    pub cycleAnchorDateOfScalingIndex: Option<IsoDatetime>,
    pub cycleOfScalingIndex: Option<String>,
    pub scalingEffect: Option<ScalingEffect>,
    // TODO: review prepayment mechanism and attributes
    pub cycleAnchorDateOfOptionality: Option<IsoDatetime>,
    pub cycleOfOptionality: Option<String>,
    pub penaltyType: Option<PenaltyType>,
    pub penaltyRate: Option<f64>,
    pub objectCodeOfPrepaymentModel: Option<String>,
    pub cycleAnchorDateOfRateReset: Option<IsoDatetime>,
    pub cycleOfRateReset: Option<String>,
    pub rateSpread: Option<f64>,
    pub marketObjectCodeOfRateReset: Option<String>,
    pub lifeCap: Option<f64>,
    pub lifeFloor: Option<f64>,
    pub periodCap: Option<f64>,
    pub periodFloor: Option<f64>,
    pub cyclePointOfRateReset: Option<CyclePointOfRateReset>,
    pub fixingPeriod: Option<String>,
    pub nextResetRate: Option<f64>,
    pub rateMultiplier: Option<f64>, // obligatoire
    pub maturityDate: Option<Rc<IsoDatetime>>, // obligatoire
    pub contractPerformance: Option<ContractPerformance>,
    pub contractStructure: Option<Vec<ContractReference>>,
    pub quantity: Option<f64>,
    pub marketValueObserved: Option<f64>,
    pub cycleOfDividendPayment: Option<String>,
    pub cycleAnchorDateOfDividendPayment: Option<IsoDatetime>,
    pub marketObjectCodeOfDividends: Option<String>,
    pub nonPerformingDate: Option<IsoDatetime>,
    pub prepaymentPeriod: Option<String>,
    pub gracePeriod: Option<String>,
    pub delinquencyPeriod: Option<String>,
    pub delinquencyRate: Option<f64>,
    pub guaranteedExposure: Option<GuaranteedExposure>,
    pub coverageOfCreditEnhancement: Option<f64>,
    pub cycleOfDividend: Option<String>,
    pub nextDividendPaymentAmount: Option<f64>,
    pub exDividendDate: Option<IsoDatetime>,
    pub arrayCycleAnchorDateOfInterestPayment: Option<String>,
    pub arrayCycleOfInterestPayment: Option<String>,
    pub exerciseAmount: Option<f64>,
    pub settlementPeriod: Option<String>,
    pub exerciseDate: Option<IsoDatetime>,
    pub optionType: Option<OptionType>,
    pub optionStrike1: Option<f64>,
    pub optionStrike2: Option<f64>,
    pub xDayNotice: Option<String>,
    pub cycleAnchorDateOfInterestCalculationBase: Option<IsoDatetime>,
    pub cycleOfInterestCalculationBase: Option<String>,
    pub interestCalculationBase: Option<InterestCalculationBase>,
    pub interestCalculationBaseAmount: Option<f64>,
    pub cycleAnchorDateOfPrincipalRedemption: Option<IsoDatetime>,
    pub cycleOfPrincipalRedemption: Option<String>,
    pub nextPrincipalRedemptionPayment: Option<f64>,
    pub amortizationDate: Option<IsoDatetime>,
    pub boundaryValue: Option<f64>,
    pub boundaryDirection: Option<BoundaryDirection>,
    pub boundaryEffect: Option<BoundaryEffect>,
    pub boundaryLegInitiallyActive: Option<BoundaryLegInitiallyActive>,
    pub boundaryMonitoringAnchorDate: Option<IsoDatetime>,
    pub boundaryMonitoringEndDate: Option<IsoDatetime>,
    pub boundaryMonitoringCycle: Option<String>,
    pub boundaryCrossedFlag: Option<bool>,

    pub arrayCycleAnchorDateOfPrincipalRedemption: Option<Vec<IsoDatetime>>,
    pub arrayCycleOfPrincipalRedemption: Option<Vec<String>>,
    pub arrayNextPrincipalRedemptionPayment: Option<Vec<f64>>,
    pub arrayIncreaseDecrease: Option<Vec<ArrayIncreaseDecrease>>,

    pub arrayCycleAnchorDateOfRateReset: Option<Vec<IsoDatetime>>,
    pub arrayCycleOfRateReset: Option<Vec<String>>,
    pub arrayRate: Option<Vec<f64>>,
    pub arrayFixedVariable: Option<Vec<ArrayFixedVariable>>,

    pub accruedInterest2: Option<f64>,
    pub nominalInterestRate2: Option<f64>,
    pub currency2: Option<String>,
    pub notionalPrincipal2: Option<f64>,
    pub creditEventTypeCovered: Option<Vec<CreditEventTypeCovered>>,
    pub futuresPrice: Option<f64>,
}

impl ContractModel {
    pub fn init() -> Self {
        ContractModel {
            calendar: None,
            businessDayAdjuster: None,
            endOfMonthConvention: None,
            contractType: None,
            contractID: None,
            statusDate: None,
            contractRole: None,
            counterpartyID: None,
            creatorID: None,
            marketObjectCode: None,
            cycleAnchorDateOfFee: None,
            cycleAnchorDateOfDividend: None,
            cycleOfFee: None,
            feeBasis: None,
            feeRate: None,
            feeAccrued: None,
            cycleAnchorDateOfInterestPayment: None,
            cycleOfInterestPayment: None,
            nominalInterestRate: None,
            dayCountConvention: None,
            deliverySettlement: None,
            accruedInterest: None,
            capitalizationEndDate: None,
            cyclePointOfInterestPayment: None,
            currency: None,
            initialExchangeDate: None,
            premiumDiscountAtIED: None,
            notionalPrincipal: None,
            purchaseDate: None,
            priceAtPurchaseDate: None,
            terminationDate: None,
            priceAtTerminationDate: None,
            marketObjectCodeOfScalingIndex: None,
            seniority: None,
            scalingIndexAtContractDealDate: None,
            notionalScalingMultiplier: None,
            interestScalingMultiplier: None,
            cycleAnchorDateOfScalingIndex: None,
            cycleOfScalingIndex: None,
            scalingEffect: None,
            cycleAnchorDateOfOptionality: None,
            cycleOfOptionality: None,
            penaltyType: None,
            penaltyRate: None,
            objectCodeOfPrepaymentModel: None,
            cycleAnchorDateOfRateReset: None,
            cycleOfRateReset: None,
            rateSpread: None,
            marketObjectCodeOfRateReset: None,
            lifeCap: None,
            lifeFloor: None,
            periodCap: None,
            periodFloor: None,
            cyclePointOfRateReset: None,
            fixingPeriod: None,
            nextResetRate: None,
            rateMultiplier: None,
            maturityDate: None,
            contractPerformance: None,
            contractStructure: None,
            quantity: None,
            marketValueObserved: None,
            cycleOfDividendPayment: None,
            cycleAnchorDateOfDividendPayment: None,
            marketObjectCodeOfDividends: None,
            nonPerformingDate: None,
            prepaymentPeriod: None,
            gracePeriod: None,
            delinquencyPeriod: None,
            delinquencyRate: None,
            guaranteedExposure: None,
            coverageOfCreditEnhancement: None,
            cycleOfDividend: None,
            nextDividendPaymentAmount: None,
            exDividendDate: None,
            arrayCycleAnchorDateOfInterestPayment: None,
            arrayCycleOfInterestPayment: None,
            exerciseAmount: None,
            settlementPeriod: None,
            exerciseDate: None,
            optionType: None,
            optionStrike1: None,
            optionStrike2: None,
            xDayNotice: None,
            cycleAnchorDateOfInterestCalculationBase: None,
            cycleOfInterestCalculationBase: None,
            interestCalculationBase: None,
            interestCalculationBaseAmount: None,
            cycleAnchorDateOfPrincipalRedemption: None,
            cycleOfPrincipalRedemption: None,
            nextPrincipalRedemptionPayment: None,
            amortizationDate: None,
            boundaryValue: None,
            boundaryDirection: None,
            boundaryEffect: None,
            boundaryLegInitiallyActive: None,
            boundaryMonitoringAnchorDate: None,
            boundaryMonitoringEndDate: None,
            boundaryMonitoringCycle: None,
            boundaryCrossedFlag: None,
            
            arrayCycleAnchorDateOfPrincipalRedemption: None,
            arrayCycleOfPrincipalRedemption: None,
            arrayNextPrincipalRedemptionPayment: None,
            arrayIncreaseDecrease: None,
            arrayCycleAnchorDateOfRateReset: None,
            arrayCycleOfRateReset: None,
            arrayRate: None,
            arrayFixedVariable: None,

            accruedInterest2: None,
            nominalInterestRate2: None,
            currency2: None,
            notionalPrincipal2: None,
            creditEventTypeCovered: None,
            futuresPrice: None,
        }
    }

    pub fn get_field(&self, field_name: &str) -> Option<FieldValue> {
        match field_name {
            "calendar" => Some(FieldValue::vCalendar(self.calendar.clone().unwrap())) , // pas d'option, champs obligatoire
            "businessDayAdjuster" => Some(FieldValue::vBusinessDayAdjuster(self.businessDayAdjuster.clone().unwrap())),
            "endOfMonthConvention" => Some(FieldValue::vEndOfMonthConvention(self.endOfMonthConvention?)),
            "contractType" => Some(FieldValue::vString(self.contractType.clone().unwrap())), // obligatoire
            "contractID" => Some(FieldValue::vString(self.contractID.clone().unwrap())),
            "statusDate" => Some(FieldValue::vIsoDatetime(self.statusDate?)),
            "contractRole" => Some(FieldValue::vContractRole(self.contractRole.clone().unwrap())),
            "counterpartyID" => Some(FieldValue::vString(self.counterpartyID.clone().unwrap())),
            "creatorID" => Some(FieldValue::vString(self.creatorID.clone().unwrap())),
            "marketObjectCode" => Some(FieldValue::vString(self.marketObjectCode.clone().unwrap())),
            "cycleAnchorDateOfFee" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfFee?)),
            "cycleAnchorDateOfDividend" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfDividend?)),
            "cycleOfFee" => Some(FieldValue::vString(self.cycleOfFee.clone().unwrap())),
            "feeBasis" => Some(FieldValue::vFeeBasis(self.feeBasis.clone().unwrap())),
            "feeRate" => Some(FieldValue::vF64(self.feeRate?)),
            "feeAccrued" => Some(FieldValue::vF64(self.feeAccrued?)),
            "cycleAnchorDateOfInterestPayment" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfInterestPayment?)),
            "cycleOfInterestPayment" => Some(FieldValue::vString(self.cycleOfInterestPayment.clone().unwrap())),
            "nominalInterestRate" => Some(FieldValue::vF64(self.nominalInterestRate?)),
            "dayCountConvention" => Some(FieldValue::vDayCountConvention(self.dayCountConvention.clone().unwrap())),
            "accruedInterest" => Some(FieldValue::vF64(self.accruedInterest?)),
            "capitalizationEndDate" => Some(FieldValue::vIsoDatetime(self.capitalizationEndDate?)),
            "cyclePointOfInterestPayment" =>Some(FieldValue::vCyclePointOfInterestPayment(self.cyclePointOfInterestPayment.clone().unwrap())),
            "currency" => Some(FieldValue::vString(self.currency.clone().unwrap())), // obligatoire
            "initialExchangeDate" => Some(FieldValue::vIsoDatetime(self.initialExchangeDate?)),
            "premiumDiscountAtIED" => Some(FieldValue::vF64(self.premiumDiscountAtIED?)),
            "notionalPrincipal" => Some(FieldValue::vF64(self.notionalPrincipal?)),
            "purchaseDate" => Some(FieldValue::vIsoDatetime(self.purchaseDate?)),
            "priceAtPurchaseDate" => Some(FieldValue::vF64(self.priceAtPurchaseDate?)),
            "terminationDate" => Some(FieldValue::vIsoDatetime(self.terminationDate?)),
            "priceAtTerminationDate" => Some(FieldValue::vF64(self.priceAtTerminationDate?)),
            "marketObjectCodeOfScalingIndex" => Some(FieldValue::vString(self.marketObjectCodeOfScalingIndex.clone().unwrap())),
            "seniority" => Some(FieldValue::vSeniority(self.seniority.clone().unwrap())),
            "scalingIndexAtContractDealDate" => Some(FieldValue::vF64(self.scalingIndexAtContractDealDate?)),
            "notionalScalingMultiplier" => Some(FieldValue::vF64(self.notionalScalingMultiplier?)),
            "interestScalingMultiplier" => Some(FieldValue::vF64(self.interestScalingMultiplier?)),
            "cycleAnchorDateOfScalingIndex" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfScalingIndex?)),
            "cycleOfScalingIndex" => Some(FieldValue::vString(self.cycleOfScalingIndex.clone().unwrap())),
            "scalingEffect" => Some(FieldValue::vScalingEffect(self.scalingEffect.clone().unwrap())),
            // TODO=> review prepayment mechanism and attributes
            "cycleAnchorDateOfOptionality" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfOptionality?)),
            "cycleOfOptionality" => Some(FieldValue::vString(self.cycleOfOptionality.clone().unwrap())),
            "penaltyType" => Some(FieldValue::vPenaltyType(self.penaltyType.clone().unwrap())),
            "penaltyRate" => Some(FieldValue::vF64(self.penaltyRate?)),
            "objectCodeOfPrepaymentModel" => Some(FieldValue::vString(self.objectCodeOfPrepaymentModel.clone().unwrap())),
            "cycleAnchorDateOfRateReset" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfRateReset?)),
            "cycleOfRateReset" => Some(FieldValue::vString(self.cycleOfRateReset.clone().unwrap())),
            "rateSpread" => Some(FieldValue::vF64(self.rateSpread?)),
            "marketObjectCodeOfRateReset" => Some(FieldValue::vString(self.marketObjectCodeOfRateReset.clone().unwrap())),
            "lifeCap" => Some(FieldValue::vF64(self.lifeCap?)),
            "lifeFloor" => Some(FieldValue::vF64(self.lifeFloor?)),
            "periodCap" => Some(FieldValue::vF64(self.periodCap?)),
            "periodFloor" => Some(FieldValue::vF64(self.periodFloor?)),
            "cyclePointOfRateReset" => Some(FieldValue::vCyclePointOfRateReset(self.cyclePointOfRateReset.clone().unwrap())),
            "fixingPeriod" => Some(FieldValue::vString(self.fixingPeriod.clone().unwrap())),
            "nextResetRate" => Some(FieldValue::vF64(self.nextResetRate?)),
            "rateMultiplier" => Some(FieldValue::vF64(self.rateMultiplier?)),
            "maturityDate" =>Some(FieldValue::vMaturityDate(self.maturityDate.clone().unwrap())),
            "contractPerformance" => Some(FieldValue::vContractPerformance(self.contractPerformance?)),
            "deliverySettlement" => Some(FieldValue::vDeliverySettlement(self.deliverySettlement.clone().unwrap())),
            "quantity" => Some(FieldValue::vF64(self.quantity.clone().unwrap())),
            "marketValueObserved" => Some(FieldValue::vF64(self.marketValueObserved.clone().unwrap())),
            "cycleOfDividendPayment" => Some(FieldValue::vString(self.cycleOfDividendPayment.clone().unwrap())),
            "cycleAnchorDateOfDividendPayment" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfDividendPayment.clone().unwrap())),
            "marketObjectCodeOfDividends" => Some(FieldValue::vString(self.marketObjectCodeOfDividends.clone().unwrap())),
            "nonPerformingDate" => Some(FieldValue::vIsoDatetime(self.nonPerformingDate.clone().unwrap())),
            "prepaymentPeriod" => Some(FieldValue::vString(self.prepaymentPeriod.clone().unwrap())),
            "gracePeriod" => Some(FieldValue::vString(self.gracePeriod.clone().unwrap())),
            "delinquencyPeriod" => Some(FieldValue::vString(self.delinquencyPeriod.clone().unwrap())),
            "delinquencyRate" => Some(FieldValue::vF64(self.delinquencyRate.clone().unwrap())),
            "guaranteedExposure" => Some(FieldValue::vGuaranteedExposure(self.guaranteedExposure.clone().unwrap())),
            "coverageOfCreditEnhancement" =>Some(FieldValue::vF64(self.coverageOfCreditEnhancement.clone().unwrap())),
            "cycleOfDividend" =>Some(FieldValue::vString(self.cycleOfDividend.clone().unwrap())),
            "nextDividendPaymentAmount" =>Some(FieldValue::vF64(self.nextDividendPaymentAmount.clone().unwrap())),
            "exDividendDate" => Some(FieldValue::vIsoDatetime(self.exDividendDate.clone().unwrap())),
            "arrayCycleAnchorDateOfInterestPayment" =>Some(FieldValue::vString(self.arrayCycleAnchorDateOfInterestPayment.clone().unwrap())),
            "arrayCycleOfInterestPayment" =>Some(FieldValue::vString(self.arrayCycleOfInterestPayment.clone().unwrap())),
            "exerciseAmount" =>Some(FieldValue::vF64(self.exerciseAmount.clone().unwrap())),
            "settlementPeriod" =>Some(FieldValue::vString(self.settlementPeriod.clone().unwrap())),
            "exerciseDate" => Some(FieldValue::vIsoDatetime(self.exerciseDate.clone().unwrap())),
            "optionType" =>Some(FieldValue::vOptionType(self.optionType.clone().unwrap())),
            "optionStrike1" =>Some(FieldValue::vF64(self.optionStrike1.clone().unwrap())),
            "optionStrike2" =>Some(FieldValue::vF64(self.optionStrike2.clone().unwrap())),
            "xDayNotice"=>Some(FieldValue::vString(self.xDayNotice.clone().unwrap())),
            "cycleAnchorDateOfInterestCalculationBase"=>Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfInterestCalculationBase.clone().unwrap())),
            "cycleOfInterestCalculationBase"=>Some(FieldValue::vString(self.cycleOfInterestCalculationBase.clone().unwrap())),
            "interestCalculationBase"=>Some(FieldValue::vInterestCalculationBase(self.interestCalculationBase.clone().unwrap())),
            "interestCalculationBaseAmount" =>Some(FieldValue::vF64(self.interestCalculationBaseAmount.clone().unwrap())),
            "cycleAnchorDateOfPrincipalRedemption"=>Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfPrincipalRedemption.clone().unwrap())),
            "cycleOfPrincipalRedemption"=>Some(FieldValue::vString(self.cycleOfPrincipalRedemption.clone().unwrap())),
            "nextPrincipalRedemptionPayment" =>Some(FieldValue::vF64(self.nextPrincipalRedemptionPayment.clone().unwrap())),
            "amortizationDate"=>Some(FieldValue::vIsoDatetime(self.amortizationDate.clone().unwrap())),
            "boundaryValue" => Some(FieldValue::vF64(self.boundaryValue.clone().unwrap())),
            "boundaryDirection"=>Some(FieldValue::vBoundaryDirection(self.boundaryDirection.clone().unwrap())),
            "boundaryEffect"=>Some(FieldValue::vBoundaryEffect(self.boundaryEffect.clone().unwrap())),
            "boundaryLegInitiallyActive"=>Some(FieldValue::vBoundaryLegInitiallyActive(self.boundaryLegInitiallyActive.clone().unwrap())),
            "boundaryMonitoringAnchorDate" => Some(FieldValue::vIsoDatetime(self.boundaryMonitoringAnchorDate.clone().unwrap())),
            "boundaryMonitoringEndDate" => Some(FieldValue::vIsoDatetime(self.boundaryMonitoringEndDate.clone().unwrap())),
            "boundaryMonitoringCycle"=>Some(FieldValue::vString(self.boundaryMonitoringCycle.clone().unwrap())),
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
            "currency2" =>Some(FieldValue::vString(self.currency2.clone().unwrap())),
            "notionalPrincipal2" =>Some(FieldValue::vF64(self.notionalPrincipal2.clone().unwrap())),
            "creditEventTypeCovered"=>Some(FieldValue::vVecCreditEventTypeCovered(self.creditEventTypeCovered.clone().unwrap())),
            "futuresPrice" =>Some(FieldValue::vF64(self.futuresPrice.clone().unwrap())),
            _ => None,
        }
    }

    pub fn new(sm: &HashMap<String, Value>) -> Result<ContractModel, String> {
        let ct = sm.get("contractType").unwrap();
        match ct.extract_string().unwrap().as_str() {
            "PAM" => {
                let mut cm = ContractModel::init();

                //let mut cm = PAM::default();
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    // Clone seulement l'Rc, pas le calendrier lui-même
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone
                    );
                }

                // Clonez simplement les Rc existantes
                if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    cm.dayCountConvention = DayCountConvention::provide(
                        sm,
                        "dayCountConvention",
                        Rc::clone(maturity_date),
                        Rc::clone(calendar)
                    );
                }

                cm.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
                cm.contractType = CommonUtils::provide_string(sm, "contractType");
                cm.contractID = CommonUtils::provide_string(sm, "contractID");
                cm.statusDate = IsoDatetime::provide(sm, "statusDate");
                cm.contractRole = ContractRole::provide(sm, "contractRole");
                cm.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
                cm.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
                cm.cycleAnchorDateOfFee = IsoDatetime::provide(sm, "cycleAnchorDateOfFee");
                cm.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
                cm.feeBasis = FeeBasis::provide(sm, "feeBasis");
                cm.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
                cm.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);
                cm.cycleAnchorDateOfInterestPayment = IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment");
                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64(sm, "nominalInterestRate");
                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);// obligatoire
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");
                cm.cyclePointOfInterestPayment = CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment");
                cm.currency = CommonUtils::provide_string(sm, "currency"); // obligatoire
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.premiumDiscountAtIED = CommonUtils::provide_f64(sm, "premiumDiscountAtIED");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64(sm, "priceAtPurchaseDate");
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64(sm, "priceAtTerminationDate");
                cm.marketObjectCodeOfScalingIndex = CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex");
                cm.scalingIndexAtContractDealDate = CommonUtils::provide_f64(sm, "scalingIndexAtContractDealDate");
                cm.notionalScalingMultiplier = CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0);
                cm.interestScalingMultiplier = CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0);
                cm.cycleAnchorDateOfScalingIndex = IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex");
                cm.cycleOfScalingIndex = CommonUtils::provide_string(sm, "cycleOfScalingIndex");
                cm.scalingEffect = ScalingEffect::provide(sm, "scalingEffect");
                // TODO: review prepayment mechanism and attributes
                cm.cycleAnchorDateOfOptionality = IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality");
                cm.cycleOfOptionality = CommonUtils::provide_string(sm, "cycleOfOptionality");
                cm.penaltyType = PenaltyType::provide(sm, "penaltyType");
                cm.penaltyRate = CommonUtils::provide_f64default(sm, "penaltyRate", 0.0);
                cm.objectCodeOfPrepaymentModel = CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel");
                cm.cycleAnchorDateOfRateReset = IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset");
                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
                cm.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
                cm.periodCap = CommonUtils::provide_f64(sm, "periodCap");
                cm.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
                cm.cyclePointOfRateReset = CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset");
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0); // obligatoire
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                
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
                
                let v = &sm.get("contractStructure").unwrap().extract_vec().unwrap() ;
                //let d1 = v.get(0).unwrap();
                //let r = ContractReference::new(&d1, &cm.contractRole.clone().unwrap());

                let a: Vec<ContractReference> = v.iter().map(|d| {
                    ContractReference::new(&d, &cm.contractRole.clone().unwrap())
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(
                        sm,
                        "BusinessDayAdjuster",
                        calendar_clone
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new_B()) {
                    Some(CyclePointOfRateReset::new_E())
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
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");

                Ok(cm)
            },
            "ANN" => {
                let mut cm = ContractModel::init();

                cm.calendar = Calendar::provide_rc(sm, "calendar");

                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new_B()) {
                    Some(CyclePointOfRateReset::new_E())
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
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.arrayCycleAnchorDateOfInterestPayment = CommonUtils::provide_string(sm, "arrayCycleAnchorDateOfInterestPayment");
                cm.arrayCycleOfInterestPayment = CommonUtils::provide_string(sm, "arrayCycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);

                cm.dayCountConvention = if let (Some(maturity_date), Some(calendar)) = (&cm.maturityDate, &cm.calendar) {
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
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

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new_B()) {
                    Some(CyclePointOfRateReset::new_E())
                } else {
                    CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
                };
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0);
                cm.deliverySettlement = DeliverySettlement::provide(sm, "deliverySettlement");

                Ok(cm)
            },
            "FXOUT" => {
                let mut cm = ContractModel::init();
                cm.calendar = Calendar::provide_rc(sm, "calendar");
                if let Some(calendar) = &cm.calendar {
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");

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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.terminationDate = IsoDatetime::provide(sm, "terminationDate");
                cm.priceAtTerminationDate = CommonUtils::provide_f64default(sm, "priceAtTerminationDate", 0.0);
                cm.xDayNotice = CommonUtils::provide_string(sm, "xDayNotice");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
                cm.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
                cm.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
                cm.xDayNotice = CommonUtils::provide_string(sm, "xDayNotice");

                cm.cycleAnchorDateOfRateReset = if cm.cycleOfRateReset.is_none() {
                    IsoDatetime::provide(sm, "initialExchangeDate")
                } else {
                    IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset")
                };

                cm.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
                cm.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
                cm.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
                } else {
                    None
                };

                cm.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);
                cm.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");

                cm.cyclePointOfRateReset = if cm.cyclePointOfInterestPayment == Some(CyclePointOfInterestPayment::new_B()) {
                    Some(CyclePointOfRateReset::new_E())
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
                cm.fixingPeriod = CommonUtils::provide_string(sm, "fixingPeriod");
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                };
                cm.creditEventTypeCovered = Some(b);

                cm.currency = CommonUtils::provide_string(sm, "currency");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.extract_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d, &cm.contractRole.clone().unwrap()))
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.gracePeriod = CommonUtils::provide_string(sm, "gracePeriod");
                cm.delinquencyPeriod = CommonUtils::provide_string(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
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
                    DayCountConvention::provide(sm, "dayCountConvention", Rc::clone(maturity_date), Rc::clone(calendar))
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
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.extract_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d, &cm.contractRole.clone().unwrap()))
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.prepaymentPeriod = CommonUtils::provide_string(sm, "prepaymentPeriod");
                cm.gracePeriod = CommonUtils::provide_string(sm, "gracePeriod");
                cm.delinquencyPeriod = CommonUtils::provide_string(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
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
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.prepaymentPeriod = CommonUtils::provide_string(sm, "prepaymentPeriod");
                cm.gracePeriod = CommonUtils::provide_string(sm, "gracePeriod");
                cm.delinquencyPeriod = CommonUtils::provide_string(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);

                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
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
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");
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
                    if let Some(structure_vec) = contractStructure.extract_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d, &cm.contractRole.clone().unwrap()))
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
                    let calendar_clone = Rc::clone(calendar);
                    cm.businessDayAdjuster = BusinessDayAdjuster::provide(sm, "businessDayAdjuster", calendar_clone);
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
                cm.prepaymentPeriod = CommonUtils::provide_string(sm, "prepaymentPeriod");
                cm.gracePeriod = CommonUtils::provide_string(sm, "gracePeriod");
                cm.delinquencyPeriod = CommonUtils::provide_string(sm, "delinquencyPeriod");
                cm.delinquencyRate = CommonUtils::provide_f64default(sm, "delinquencyRate", 0.0);

                cm.guaranteedExposure = GuaranteedExposure::provide(sm, "guaranteedExposure");
                cm.coverageOfCreditEnhancement = CommonUtils::provide_f64default(sm, "coverageOfCreditEnhancement", 1.0);


                let a = CreditEventTypeCovered::provide_vec(sm,"creditEventTypeCovered");
                let b: Vec<CreditEventTypeCovered> = if a.is_none() {
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
                    w
                } else {
                    // A REFAIRE PAS LE BON CODE
                    let mut w: Vec<CreditEventTypeCovered> = vec![];
                    w.push(CreditEventTypeCovered::new_DF());
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

                cm.arrayCycleAnchorDateOfInterestPayment = CommonUtils::provide_string(sm, "arrayCycleAnchorDateOfInterestPayment");
                cm.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
                cm.arrayCycleOfInterestPayment = CommonUtils::provide_string(sm, "arrayCycleOfInterestPayment");
                cm.nominalInterestRate = CommonUtils::provide_f64default(sm, "nominalInterestRate", 0.0);
                cm.exerciseAmount = CommonUtils::provide_f64default(sm, "exerciseAmount", 0.0);
                cm.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
                cm.settlementPeriod = CommonUtils::provide_string(sm, "settlementPeriod");
                cm.exerciseDate = IsoDatetime::provide(sm, "exerciseDate");
                cm.priceAtPurchaseDate = CommonUtils::provide_f64default(sm, "priceAtPurchaseDate", 0.0);
                cm.optionType = OptionType::provide(sm, "optionType");
                cm.optionStrike1 = CommonUtils::provide_f64(sm, "optionStrike1");
                cm.optionStrike2 = CommonUtils::provide_f64(sm, "optionStrike2");
                cm.currency = CommonUtils::provide_string(sm, "currency");

                if let Some(contractStructure) = sm.get("contractStructure") {
                    if let Some(structure_vec) = contractStructure.extract_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d, &cm.contractRole.clone().unwrap()))
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
                    if let Some(structure_vec) = contractStructure.extract_vec() {
                        let contract_structure: Vec<ContractReference> = structure_vec.iter()
                            .map(|d| ContractReference::new(d, &cm.contractRole.clone().unwrap()))
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
