use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_boundary::BoundaryValue::BoundaryValue;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[allow(non_camel_case_types)]
pub struct STF_ME_BCS;


impl TraitStateTransitionFunction for STF_ME_BCS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        
        if states.boundary_monitoring_flag.unwrap_or(false) {
            if let Some(contract_structure) = &contract_structure {
                if let Some(contract_reference) = contract_structure.iter().find(|e| {
                    e.reference_role == ReferenceRole::externalReferenceIndex
                }) {


                    let mut cbv = None;
                    if let Some(rfm) = risk_factor_model {
                    cbv = rfm.state_at(
                        contract_reference.get_object().as_string().unwrap().clone(),
                            time,
                            states,
                            contract_terms,
                            true
                        );
                    } else {
                        cbv = None
                    }



                    let boundary_direction = contract_terms.boundary_direction.clone().unwrap();
                    let boundary_value = if contract_terms.boundary_value.is_some() {
                        contract_terms.boundary_value.clone()
                    } else {BoundaryValue::new(0.0).ok()}.unwrap();



                    if (boundary_direction.clone() == BoundaryDirection::DECR(DECR) && cbv.clone().unwrap() <= boundary_value.value()) ||
                        (boundary_direction.clone() == BoundaryDirection::INCR(INCR) && cbv.clone().unwrap() >= boundary_value.value()) {

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
