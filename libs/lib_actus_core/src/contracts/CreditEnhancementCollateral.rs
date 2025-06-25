use std::error::Error;
use std::rc::Rc;
use chrono::{DateTime, Duration, Local};
use crate::events::{ContractEvent, EventFactory, EventType};
use crate::externals::RiskFactorModel;
use crate::state_space::StateSpace;
use crate::attributes::ContractModel;
use crate::types::{ContractReference, GuaranteedExposure, ReferenceRole, CreditEventTypeCovered, IsoDatetime};
use crate::functions::cec::{POF_MD_CEG, STF_MD_CEG, STF_XD_CEC, STF_STD_CEC};
use crate::functions::ceg::POF_XD_OPTNS;
use crate::functions::optns::POF_STD_CEC;
use crate::conventions::contractrole::ContractRoleConvention;

pub struct CreditEnhancementCollateral;

impl CreditEnhancementCollateral {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Maturity
        if model.exercise_date.is_none() {
            events.push(EventFactory::create_event(
                Some(maturity),
                EventType::MD,
                model.currency.as_ref(),
                Some(Rc::new(POF_MD_CEG)),
                Some(Rc::new(STF_MD_CEG)),
                model.contract_id.as_ref(),
            ));
        }

        // Exercise
        if let Some(exercise_date) = &model.exercise_date {
            events.push(EventFactory::create_event(
                Some(exercise_date.clone()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                model.contract_id.as_ref(),
            ));

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = exercise_date.plus_period(&settlement_period);
            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                model.business_day_adjuster.as_ref().unwrap(),
                model.contract_id.as_ref(),
            ));
        }

        Ok(events)
    }

    pub fn apply(
        mut events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let maturity = Self::maturity(model);
        events = Self::add_external_xd_event(model, events, observer, &maturity)?;

        let mut states = Self::init_state_space(model, observer, &maturity)?;

        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone().unwrap(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        Ok(events)
    }

    fn maturity(model: &ContractModel) -> IsoDatetime {
        let covered_contract_refs: Vec<&ContractReference> = model.contract_structure
            .iter()
            .filter(|ref| ref.reference_role == ReferenceRole::COVE)
            .collect();

        let mut maturity_dates: Vec<IsoDatetime> = covered_contract_refs
            .iter()
            .map(|c| c.get_contract_attribute("maturityDate").parse().unwrap())
            .collect();

        maturity_dates.sort();
        maturity_dates.last().unwrap().clone()
    }

    fn init_state_space(
        model: &ContractModel,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<StateSpace, Box<dyn Error>> {
        let mut states = StateSpace::default();
        states.maturity_date = Some(maturity.clone());
        states.status_date = model.status_date.clone();

        if states.status_date.unwrap() > states.maturity_date.unwrap() {
            states.notional_principal = Some(0.0);
        } else {
            states.notional_principal = Some(Self::calculate_notional_principal(
                model,
                observer,
                &states.status_date.unwrap(),
            ));
        }

        states.exercise_amount = model.exercise_amount;
        states.exercise_date = model.exercise_date.clone();

        Ok(states)
    }

    fn calculate_notional_principal(
        model: &ContractModel,
        observer: &RiskFactorModel,
        time: &IsoDatetime,
    ) -> f64 {
        let covered_contract_refs: Vec<&ContractReference> = model.contract_structure
            .iter()
            .filter(|ref| ref.reference_role == ReferenceRole::COVE)
            .collect();

        let states_at_time_point: Vec<StateSpace> = covered_contract_refs
            .iter()
            .map(|c| c.get_state_space_at_timepoint(time, observer))
            .collect();

        let role_sign = ContractRoleConvention::role_sign(&model.contract_role);
        let coverage = model.coverage_of_credit_enhancement.unwrap();

        match model.guaranteed_exposure {
            GuaranteedExposure::NO => coverage
                * role_sign
                * states_at_time_point
                .iter()
                .map(|s| s.notional_principal.unwrap_or(0.0))
                .sum::<f64>(),
            GuaranteedExposure::NI => coverage
                * role_sign
                * (states_at_time_point
                .iter()
                .map(|s| s.notional_principal.unwrap_or(0.0))
                .sum::<f64>()
                + states_at_time_point
                .iter()
                .map(|s| s.accrued_interest.unwrap_or(0.0))
                .sum::<f64>()),
            _ => {
                let market_object_codes: Vec<String> = covered_contract_refs
                    .iter()
                    .map(|c| c.get_contract_attribute("marketObjectCode"))
                    .collect();

                coverage
                    * role_sign
                    * market_object_codes
                    .iter()
                    .map(|code| observer.state_at(code, time, &StateSpace::default(), model, true))
                    .sum::<f64>()
            }
        }
    }

    fn calculate_market_value_covering_contracts(
        model: &ContractModel,
        observer: &RiskFactorModel,
        time: &IsoDatetime,
    ) -> f64 {
        let covering_contract_refs: Vec<&ContractReference> = model.contract_structure
            .iter()
            .filter(|ref| ref.reference_role == ReferenceRole::COVI)
            .collect();

        let market_object_codes: Vec<String> = covering_contract_refs
            .iter()
            .map(|ref| ref.get_contract_attribute("marketObjectCode"))
            .collect();

        market_object_codes
            .iter()
            .map(|code| observer.state_at(code, time, &StateSpace::default(), model, true))
            .sum()
    }

    fn add_external_xd_event(
        model: &ContractModel,
        mut events: Vec<ContractEvent>,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let contract_identifiers: Vec<String> = model.contract_structure
            .iter()
            .map(|c| c.get_contract_attribute("contractID"))
            .collect();

        let credit_event_type_covered = model.credit_event_type_covered[0].clone();

        let observed_events = observer.events(model);

        let ce_events: Vec<ContractEvent> = observed_events
            .into_iter()
            .filter(|e| {
                contract_identifiers.contains(&e.contract_id)
                    && &e.event_time <= maturity
                    && e.states.contract_performance.to_string()
                    == credit_event_type_covered.to_string()
            })
            .collect();

        if !ce_events.is_empty() {
            let ce_event = &ce_events[0];
            events.retain(|e| e.event_type != EventType::MD);

            events.push(EventFactory::create_event(
                Some(ce_event.event_time.clone()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                model.contract_id.as_ref(),
            ));

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = ce_event.event_time.plus_period(&settlement_period);

            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                model.business_day_adjuster.as_ref().unwrap(),
                model.contract_id.as_ref(),
            ));
        }

        Ok(events)
    }
}
