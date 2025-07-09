// use std::hash::Hasher;
// use std::hash::Hash;
// use std::rc::Rc;
// use std::cmp::Ordering;

// use crate::events::ContractEvent::ContractEvent;
// use crate::events::ContractEvent::EventTime;
// use crate::events::EventType::EventType;
// use crate::terms::grp_contract_identification::ContractID::ContractID;
// use crate::terms::grp_notional_principal::Currency::Currency;
// use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
// use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
// use crate::terms::grp_contract_identification::StatusDate::StatusDate;
// use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
// use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
// use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
// use crate::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
// use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
// use crate::terms::grp_margining::CycleAnchorDateOfMargining::CycleAnchorDateOfMargining;
// use crate::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
// use crate::terms::grp_notional_principal::ContractDealDate::ContractDealDate;
// use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
// use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
// use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
// use crate::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
// use crate::terms::grp_optionality::OptionExerciceEndDate::OptionExerciceEndDate;
// use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
// use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
// use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
// use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
// use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
// use crate::types::IsoDatetime::IsoDatetime;

// pub enum AnyContractEvent {
//     InitialExchangeDate(ContractEvent<InitialExchangeDate, InitialExchangeDate>),
//     MaturityDate(ContractEvent<MaturityDate, MaturityDate>),
//     StatusDate(ContractEvent<StatusDate, StatusDate>),
//     NonPerformingDate(ContractEvent<NonPerformingDate, NonPerformingDate>),
//     CapitalizationEndDate(ContractEvent<CapitalizationEndDate, CapitalizationEndDate>),
//     CycleAnchorDateOfInterestCalculationBase(ContractEvent<CycleAnchorDateOfInterestCalculationBase, CycleAnchorDateOfInterestCalculationBase>),
//     CycleAnchorDateOfInterestPayment(ContractEvent<CycleAnchorDateOfInterestPayment,CycleAnchorDateOfInterestPayment>),
//     CycleAnchorDateOfMargining(ContractEvent<CycleAnchorDateOfMargining,CycleAnchorDateOfMargining>),
//     AmortizationDate(ContractEvent<AmortizationDate, AmortizationDate>),
//     ContractDealDate(ContractEvent<ContractDealDate, ContractDealDate>),
//     CycleAnchorDateOfPrincipalRedemption(ContractEvent<CycleAnchorDateOfPrincipalRedemption,CycleAnchorDateOfPrincipalRedemption>),
//     PurchaseDate(ContractEvent<PurchaseDate, PurchaseDate>),
//     TerminationDate(ContractEvent<TerminationDate, TerminationDate>),
//     OptionExersiceEndDate(ContractEvent<OptionExerciceEndDate, OptionExerciceEndDate>),
//     CycleAnchorDateOfOptionality(ContractEvent<CycleAnchorDateOfOptionality, CycleAnchorDateOfOptionality>),
//     CycleAnchorDateOfRateReset(ContractEvent<CycleAnchorDateOfRateReset, CycleAnchorDateOfRateReset>),
//     ExerciseDate(ContractEvent<ExerciseDate, ExerciseDate>),
//     CycleAnchorDateOfFee(ContractEvent<CycleAnchorDateOfFee, CycleAnchorDateOfFee>),

//     // Ajoutez d'autres variantes selon vos besoins
// }

// trait ConvertToAnyContractEvent {
//     fn convert_to_any(self) -> Result<AnyContractEvent, String>;
// }


// impl AnyContractEvent {
    
//     pub fn from_contract_event<T1, T2>(ce: ContractEvent<T1, T2>) -> Result<Self, String> where
//         T1: TraitMarqueurIsoDatetime  + 'static,
//         T2: TraitMarqueurIsoDatetime  + 'static,
//         ContractEvent<T1, T2>: ConvertToAnyContractEvent,
//     {
//         ce.convert_to_any()
//     }

