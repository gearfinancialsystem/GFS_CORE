use std::error::Error;
use std::rc::Rc;
use crate::events::ContractEvent;
use crate::events::EventFactory;
use crate::events::EventType;
use crate::externals::RiskFactorModel;
use crate::functions::bcs::{POF_PRD_BCS, POF_TD_BCS, STF_ME_BCS, STF_TD_BCS};
use crate::functions::optns::POF_PRD_OPTNS;
use crate::functions::pam::{POF_AD_PAM, POF_IED_PAM, STF_IED_PAM};
use crate::functions::stk::STF_PRD_STK;
use crate::state_space::StateSpace;
use crate::types::{ContractReference, ContractTypeEnum, IsoDatetime, ReferenceRole};
use crate::attributes::ContractModel;
use crate::time::ScheduleFactory;

pub struct BoundaryControlledSwitch;

impl BoundaryControlledSwitch {
    pub fn schedule(
        to: &IsoDatetime,
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
        if states.boundaryCrossedFlag {
            match model.boundaryEffect.as_ref().unwrap().as_str() {
                "knockINFirstLeg" => {
                    states.boundaryLeg1ActiveFlag = true;
                    states.boundaryLeg2ActiveFlag = false;
                }
                "knockINSecondLeg" => {
                    states.boundaryLeg2ActiveFlag = true;
                    states.boundaryLeg1ActiveFlag = false;
                }
                "knockOUTCurrent" => {
                    states.boundaryLeg1ActiveFlag = false;
                    states.boundaryLeg2ActiveFlag = false;
                }
                _ => {}
            }
        }

        // First leg model
        let first_leg_model = model.contractStructure.iter()
            .find(|c| c.referenceRole == ReferenceRole::FIL)
            .and_then(|c| c.object.clone())
            .unwrap();

        let mut first_leg_schedule = Vec::new();

        // Second leg model
        let second_leg = model.contractStructure.iter()
            .find(|c| c.referenceRole == ReferenceRole::SEL)
            .and_then(|c| c.object.clone());

        let mut second_leg_schedule = Vec::new();
        let second_leg_model = second_leg.unwrap();

        // Create children event schedule based on boundary conditions
        if states.boundaryLeg1ActiveFlag {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturityDate.as_ref().unwrap(),
                &first_leg_model,
            ).unwrap();

            if !first_leg_model.contractType.eq(&ContractTypeEnum::PAM) {
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
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer);
            events.extend(first_leg_events);
        } else if !states.boundaryLeg1ActiveFlag
            && model.boundaryLegInitiallyActive.is_some()
            && model.boundaryLegInitiallyActive.as_ref().unwrap().eq(&ReferenceRole::FIL)
        {
            first_leg_schedule = ContractType::schedule(
                first_leg_model.maturityDate.as_ref().unwrap(),
                &first_leg_model,
            ).unwrap();

            if !first_leg_model.contractType.eq(&ContractTypeEnum::PAM) {
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
            events.extend(first_leg_events);
        }

        if states.boundaryLeg2ActiveFlag {
            second_leg_schedule = ContractType::schedule(
                second_leg_model.maturityDate.as_ref().unwrap(),
                &second_leg_model,
            ).unwrap();

            if !second_leg_model.contractType.eq(&ContractTypeEnum::PAM) {
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
            events.extend(second_leg_events);
        } else if !states.boundaryLeg2ActiveFlag
            && model.boundaryLegInitiallyActive.is_some()
            && model.boundaryLegInitiallyActive.as_ref().unwrap().eq(&ReferenceRole::SEL)
        {
            if !second_leg_model.contractType.eq(&ContractTypeEnum::PAM) {
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
                second_leg_model.maturityDate.as_ref().unwrap(),
                &second_leg_model,
            ).unwrap();

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events);
        }

        // Termination of master contract
        if states.boundaryCrossedFlag && model.boundaryEffect.as_ref().unwrap().ne("knockINFirstLeg") {
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
        states.boundaryCrossedFlag = false;
        states.boundaryMonitoringFlag = true;

        if let Some(role) = &model.boundaryLegInitiallyActive {
            match role {
                ReferenceRole::FIL => {
                    states.boundaryLeg1ActiveFlag = true;
                    states.boundaryLeg2ActiveFlag = false;
                }
                ReferenceRole::SEL => {
                    states.boundaryLeg2ActiveFlag = true;
                    states.boundaryLeg1ActiveFlag = false;
                }
                _ => {
                    states.boundaryLeg1ActiveFlag = false;
                    states.boundaryLeg2ActiveFlag = false;
                }
            }
        } else {
            states.boundaryLeg1ActiveFlag = false;
            states.boundaryLeg2ActiveFlag = false;
        }

        states
    }
}
