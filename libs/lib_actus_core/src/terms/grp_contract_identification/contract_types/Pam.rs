use std::collections::HashMap;
use crate::util::CommonUtils::CommonUtils;
use std::rc::Rc;
use chrono::NaiveDateTime;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};

// use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;


#[derive(Clone, Debug, PartialEq)]
pub struct PAM {
    pub calendar: Option<Rc<Calendar>>,
    pub businessDayConvention: Option<Box<BusinessDayConvention>>,
    pub endOfMonthConvention: Option<Box<EndOfMonthConvention>>,
    pub contractType: Option<Box<String>>, // obligatoire
    pub contractID: Option<Box<String>>,
    pub statusDate: Option<Box<IsoDatetime>>,
    pub contractRole: Option<Box<ContractRole>>,
    pub counterpartyID: Option<Box<String>>,
    pub marketObjectCode: Option<Box<String>>,
    pub cycleAnchorDateOfFee: Option<Box<IsoDatetime>>,
    pub cycleOfFee: Option<Box<String>>,
    pub feeBasis: Option<Box<FeeBasis>>,
    pub feeRate: Option<Box<f64>>,
    pub feeAccrued: Option<Box<f64>>,
    pub cycleAnchorDateOfInterestPayment: Option<Box<Vec<IsoDatetime>>>,
    pub cycleOfInterestPayment: Option<Box<String>>,
    pub nominalInterestRate: Option<Box<f64>>,
    pub dayCountConvention: Option<Box<DayCountConvention>>,
    pub accruedInterest: Option<Box<f64>>, // obligatoire
    pub capitalizationEndDate: Option<Box<IsoDatetime>>,
    pub cyclePointOfInterestPayment: Option<Box<CyclePointOfInterestPayment>>,
    pub currency: Option<Box<String>>, // obligatoire
    pub initialExchangeDate: Option<Box<IsoDatetime>>,
    pub premiumDiscountAtIED: Option<Box<f64>>,
    pub notionalPrincipal: Option<Box<f64>>,
    pub purchaseDate: Option<Box<IsoDatetime>>,
    pub priceAtPurchaseDate: Option<Box<f64>>,
    pub terminationDate: Option<Box<IsoDatetime>>,
    pub priceAtTerminationDate: Option<Box<f64>>,
    pub marketObjectCodeOfScalingIndex: Option<Box<String>>,
    pub scalingIndexAtContractDealDate: Option<Box<f64>>,
    pub notionalScalingMultiplier: Option<Box<f64>>,
    pub interestScalingMultiplier: Option<Box<f64>>,
    pub cycleAnchorDateOfScalingIndex: Option<Box<IsoDatetime>>,
    pub cycleOfScalingIndex: Option<Box<String>>,
    pub scalingEffect: Option<Box<ScalingEffect>>,
    // TODO: review prepayment mechanism and attributes
    pub cycleAnchorDateOfOptionality: Option<Box<IsoDatetime>>,
    pub cycleOfOptionality: Option<Box<String>>,
    pub penaltyType: Option<Box<PenaltyType>>,
    pub penaltyRate: Option<Box<f64>>,
    pub objectCodeOfPrepaymentModel: Option<Box<String>>,
    pub cycleAnchorDateOfRateReset: Option<Box<Vec<IsoDatetime>>>,
    pub cycleOfRateReset: Option<Box<String>>,
    pub rateSpread: Option<Box<f64>>,
    pub marketObjectCodeOfRateReset: Option<Box<String>>,
    pub lifeCap: Option<Box<f64>>,
    pub lifeFloor: Option<Box<f64>>,
    pub periodCap: Option<Box<f64>>,
    pub periodFloor: Option<Box<f64>>,
    pub cyclePointOfRateReset: Option<Box<CyclePointOfRateReset>>,
    pub fixingPeriod: Option<Box<IsoDatetime>>,
    pub nextResetRate: Option<Box<f64>>,
    pub rateMultiplier: Option<Box<f64>>, // obligatoire
    pub maturityDate: Option<Box<IsoDatetime>>, // obligatoire
}

impl Default for PAM {
    fn default() -> Self {
        PAM {
            calendar: None,
            businessDayConvention: None,
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
        }
    }


}

impl TraitContractModel for PAM {
    fn cm(&self) -> String {
        "PAM".to_string()
    }
}

