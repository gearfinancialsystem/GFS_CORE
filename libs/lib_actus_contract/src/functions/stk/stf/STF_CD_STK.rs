use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_CD_STK;

impl TraitStateTransitionFunction for STF_CD_STK {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    )  {
        states.contract_performance = Some(ContractPerformance::new("DF").expect("ok cp"));
        states.status_date = StatusDate::new(time.value()).ok();

    }
}
