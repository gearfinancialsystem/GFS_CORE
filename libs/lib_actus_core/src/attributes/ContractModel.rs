use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::attributes::ContractReference::ContractReference;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractStructure::ContractStructure;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
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
    pub contractStructure: Option<Vec<ContractReference>>,
    pub quantity: Option<f64>,
    pub marketValueObserved: Option<f64>,
    pub cycleOfDividendPayment: Option<String>,
    pub cycleAnchorDateOfDividendPayment: Option<IsoDatetime>,
    pub marketObjectCodeOfDividends: Option<String>
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
            marketObjectCodeOfDividends: None
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
            "marketObjectCode" => Some(FieldValue::vString(self.marketObjectCode.clone().unwrap())),
            "cycleAnchorDateOfFee" => Some(FieldValue::vIsoDatetime(self.cycleAnchorDateOfFee?)),
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
            "fixingPeriod" => Some(FieldValue::vIsoDatetime(self.fixingPeriod?)),
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
                cm.fixingPeriod = IsoDatetime::provide(sm, "fixingPeriod");
                cm.nextResetRate = CommonUtils::provide_f64(sm, "nextResetRate");
                cm.rateMultiplier = CommonUtils::provide_f64default(sm, "rateMultiplier", 1.0); // obligatoire
                cm.contractPerformance = ContractPerformance::provide(sm, "contractPerformance");
                
                Ok(cm)
            }
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
            }
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
            }
           
            _ => Err("test erreur".to_string()),

        }
    }
    
    
}
// //Implémentation de Deref pour ContractModel
// impl Deref for ContractModel {
//     type Target = ContractModel;
//
//     fn deref(&self) -> &Self::Target {
//         match self {
//             ContractModel::PAM(pam) => pam,
//             ContractModel::SWAPS(swaps) => swaps,
//         }
//     }
// }
//
// // Implémentation de DerefMut pour ContractModel
// impl DerefMut for ContractModel {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self {
//             ContractModel::PAM(pam) => pam,
//             ContractModel::SWAPS(swaps) => swaps,
//         }
//     }
// }