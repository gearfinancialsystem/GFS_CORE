use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;

use crate::attributes::reference_role::ReferenceRole::ReferenceRole;

use crate::functions::bcs::pof::POF_PRD_BCS::POF_PRD_BCS;
use crate::functions::bcs::pof::POF_TD_BCS::POF_TD_BCS;
use crate::functions::bcs::stf::STF_ME_BCS::STF_ME_BCS;
use crate::functions::bcs::stf::STF_TD_BCS::STF_TD_BCS;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::pam::pof::POF_AD_PAM::POF_AD_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;
use crate::terms::grp_contract_identification::contract_types::Ann::ANN;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::IsoDatetime::IsoDatetime;

pub struct BCS;

impl BCS {
    pub fn schedule(
        _to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Purchase date event of master contract
        if model.purchase_date.is_some() {
            events.push(EventFactory::create_event(
                &model.purchase_date.clone(),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                &model.contract_id,
            ));
        }

        // Raw monitoring events
        let monitoring_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                &model.boundary_monitoring_anchor_date,
                &model.boundary_monitoring_end_date,
                &model.boundary_monitoring_cycle,
                &model.end_of_month_convention.clone().unwrap(),
                true,
            ),
            EventType::ME,
            &model.currency,
            Some(Rc::new(POF_AD_PAM)),
            Some(Rc::new(STF_ME_BCS)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        events.extend(monitoring_events);

        Ok(events)
    }