//     // pub fn value<T1, T2>(ace: AnyContractEvent) -> ContractEvent<T1, T2> {
//     //     match ace {
//     //         AnyContractEvent::InitialExchangeDate(v) => {
//     //             let vv = v.
//     //         },
//     //         AnyContractEvent::MaturityDate(ContractEvent<MaturityDate, MaturityDate>),
//     //         AnyContractEvent::StatusDate(ContractEvent<StatusDate, StatusDate>),
//     //         AnyContractEvent::NonPerformingDate(ContractEvent<NonPerformingDate, NonPerformingDate>),
//     //         AnyContractEvent::CapitalizationEndDate(ContractEvent<CapitalizationEndDate, CapitalizationEndDate>),
//     //         AnyContractEvent::CycleAnchorDateOfInterestCalculationBase(ContractEvent<CycleAnchorDateOfInterestCalculationBase, CycleAnchorDateOfInterestCalculationBase>),
//     //         AnyContractEvent::CycleAnchorDateOfInterestPayment(ContractEvent<CycleAnchorDateOfInterestPayment,CycleAnchorDateOfInterestPayment>),
//     //         AnyContractEvent::CycleAnchorDateOfMargining(ContractEvent<CycleAnchorDateOfMargining,CycleAnchorDateOfMargining>),
//     //         AnyContractEvent::AmortizationDate(ContractEvent<AmortizationDate, AmortizationDate>),
//     //         AnyContractEvent::ContractDealDate(ContractEvent<ContractDealDate, ContractDealDate>),
//     //         AnyContractEvent::CycleAnchorDateOfPrincipalRedemption(ContractEvent<CycleAnchorDateOfPrincipalRedemption,CycleAnchorDateOfPrincipalRedemption>),
//     //         AnyContractEvent::PurchaseDate(ContractEvent<PurchaseDate, PurchaseDate>),
//     //         AnyContractEvent::TerminationDate(ContractEvent<TerminationDate, TerminationDate>),
//     //         AnyContractEvent::OptionExersiceEndDate(ContractEvent<OptionExerciceEndDate, OptionExerciceEndDate>),
//     //         AnyContractEvent::CycleAnchorDateOfOptionality(ContractEvent<CycleAnchorDateOfOptionality, CycleAnchorDateOfOptionality>),
//     //         AnyContractEvent::CycleAnchorDateOfRateReset(ContractEvent<CycleAnchorDateOfRateReset, CycleAnchorDateOfRateReset>),
//     //         AnyContractEvent::ExerciseDate(ContractEvent<ExerciseDate, ExerciseDate>),
//     //         AnyContractEvent::CycleAnchorDateOfFee(ContractEvent<CycleAnchorDateOfFee, CycleAnchorDateOfFee>),
//     //     }
//     // }
// }

// // Implémentation des traits pour la comparaison

// impl<T1, T2> PartialOrd for AnyContractEvent
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl<T1, T2> Ord for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.epoch_offset.cmp(&other.epoch_offset)
//     }
// }

// // Implémentation manuelle de PartialEq pour ContractEvent
// impl<T1, T2> PartialEq for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.contract_id == other.contract_id
//             && self.currency == other.currency
//             && self.event_time == other.event_time
//             && self.event_type == other.event_type
//             && self.schedule_time == other.schedule_time
//             // Comparer les pointeurs des traits dynamiques (optionnel)
//             && Rc::ptr_eq(&self.fpayoff.clone().unwrap(), &other.fpayoff.clone().unwrap())
//             && Rc::ptr_eq(&self.fstate.clone().unwrap(), &other.fstate.clone().unwrap())
//     }
// }
// impl<T1, T2> Eq for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {}

// impl<T1, T2> Hash for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.contract_id.hash(state);
//         self.currency.hash(state);
//         self.event_time.clone().hash(state);
//         self.event_type.hash(state);
//         self.schedule_time.hash(state);

//         // Hasher les pointeurs des traits dynamiques
//         Rc::as_ptr(&self.fpayoff.clone().unwrap()).hash(state);
//         Rc::as_ptr(&self.fstate.clone().unwrap()).hash(state);
//     }
// }