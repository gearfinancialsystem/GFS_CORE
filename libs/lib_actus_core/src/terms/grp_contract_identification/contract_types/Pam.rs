use std::collections::HashMap;
use std::rc::Rc;
use crate::util::CommonUtils::CommonUtils;

use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};

// use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;


#[derive(Clone, Debug, PartialEq)]
pub struct PAM {
    pub calendar: Option<Rc<Calendar>>, // pas d'option, champs obligatoire
    pub BusinessDayAdjuster: Option<BusinessDayAdjuster>,
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
            BusinessDayAdjuster: None,
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

impl TraitContractModel for PAM {
    fn cm(&self) -> String {
        "PAM".to_string()
    }

}

impl PAM {
    pub fn parse_from_dict(&mut self, sm: &HashMap<String, String>) {

        //let mut cm = PAM::default();
        self.maturityDate = IsoDatetime::provide_rc(sm, "maturityDate");
        self.calendar = Calendar::provide_rc(sm, "calendar");

        if let Some(calendar) = &self.calendar {
            // Clone seulement l'Rc, pas le calendrier lui-même
            let calendar_clone = Rc::clone(calendar);
            self.BusinessDayAdjuster = BusinessDayAdjuster::provide(
                sm,
                "BusinessDayAdjuster",
                calendar_clone
            );
        }

        // Clonez simplement les Rc existantes
        if let (Some(maturity_date), Some(calendar)) = (&self.maturityDate, &self.calendar) {
            self.dayCountConvention = DayCountConvention::provide(
                sm,
                "dayCountConvention",
                Rc::clone(maturity_date),
                Rc::clone(calendar)
            );
        }
        // self.dayCountConvention = DayCountConvention::provide_box(&sm, "dayCountConvention",
        //         *self.maturityDate.clone().unwrap(),
        //         Calendar::provide_rc(&sm, "calendar").unwrap());

        


        self.endOfMonthConvention = EndOfMonthConvention::provide(sm, "endOfMonthConvention");
        self.contractType = CommonUtils::provide_string(sm, "contractType");
        self.contractID = CommonUtils::provide_string(sm, "contractID");
        self.statusDate = IsoDatetime::provide(sm, "statusDate");
        self.contractRole = ContractRole::provide(sm, "contractRole");
        self.counterpartyID = CommonUtils::provide_string(sm, "counterpartyID");
        self.marketObjectCode = CommonUtils::provide_string(sm, "marketObjectCode");
        self.cycleAnchorDateOfFee = IsoDatetime::provide(sm, "cycleAnchorDateOfFee");
        self.cycleOfFee = CommonUtils::provide_string(sm, "cycleOfFee");
        self.feeBasis = FeeBasis::provide(sm, "feeBasis");
        self.feeRate = CommonUtils::provide_f64default(sm, "feeRate", 0.0);
        self.feeAccrued = CommonUtils::provide_f64default(sm, "feeAccrued", 0.0);
        self.cycleAnchorDateOfInterestPayment = IsoDatetime::provide(sm, "cycleAnchorDateOfInterestPayment");
        self.cycleOfInterestPayment = CommonUtils::provide_string(sm, "cycleOfInterestPayment");
        self.nominalInterestRate = CommonUtils::provide_f64(sm, "nominalInterestRate");
        self.accruedInterest = CommonUtils::provide_f64default(sm, "accruedInterest", 0.0);// obligatoire
        self.capitalizationEndDate = IsoDatetime::provide(sm, "capitalizationEndDate");
        self.cyclePointOfInterestPayment = CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment");
        self.currency = CommonUtils::provide_string(sm, "currency"); // obligatoire
        self.initialExchangeDate = IsoDatetime::provide(sm, "initialExchangeDate");
        self.premiumDiscountAtIED = CommonUtils::provide_f64(sm, "premiumDiscountAtIED");
        self.notionalPrincipal = CommonUtils::provide_f64(sm, "notionalPrincipal");
        self.purchaseDate = IsoDatetime::provide(sm, "purchaseDate");
        self.priceAtPurchaseDate = CommonUtils::provide_f64(sm, "priceAtPurchaseDate");
        self.terminationDate = IsoDatetime::provide(sm, "terminationDate");
        self.priceAtTerminationDate = CommonUtils::provide_f64(sm, "priceAtTerminationDate");
        self.marketObjectCodeOfScalingIndex = CommonUtils::provide_string(sm, "marketObjectCodeOfScalingIndex");
        self.scalingIndexAtContractDealDate = CommonUtils::provide_f64(sm, "scalingIndexAtContractDealDate");
        self.notionalScalingMultiplier = CommonUtils::provide_f64default(sm, "notionalScalingMultiplier", 1.0);
        self.interestScalingMultiplier = CommonUtils::provide_f64default(sm, "interestScalingMultiplier", 1.0);
        self.cycleAnchorDateOfScalingIndex = IsoDatetime::provide(sm, "cycleAnchorDateOfScalingIndex");
        self.cycleOfScalingIndex = CommonUtils::provide_string(sm, "cycleOfScalingIndex");
        self.scalingEffect = ScalingEffect::provide(sm, "scalingEffect");
        // TODO: review prepayment mechanism and attributes
        self.cycleAnchorDateOfOptionality = IsoDatetime::provide(sm, "cycleAnchorDateOfOptionality");
        self.cycleOfOptionality = CommonUtils::provide_string(sm, "cycleOfOptionality");
        self.penaltyType = PenaltyType::provide(sm, "penaltyType");
        self.penaltyRate = CommonUtils::provide_f64default(sm, "penaltyRate", 0.0);
        self.objectCodeOfPrepaymentModel = CommonUtils::provide_string(sm, "objectCodeOfPrepaymentModel");
        self.cycleAnchorDateOfRateReset = IsoDatetime::provide(sm, "cycleAnchorDateOfRateReset");
        self.cycleOfRateReset = CommonUtils::provide_string(sm, "cycleOfRateReset");
        self.rateSpread = CommonUtils::provide_f64default(sm, "rateSpread", 0.0);
        self.marketObjectCodeOfRateReset = CommonUtils::provide_string(sm, "marketObjectCodeOfRateReset");
        self.lifeCap = CommonUtils::provide_f64(sm, "lifeCap");
        self.lifeFloor = CommonUtils::provide_f64(sm, "lifeFloor");
        self.periodCap = CommonUtils::provide_f64(sm, "periodCap");
        self.periodFloor = CommonUtils::provide_f64(sm, "periodFloor");
        self.cyclePointOfRateReset = CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset");
        self.fixingPeriod = IsoDatetime::provide(sm, "fixingPeriod");
        self.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
        self.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0); // obligatoire
        self.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
    }

}