    pub fn apply(
        mut events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_state_space(model);

        // Sort the events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone().unwrap(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        // Remove monitoring events
        events.retain(|e| e.event_type != EventType::ME);

        // Activating child legs based on boundaryEffect
        if states.boundary_crossed_flag.unwrap() {
            match model.boundaryEffect.as_ref().unwrap() {
                BoundaryEffect::INFIL(INFIL) => {
                    states.boundary_leg1_active_flag = Some(true);
                    states.boundary_leg2_active_flag = Some(false);
                }
                BoundaryEffect::INSEL(INSEL) => {
                    states.boundary_leg2_active_flag = Some(true);
                    states.boundary_leg1_active_flag = Some(false);
                }
                BoundaryEffect::OUT(OUT) => {
                    states.boundary_leg1_active_flag = Some(false);
                    states.boundary_leg2_active_flag = Some(false);
                }
                _ => {}
            }
        }

        // First leg model
        let first_leg_model = model.contract_structure.clone().unwrap().iter()
            .find(|c| c.reference_role == ReferenceRole::FIL)
            .and_then(|c| c.object.clone().as_cm())
            .unwrap();

        let mut first_leg_schedule = Vec::new();

        // Second leg model
        let second_leg = model.contract_structure.clone().unwrap().iter()
            .find(|c| c.reference_role == ReferenceRole::SEL)
            .and_then(|c| c.object.clone().as_cm());

        let mut second_leg_schedule = Vec::new();
        let second_leg_model = second_leg.unwrap();

        // Create children event schedule based on boundary conditions
        if states.boundary_leg1_active_flag.unwrap() == true {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturity_date.clone().map(|rc| (*rc).clone()),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contractType.clone().unwrap() != "PAM" {
                first_leg_schedule.push(EventFactory::create_event(
                    states.status_date.clone(),
                    EventType::PRD,
                    first_leg_&model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    first_leg_&model.contract_id,
                ));
            } else {
                first_leg_schedule.retain(|e| e.event_type != EventType::IED);
                first_leg_schedule.push(EventFactory::create_event(
                    states.status_date.clone(),
                    EventType::IED,
                    first_leg_schedule&model.currency,
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    first_leg_&model.contract_id,
                ));
            }

            first_leg_schedule.retain(|e| e.event_time >= states.status_date);

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer).unwrap();
            events.extend(first_leg_events);
        } else if states.boundary_leg1_active_flag.clone().unwrap() == false
            && model.boundary_leg_initially_active.is_some()
            && model.boundary_leg_initially_active.clone().unwrap().to_stringx().unwrap() == ReferenceRole::FIL.to_stringx().unwrap()
        {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturity_date.clone().map(|rc| (*rc).clone()),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contractType.clone().unwrap() != "PAM" {
                first_leg_schedule.push(EventFactory::create_event(
                    model.purchase_date.clone(),
                    EventType::PRD,
                    first_leg_&model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    first_leg_&model.contract_id,
                ));
            }

            let td_event = EventFactory::create_event(
                states.status_date.clone(),
                EventType::TD,
                first_leg_&model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                first_leg_&model.contract_id,
            );

            first_leg_schedule.retain(|e| e.compare_to(&td_event) != 1);
            first_leg_schedule.push(td_event);

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer);
            events.extend(first_leg_events.unwrap());
        }

        if states.boundary_leg2_active_flag.clone().unwrap() == true {
            second_leg_schedule = ContractType::schedule(
                second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()),
                &second_leg_model,
            ).unwrap();

            if second_leg_model.contractType.clone().unwrap().to_string() != "PAM"{
                second_leg_schedule.push(EventFactory::create_event(
                    states.status_date.clone(),
                    EventType::PRD,
                    second_leg_&model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    second_leg_&model.contract_id,
                ));
            } else {
                second_leg_schedule.retain(|e| e.event_type != EventType::IED);
                second_leg_schedule.push(EventFactory::create_event(
                    states.status_date.clone(),
                    EventType::IED,
                    second_leg_&model.currency,
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    second_leg_&model.contract_id,
                ));
            }

            second_leg_schedule.retain(|e| e.event_time >= states.status_date);

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        } else if states.boundary_leg2_active_flag.clone().unwrap() == false
            && model.boundary_leg_initially_active.is_some()
            && model.boundary_leg_initially_active.as_ref().unwrap().to_stringx().unwrap() == ReferenceRole::SEL.to_stringx().unwrap()
        {
            if second_leg_model.contractType.clone().unwrap() != "PAM" {
                second_leg_schedule.push(EventFactory::create_event(
                    model.purchase_date.clone(),
                    EventType::PRD,
                    second_leg_&model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    second_leg_&model.contract_id,
                ));
            }

            let td_event = EventFactory::create_event(
                states.status_date.clone(),
                EventType::TD,
                second_leg_&model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                second_leg_&model.contract_id,
            );

            second_leg_schedule.retain(|e| e.compare_to(&td_event) != 1);
            second_leg_schedule.push(td_event);

            second_leg_schedule = ContractType::schedule(
                second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()),
                &second_leg_model,
            ).unwrap();

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        }

        // Termination of master contract
        if states.boundary_crossed_flag.clone().unwrap() == true && model.boundaryEffect.clone().unwrap() != BoundaryEffect::INFIL(INFIL) {
            events.push(EventFactory::create_event(
                states.status_date.clone(),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &model.contract_id,
            ));
        } else {
            events.push(EventFactory::create_event(
                model.boundary_monitoring_end_date.clone(),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &model.contract_id,
            ));
        }

        // Sort the events according to their time sequence
        events.sort();

        // Return post events states
        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        // Initialize state variables
        states.status_date = model.status_date;
        states.contract_performance = model.contract_performance;
        states.boundary_crossed_flag = Some(false);
        states.boundary_monitoring_flag = Some(true);

        if let role = &model.boundary_leg_initially_active.clone().unwrap().to_stringx().unwrap() {
            match role.as_str() {
                "FIL" => {
                    states.boundary_leg1_active_flag = Some(true);
                    states.boundary_leg2_active_flag = Some(false);
                }
                "SEL" => {
                    states.boundary_leg2_active_flag = Some(true);
                    states.boundary_leg1_active_flag = Some(false);
                }
                _ => {
                    states.boundary_leg1_active_flag = Some(false);
                    states.boundary_leg2_active_flag = Some(false);
                }
            }
        } else {
            states.boundary_leg1_active_flag = Some(false);
            states.boundary_leg2_active_flag = Some(false);
        }

        states
    }
}
impl fmt::Display for BCS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BCS")
    }
}
