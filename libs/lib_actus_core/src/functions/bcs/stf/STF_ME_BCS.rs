use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

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
    ) {
        if states.boundaryMonitoringFlag.unwrap_or(false) {
            if let Some(contract_structure) = &model.contractStructure {
                if let Some(contract_reference) = contract_structure.iter().find(|e| {
                    e.reference_role == ReferenceRole::externalReferenceIndex
                }) {
                    let cbv = risk_factor_model.state_at(
                        contract_reference.get_object().as_string().unwrap(),
                        time,
                        states,
                        model,
                        true
                    );

                    let boundary_direction = model.boundaryDirection.as_ref().unwrap();
                    let boundary_value = model.boundaryValue.unwrap_or(0.0);

                    if (boundary_direction.clone() == BoundaryDirection::DECR(DECR) && cbv.clone().unwrap() <= boundary_value) ||
                        (boundary_direction.clone() == BoundaryDirection::INCR(INCR) && cbv.clone().unwrap() >= boundary_value) {

                        // Update state space
                        states.boundaryMonitoringFlag = Some(false);
                        states.boundaryCrossedFlag = Some(true);
                        states.statusDate = Some(*time);
                    }
                }
            }
        }

    }
}
