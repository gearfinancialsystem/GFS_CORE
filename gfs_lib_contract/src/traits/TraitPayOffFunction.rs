use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::traits::TraitExternalData::TraitExternalData;

pub trait TraitPayOffFunction
{
    fn new() -> Self;
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        related_contracts:  &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> Result<PayOff, ErrorContractEnum>;
}
