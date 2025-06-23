use std::rc::Rc;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::contract_types::Pam::PAM;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::types::isoDatetime::IsoDatetime;


pub struct Champs {
    pub calendar: Option<Rc<Calendar>>, // pas d'option, champs obligatoire
    pub businessDayAdjuster: Option<BusinessDayAdjuster>,
    pub endOfMonthConvention: Option<EndOfMonthConvention>,
    pub contractType: Option<String>, // obligatoire
    pub contractID: Option<String>,
    pub statusDate: Option<IsoDatetime>,
    pub contractRole: Option<ContractRole>,
    pub counterpartyID: Option<String>,
    pub marketObjectCode: Option<String>,
    pub cycleAnchorDateOfFee: Option<IsoDatetime>,
    pub cycleOfFee: Option<String>,
    pub feeBasis: Option<FeeBasis>,
    pub feeRate: Option<f64>,
    pub feeAccrued: Option<f64>,
    pub cycleAnchorDateOfInterestPayment: Option<IsoDatetime>,
    pub cycleOfInterestPayment: Option<String>,
    pub nominalInterestRate: Option<f64>,
    pub dayCountConvention: Option<DayCountConvention>,
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
    pub fixingPeriod: Option<IsoDatetime>,
    pub nextResetRate: Option<f64>,
    pub rateMultiplier: Option<f64>, // obligatoire
    pub maturityDate: Option<Rc<IsoDatetime>>, // obligatoire
    pub contractPerformance: Option<ContractPerformance>,
}

impl PAM {
    pub fn init() -> Self {
        PAM {
            calendar: None,
            businessDayAdjuster: None,
            endOfMonthConvention: None,
            contractType: None,
            contractID: None,
            statusDate: None,
            contractRole: None,
            counterpartyID: None,
            marketObjectCode: None,
            cycleAnchorDateOfFee: None,
            cycleOfFee: None,
            feeBasis: None,
            feeRate: None,
            feeAccrued: None,
            cycleAnchorDateOfInterestPayment: None,
            cycleOfInterestPayment: None,
            nominalInterestRate: None,
            dayCountConvention: None,
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
            contractPerformance: None
        }
    }


}