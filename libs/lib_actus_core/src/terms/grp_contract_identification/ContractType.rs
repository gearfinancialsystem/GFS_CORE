use crate::attributes::ContractModel::ContractModel;
use crate::contracts::Annuity::Annuity;
use crate::contracts::BoundaryControlledSwitch::BoundaryControlledSwitch;
use crate::contracts::{CapFloor::CapFloor, CreditEnhancementGuarantee::CreditEnhancementGuarantee};
use crate::contracts::CallMoney::CallMoney;
use crate::contracts::Cash::Cash;
use crate::contracts::CreditEnhancementCollateral::CreditEnhancementCollateral;
use crate::contracts::ExoticLinearAmortizer::ExoticLinearAmortizer;
use crate::contracts::ForeignExchangeOutright::ForeignExchangeOutright;
use crate::contracts::Future::Future;
use crate::contracts::LinearAmortizer::LinearAmortizer;
use crate::contracts::NegativeAmortizer::NegativeAmortizer;
use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;
use crate::contracts::Stock::Stock;
use crate::contracts::Swap::Swap;
use crate::contracts::Option::Option as Optionx;
use crate::contracts::PlainVanillaInterestRateSwap::PlainVanillaInterestRateSwap;
use crate::types::isoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;

#[derive(Debug, PartialEq)]
pub struct ContractType;

impl ContractType {


    pub fn schedule(to: Option<IsoDatetime>, cm: &ContractModel) -> Option<Vec<ContractEvent>> {

        match cm.clone().contractType.unwrap().as_str() {
            "ANN" => Some(Annuity::schedule(&to.unwrap(),cm).unwrap()),
            "BCS" => Some(BoundaryControlledSwitch::schedule(&to.unwrap(),cm).unwrap()),
            "CPFL" => Some(CapFloor::schedule(&to.unwrap(),cm).unwrap()),
            "CEC" => Some(CreditEnhancementCollateral::schedule(&to.unwrap(),cm).unwrap()),
            "CEG" => Some(CreditEnhancementGuarantee::schedule(&to.unwrap(),cm).unwrap()),
            "CLM" => Some(CallMoney::schedule(&to.unwrap(),cm).unwrap()),
            "CSH" => Some(Cash::schedule(&to.unwrap(),cm).unwrap()),
            "FUTUR" => Some(Future::schedule(&to.unwrap(),cm).unwrap()),
            "FXOUT" => Some(ForeignExchangeOutright::schedule(&to.unwrap(),cm).unwrap()),
            "LAM" => Some(LinearAmortizer::schedule(&to.unwrap(),cm).unwrap()),
            "LAX" => Some(ExoticLinearAmortizer::schedule(&to.unwrap(),cm).unwrap()),
            "NAM" => Some(NegativeAmortizer::schedule(&to.unwrap(),cm).unwrap()),
            "OPTNS" => Some(Optionx::schedule(&to.unwrap(),cm).unwrap()),
            "PAM" => Some(PrincipalAtMaturity::schedule(&to.unwrap(), cm).unwrap()),
            "STK" => Some(Stock::schedule(&to.unwrap(),cm).unwrap()),
            "SWAPS" => Some(Swap::schedule(&to.unwrap(),cm).unwrap()),
            "SWPPV" => Some(PlainVanillaInterestRateSwap::schedule(&to.unwrap(),cm).unwrap()),
            "UMP" => Some(BoundaryControlledSwitch::schedule(&to.unwrap(),cm).unwrap()),
            _ => None
        }

    }
    pub fn apply(events: Vec<ContractEvent>, cm: &ContractModel, observer: &RiskFactorModel) -> Option<Vec<ContractEvent>> {

        match cm.clone().contractType.unwrap().as_str() {
            "ANN" => Some(Annuity::apply(events, cm, observer)),
            "BCS" => Some(BoundaryControlledSwitch::apply(events, cm, observer)),
            "CPFL" => Some(CapFloor::apply(events, cm, observer)),
            "CEC" => Some(CreditEnhancementCollateral::apply(events, cm, observer)),
            "CEG" => Some(CreditEnhancementGuarantee::apply(events, cm, observer)),
            "CLM" => Some(CallMoney::apply(events, cm, observer)),
            "CSH" => Some(Cash::apply(events, cm, observer)),
            "FUTUR" => Some(Future::apply(events, cm, observer)),
            "FXOUT" => Some(ForeignExchangeOutright::apply(events, cm, observer))
            "LAM" => Some(LinearAmortizer::apply(events, cm, observer)),
            "LAX" => Some(ExoticLinearAmortizer::apply(events, cm, observer)),
            "NAM" => Some(NegativeAmortizer::apply(events, cm, observer)),
            "OPTNS" => Some(Optionx::apply(events, cm, observer)),
            "PAM" => Some(PrincipalAtMaturity::apply(events, cm, observer)),
            "STK" => Some(Stock::apply(events, cm, observer)),
            "SWAPS" => Some(Swap::apply(events, cm, observer)),
            "SWPPV" => Some(PlainVanillaInterestRateSwap::apply(events, cm, observer)),
            "UMP" => Some(BoundaryControlledSwitch::apply(events, cm, observer)),
            _ => None
        }
    }
}




