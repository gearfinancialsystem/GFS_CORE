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
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};

// use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;


#[derive(Clone, Debug, PartialEq)]
pub struct SWAPS {

    pub contractID: Option<String>,
    pub statusDate: Option<IsoDatetime>,
    pub contractRole: Option<ContractRole>,
    pub counterpartyID: Option<String>,
    pub marketObjectCode: Option<String>,
    pub purchaseDate: Option<IsoDatetime>,
    pub priceAtPurchaseDate: Option<f64>,
    pub terminationDate: Option<IsoDatetime>,
    pub priceAtTerminationDate: Option<f64>,
    pub deliverySettlement: Option<DeliverySettlement>,
    pub contractType: Option<String>, // obligatoire
    pub contractStructure: Option<ContractStructure> 
    
}