impl PAM {
    pub fn parse_from_dict(&mut self, sm: &HashMap<String, String>) {

        //let mut cm = PAM::default();

        self.calendar = Calendar::provide_rc(sm, "calendar");
        self.businessDayConvention = BusinessDayConvention::provide_box(sm, "businessDayConvention", self.calendar.clone().unwrap());
        self.endOfMonthConvention = EndOfMonthConvention::provide_box(sm, "endOfMonthConvention");
        self.contractType = CommonUtils::provide_box_string(sm, "contractType");
        self.contractID = CommonUtils::provide_box_string(sm, "contractID");
        self.statusDate = IsoDatetime::provide_box(sm, "statusDate");
        self.contractRole = ContractRole::provide_box(sm, "contractRole");
        self.counterpartyID = CommonUtils::provide_box_string(sm, "counterpartyID");
        self.marketObjectCode = CommonUtils::provide_box_string(sm, "marketObjectCode");
        self.cycleAnchorDateOfFee = IsoDatetime::provide_box(sm, "cycleAnchorDateOfFee");
        self.cycleOfFee = CommonUtils::provide_box_string(sm, "cycleOfFee");
        self.feeBasis = FeeBasis::provide_box(sm, "feeBasis");
        self.feeRate = CommonUtils::provide_box_f64(sm, "feeRate");
        self.feeAccrued = CommonUtils::provide_box_f64(sm, "feeAccrued");
        self.cycleAnchorDateOfInterestPayment = IsoDatetime::provide_box_vec(sm, "cycleAnchorDateOfInterestPayment");
        self.cycleOfInterestPayment = CommonUtils::provide_box_string(sm, "cycleOfInterestPayment");
        self.nominalInterestRate = CommonUtils::provide_box_f64(sm, "nominalInterestRate");
        self.maturityDate = IsoDatetime::provide_box(sm, "maturityDate");
        self.dayCountConvention = DayCountConvention::provide_box(&sm, "dayCountConvention",
                *self.maturityDate.clone().unwrap(),
                Calendar::provide_rc(&sm, "calendar").unwrap());
        self.accruedInterest = CommonUtils::provide_box_f64(sm, "accruedInterest");// obligatoire
        self.capitalizationEndDate = IsoDatetime::provide_box(sm, "capitalizationEndDate");
        self.cyclePointOfInterestPayment = CyclePointOfInterestPayment::provide_box(sm, "cyclePointOfInterestPayment");
        self.currency = CommonUtils::provide_box_string(sm, "currency"); // obligatoire
        self.initialExchangeDate = IsoDatetime::provide_box(sm, "initialExchangeDate");
        self.premiumDiscountAtIED = CommonUtils::provide_box_f64(sm, "premiumDiscountAtIED");
        self.notionalPrincipal = CommonUtils::provide_box_f64(sm, "notionalPrincipal");
        self.purchaseDate = IsoDatetime::provide_box(sm, "purchaseDate");
        self.priceAtPurchaseDate = CommonUtils::provide_box_f64(sm, "priceAtPurchaseDate");
        self.terminationDate = IsoDatetime::provide_box(sm, "terminationDate");
        self.priceAtTerminationDate = CommonUtils::provide_box_f64(sm, "priceAtTerminationDate");
        self.marketObjectCodeOfScalingIndex = CommonUtils::provide_box_string(sm, "marketObjectCodeOfScalingIndex");
        self.scalingIndexAtContractDealDate = CommonUtils::provide_box_f64(sm, "scalingIndexAtContractDealDate");
        self.notionalScalingMultiplier = CommonUtils::provide_box_f64(sm, "notionalScalingMultiplier");
        self.interestScalingMultiplier = CommonUtils::provide_box_f64(sm, "interestScalingMultiplier");
        self.cycleAnchorDateOfScalingIndex = IsoDatetime::provide_box(sm, "cycleAnchorDateOfScalingIndex");
        self.cycleOfScalingIndex = CommonUtils::provide_box_string(sm, "cycleOfScalingIndex");
        self.scalingEffect = ScalingEffect::provide_box(sm, "scalingEffect");
        // TODO: review prepayment mechanism and attributes
        self.cycleAnchorDateOfOptionality = IsoDatetime::provide_box(sm, "cycleAnchorDateOfOptionality");
        self.cycleOfOptionality = CommonUtils::provide_box_string(sm, "cycleOfOptionality");
        self.penaltyType = PenaltyType::provide_box(sm, "penaltyType");
        self.penaltyRate = CommonUtils::provide_box_f64(sm, "penaltyRate");
        self.objectCodeOfPrepaymentModel = CommonUtils::provide_box_string(sm, "objectCodeOfPrepaymentModel");
        self.cycleAnchorDateOfRateReset = IsoDatetime::provide_box_vec(sm, "cycleAnchorDateOfRateReset");
        self.cycleOfRateReset = CommonUtils::provide_box_string(sm, "cycleOfRateReset");
        self.rateSpread = CommonUtils::provide_box_f64(sm, "rateSpread");
        self.marketObjectCodeOfRateReset = CommonUtils::provide_box_string(sm, "marketObjectCodeOfRateReset");
        self.lifeCap = CommonUtils::provide_box_f64(sm, "lifeCap");
        self.lifeFloor = CommonUtils::provide_box_f64(sm, "lifeFloor");
        self.periodCap = CommonUtils::provide_box_f64(sm, "periodCap");
        self.periodFloor = CommonUtils::provide_box_f64(sm, "periodFloor");
        self.cyclePointOfRateReset = CyclePointOfRateReset::provide_box(sm, "cyclePointOfRateReset");
        self.fixingPeriod = IsoDatetime::provide_box(sm, "fixingPeriod");
        self.nextResetRate = CommonUtils::provide_box_f64(sm, "nextResetRate");
        self.rateMultiplier = CommonUtils::provide_box_f64(sm, "rateMultiplier"); // obligatoire
        
    }

}
