use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;

use crate::state_space::StateSpace::{BoundaryMonitoringFlag, StateSpace};
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_boundary::BoundaryValue::BoundaryValue;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct STF_ME_BCS;


impl TraitStateTransitionFunction for STF_ME_BCS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        
        if states.boundary_monitoring_flag.unwrap_or(false) {
            if let Some(contract_structure) = &model.contract_structure {
                if let Some(contract_reference) = contract_structure.0.iter().find(|e| {
                    e.reference_role == ReferenceRole::externalReferenceIndex
                }) {
                    let cbv = risk_factor_model.state_at(
                        contract_reference.get_object().as_string().unwrap().clone(),
                        time,
                        states,
                        model,
                        true
                    );

                    let boundary_direction = model.boundary_direction.clone().unwrap();
                    let boundary_value = if model.boundary_value.is_some() {
                        model.boundary_value.clone()
                    } else {BoundaryValue::new(0.0).ok()}.unwrap();



                    if (boundary_direction.clone() == BoundaryDirection::DECR(DECR) && cbv.clone() <= boundary_value.value()) ||
                        (boundary_direction.clone() == BoundaryDirection::INCR(INCR) && cbv.clone() >= boundary_value.value()) {

                        // Update state space
                        states.boundary_monitoring_flag = Some(false);
                        states.boundary_crossed_flag = BoundaryCrossedFlag::new(true).ok();
                        states.status_date = Some(StatusDate::from(*time));
                    }
                }
            }
        }

    }
}
