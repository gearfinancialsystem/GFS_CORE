use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;


use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::contracts::Ceg::CEG;

#[allow(non_camel_case_types)]
pub struct STF_FP_CEG;

impl TraitStateTransitionFunction for STF_FP_CEG {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        if contract_terms.notional_principal.is_none() {
            states.notional_principal = NotionalPrincipal::new( Some(
                CEG::calculate_notional_principal(
                    contract_terms,
                    &contract_structure.clone().expect("should be one"),
                    &risk_factor_model.clone().expect("should have one"),
                    time,
                )
            ).unwrap()).ok();
        }

        states.fee_accrued = FeeAccrued::new(0.0).ok();
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
