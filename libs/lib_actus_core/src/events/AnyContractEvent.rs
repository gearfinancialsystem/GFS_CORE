use crate::events::ContractEvent::ContractEvent;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use std::any::{Any, TypeId};
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_margining::CycleAnchorDateOfMargining::CycleAnchorDateOfMargining;
use crate::terms::grp_margining::CycleOfMargining::CycleOfMargining;
use crate::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use crate::terms::grp_notional_principal::ContractDealDate::ContractDealDate;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use crate::terms::grp_optionality::OptionExerciceEndDate::OptionExerciceEndDate;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

pub enum AnyContractEvent {
    InitialExchangeDate(ContractEvent<InitialExchangeDate, InitialExchangeDate>),
    MaturityDate(ContractEvent<MaturityDate, MaturityDate>),
    StatusDate(ContractEvent<StatusDate, StatusDate>),
    NonPerformingDate(ContractEvent<NonPerformingDate, NonPerformingDate>),
    CapitalizationEndDate(ContractEvent<CapitalizationEndDate, CapitalizationEndDate>),
    CycleAnchorDateOfInterestCalculationBase(ContractEvent<CycleAnchorDateOfInterestCalculationBase, CycleAnchorDateOfInterestCalculationBase>),
    CycleAnchorDateOfInterestPayment(ContractEvent<CycleAnchorDateOfInterestPayment,CycleAnchorDateOfInterestPayment>),
    CycleAnchorDateOfMargining(ContractEvent<CycleAnchorDateOfMargining,CycleAnchorDateOfMargining>),
    AmortizationDate(ContractEvent<AmortizationDate, AmortizationDate>),
    ContractDealDate(ContractEvent<ContractDealDate, ContractDealDate>),
    CycleAnchorDateOfPrincipalRedemption(ContractEvent<CycleAnchorDateOfPrincipalRedemption,CycleAnchorDateOfPrincipalRedemption>),
    PurchaseDate(ContractEvent<PurchaseDate, PurchaseDate>),
    TerminationDate(ContractEvent<TerminationDate, TerminationDate>),
    OptionExersiceEndDate(ContractEvent<OptionExerciceEndDate, OptionExerciceEndDate>),
    CycleAnchorDateOfOptionality(ContractEvent<CycleAnchorDateOfOptionality, CycleAnchorDateOfOptionality>),
    CycleAnchorDateOfRateReset(ContractEvent<CycleAnchorDateOfRateReset, CycleAnchorDateOfRateReset>),
    ExerciseDate(ContractEvent<ExerciseDate, ExerciseDate>)
    // Ajoutez d'autres variantes selon vos besoins
}

trait ConvertToAnyContractEvent {
    fn convert_to_any(self) -> Result<AnyContractEvent, String>;
}
// impl ConvertToAnyContractEvent for ContractEvent<InitialExchangeDate, InitialExchangeDate> {
//     fn convert_to_any(self) -> Result<AnyContractEvent, String> {
//         Ok(AnyContractEvent::InitialExchange(self))
//     }
// }
//
// impl ConvertToAnyContractEvent for ContractEvent<MaturityDate, MaturityDate> {
//     fn convert_to_any(self) -> Result<AnyContractEvent, String> {
//         Ok(AnyContractEvent::InterestPayment(self))
//     }
// }

impl AnyContractEvent {
    pub fn from_contract_event<T1, T2>(ce: ContractEvent<T1, T2>) -> Result<Self, String> where
        T1: TraitMarqueurIsoDatetime  + 'static,
        T2: TraitMarqueurIsoDatetime  + 'static,
        ContractEvent<T1, T2>: ConvertToAnyContractEvent,
    {
        ce.convert_to_any()
    }


}