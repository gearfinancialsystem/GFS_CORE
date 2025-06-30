use std::error::Error;
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
        if model.purchaseDate.is_some() {
            events.push(EventFactory::create_event(
                model.purchaseDate.clone(),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                model.contractID.as_ref(),
            ));
        }

        // Raw monitoring events
        let monitoring_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.boundaryMonitoringAnchorDate.clone(),
                model.boundaryMonitoringEndDate.clone(),
                model.boundaryMonitoringCycle.clone(),
                model.endOfMonthConvention.clone().unwrap(),
                true,
            ),
            EventType::ME,
            model.currency.as_ref(),
            Some(Rc::new(POF_AD_PAM)),
            Some(Rc::new(STF_ME_BCS)),
            model.businessDayAdjuster.as_ref().unwrap(),
            model.contractID.as_ref(),
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
                &model.dayCountConvention.clone().unwrap(),
                &model.businessDayAdjuster.clone().unwrap(),
            );
        }

        // Remove monitoring events
        events.retain(|e| e.eventType != EventType::ME);

        // Activating child legs based on boundaryEffect
        if states.boundaryCrossedFlag.unwrap() {
            match model.boundaryEffect.as_ref().unwrap() {
                BoundaryEffect::INFIL(INFIL) => {
                    states.boundaryLeg1ActiveFlag = Some(true);
                    states.boundaryLeg2ActiveFlag = Some(false);
                }
                BoundaryEffect::INSEL(INSEL) => {
                    states.boundaryLeg2ActiveFlag = Some(true);
                    states.boundaryLeg1ActiveFlag = Some(false);
                }
                BoundaryEffect::OUT(OUT) => {
                    states.boundaryLeg1ActiveFlag = Some(false);
                    states.boundaryLeg2ActiveFlag = Some(false);
                }
                _ => {}
            }
        }

        // First leg model
        let first_leg_model = model.contractStructure.clone().unwrap().iter()
            .find(|c| c.reference_role == ReferenceRole::FIL)
            .and_then(|c| c.object.clone().as_cm())
            .unwrap();

        let mut first_leg_schedule = Vec::new();

        // Second leg model
        let second_leg = model.contractStructure.clone().unwrap().iter()
            .find(|c| c.reference_role == ReferenceRole::SEL)
            .and_then(|c| c.object.clone().as_cm());

        let mut second_leg_schedule = Vec::new();
        let second_leg_model = second_leg.unwrap();

        // Create children event schedule based on boundary conditions
        if states.boundaryLeg1ActiveFlag.unwrap() == true {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturityDate.clone().map(|rc| (*rc).clone()),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contractType.clone().unwrap() != "PAM" {
                first_leg_schedule.push(EventFactory::create_event(
                    states.statusDate.clone(),
                    EventType::PRD,
                    first_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    first_leg_model.contractID.as_ref(),
                ));
            } else {
                first_leg_schedule.retain(|e| e.eventType != EventType::IED);
                first_leg_schedule.push(EventFactory::create_event(
                    states.statusDate.clone(),
                    EventType::IED,
                    first_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    first_leg_model.contractID.as_ref(),
                ));
            }

            first_leg_schedule.retain(|e| e.eventTime >= states.statusDate);

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer).unwrap();
            events.extend(first_leg_events);
        } else if states.boundaryLeg1ActiveFlag.clone().unwrap() == false
            && model.boundaryLegInitiallyActive.is_some()
            && model.boundaryLegInitiallyActive.clone().unwrap().to_stringx().unwrap() == ReferenceRole::FIL.to_stringx().unwrap()
        {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturityDate.clone().map(|rc| (*rc).clone()),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contractType.clone().unwrap() != "PAM" {
                first_leg_schedule.push(EventFactory::create_event(
                    model.purchaseDate.clone(),
                    EventType::PRD,
                    first_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    first_leg_model.contractID.as_ref(),
                ));
            }

            let td_event = EventFactory::create_event(
                states.statusDate.clone(),
                EventType::TD,
                first_leg_model.currency.as_ref(),
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                first_leg_model.contractID.as_ref(),
            );

            first_leg_schedule.retain(|e| e.compare_to(&td_event) != 1);
            first_leg_schedule.push(td_event);

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer);
            events.extend(first_leg_events.unwrap());
        }

        if states.boundaryLeg2ActiveFlag.clone().unwrap() == true {
            second_leg_schedule = ContractType::schedule(
                second_leg_model.maturityDate.clone().map(|rc| (*rc).clone()),
                &second_leg_model,
            ).unwrap();

            if second_leg_model.contractType.clone().unwrap().to_string() != "PAM"{
                second_leg_schedule.push(EventFactory::create_event(
                    states.statusDate.clone(),
                    EventType::PRD,
                    second_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    second_leg_model.contractID.as_ref(),
                ));
            } else {
                second_leg_schedule.retain(|e| e.eventType != EventType::IED);
                second_leg_schedule.push(EventFactory::create_event(
                    states.statusDate.clone(),
                    EventType::IED,
                    second_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    second_leg_model.contractID.as_ref(),
                ));
            }

            second_leg_schedule.retain(|e| e.eventTime >= states.statusDate);

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        } else if states.boundaryLeg2ActiveFlag.clone().unwrap() == false
            && model.boundaryLegInitiallyActive.is_some()
            && model.boundaryLegInitiallyActive.as_ref().unwrap().to_stringx().unwrap() == ReferenceRole::SEL.to_stringx().unwrap()
        {
            if second_leg_model.contractType.clone().unwrap() != "PAM" {
                second_leg_schedule.push(EventFactory::create_event(
                    model.purchaseDate.clone(),
                    EventType::PRD,
                    second_leg_model.currency.as_ref(),
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    second_leg_model.contractID.as_ref(),
                ));
            }

            let td_event = EventFactory::create_event(
                states.statusDate.clone(),
                EventType::TD,
                second_leg_model.currency.as_ref(),
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                second_leg_model.contractID.as_ref(),
            );

            second_leg_schedule.retain(|e| e.compare_to(&td_event) != 1);
            second_leg_schedule.push(td_event);

            second_leg_schedule = ContractType::schedule(
                second_leg_model.maturityDate.clone().map(|rc| (*rc).clone()),
                &second_leg_model,
            ).unwrap();

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        }

        // Termination of master contract
        if states.boundaryCrossedFlag.clone().unwrap() == true && model.boundaryEffect.clone().unwrap() != BoundaryEffect::INFIL(INFIL) {
            events.push(EventFactory::create_event(
                states.statusDate.clone(),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                model.contractID.as_ref(),
            ));
        } else {
            events.push(EventFactory::create_event(
                model.boundaryMonitoringEndDate.clone(),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                model.contractID.as_ref(),
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
        states.statusDate = model.statusDate;
        states.contractPerformance = model.contractPerformance;
        states.boundaryCrossedFlag = Some(false);
        states.boundaryMonitoringFlag = Some(true);

        if let role = &model.boundaryLegInitiallyActive.clone().unwrap().to_stringx().unwrap() {
            match role.as_str() {
                "FIL" => {
                    states.boundaryLeg1ActiveFlag = Some(true);
                    states.boundaryLeg2ActiveFlag = Some(false);
                }
                "SEL" => {
                    states.boundaryLeg2ActiveFlag = Some(true);
                    states.boundaryLeg1ActiveFlag = Some(false);
                }
                _ => {
                    states.boundaryLeg1ActiveFlag = Some(false);
                    states.boundaryLeg2ActiveFlag = Some(false);
                }
            }
        } else {
            states.boundaryLeg1ActiveFlag = Some(false);
            states.boundaryLeg2ActiveFlag = Some(false);
        }

        states
    }
}
