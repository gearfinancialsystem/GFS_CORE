use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_ME_BCS;

impl TraitStateTransitionFunction for STF_ME_BCS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> StateSpace {
        if states.boundaryMonitoringFlag.unwrap_or(false) {
            if let Some(contract_structure) = &model.contractStructure {
                if let Some(contract_reference) = contract_structure.iter().find(|e| {
                    e.referenceRole == ReferenceRole::ExternalReferenceIndex
                }) {
                    let cbv = risk_factor_model.state_at(
                        contract_reference.get_object(),
                        time,
                        states,
                        model,
                        true
                    );

                    let boundary_direction = model.boundaryDirection.as_ref().unwrap();
                    let boundary_value = model.boundaryValue.unwrap_or(0.0);

                    if (boundary_direction == "DECR" && cbv <= boundary_value) ||
                        (boundary_direction == "INCR" && cbv >= boundary_value) {

                        // Update state space
                        states.boundaryMonitoringFlag = Some(false);
                        states.boundaryCrossedFlag = Some(true);
                        states.statusDate = Some(*time);
                    }
                }
            }
        }

        StateSpace::copy_state_space(states)
    }
}
