use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::functions::cec::pof::POF_STD_CEC::POF_STD_CEC;
use crate::functions::cec::stf::STF_STD_CEC::STF_STD_CEC;
use crate::functions::cec::stf::STF_XD_CEC::STF_XD_CEC;
use crate::functions::ceg::pof::POF_MD_CEG::POF_MD_CEG;
use crate::functions::ceg::stf::STF_MD_CEG::STF_MD_CEG;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;

pub struct CEC;

impl TraitContractModel for CEC {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Maturity
        if model.exercise_date.is_none() {
            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(maturity),
                &EventType::MD,
                &model.currency,
                Some(Rc::new(POF_MD_CEG)),
                Some(Rc::new(STF_MD_CEG)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Exercise
        if let Some(exercise_date) = &model.exercise_date {
            let e: ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(exercise_date.clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = exercise_date.clone() + settlement_period.clone();

            let e: ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        Ok(events)
    }

    fn apply(
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let maturity = Self::maturity(model);
        events = Self::add_external_xd_event(model, events, observer, &maturity).unwrap();

        let mut states = Self::init_state_space(model, observer, &maturity).unwrap();

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

    fn init_state_space(
        model: &ContractModel,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<StateSpace, String> {
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

        states.exercise_amount = model.exercise_amount.clone();
        states.exercise_date = model.exercise_date.clone();

        Ok(states)
    }


}

impl CEC {

    fn maturity(model: &ContractModel) -> IsoDatetime {

        let covered_contract_refs = model.contract_structure.clone().unwrap()
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();


        let mut maturity_dates: Vec<IsoDatetime> = covered_contract_refs
            .iter()
            .map(|c| IsoDatetime::parse_from_str(c.get_contract_attribute("maturityDate").unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap())
            .collect();

        maturity_dates.sort();
        maturity_dates.last().unwrap().clone()
    }

    pub fn calculate_notional_principal(
        model: &ContractModel,
        observer: &RiskFactorModel,
        time: &IsoDatetime,
    ) -> f64 {

        let covered_contract_refs = model.contract_structure.clone().unwrap().0
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();

        let states_at_time_point: Vec<StateSpace> = covered_contract_refs
            .iter()
            .map(|c| c.get_state_space_at_time_point(time.clone(), observer))
            .collect();

        let role_sign = &model.contract_role.clone().unwrap().role_sign();
        let coverage = model.coverage_of_credit_enhancement.clone().unwrap();

        match model.guaranteed_exposure {
            Some(GuaranteedExposure::NO(NO)) => coverage
                * role_sign
                * states_at_time_point
                .iter()
                .map(|s| s.notional_principal.unwrap_or(0.0))
                .sum::<f64>(),
            Some(GuaranteedExposure::NI(NI)) => coverage
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
                    .map(|c| c.get_contract_attribute("marketObjectCode").unwrap().to_string())
                    .collect();

                coverage
                    * role_sign
                    * market_object_codes
                    .iter()
                    .map(|code| observer.state_at(code, time, &StateSpace::default(), model, true).unwrap())
                    .sum::<f64>()
            }
        }
    }

    pub fn calculate_market_value_covering_contracts(
        model: &ContractModel,
        observer: &RiskFactorModel,
        time: &IsoDatetime,
    ) -> f64 {
        let covering_contract_refs = model.contract_structure.clone().unwrap()
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVI)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();
        let market_object_codes: Vec<String> = covering_contract_refs
            .iter()
            .map(|e| e.get_contract_attribute("marketObjectCode").unwrap())
            .collect();

        market_object_codes
            .iter()
            .map(|code| observer.state_at(code, time, &StateSpace::default(), model, true).unwrap())
            .sum()
    }

    fn add_external_xd_event(
        model: &ContractModel,
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let contract_identifiers: Vec<String> = model.contract_structure.clone().unwrap().0
            .iter()
            .map(|c| c.get_contract_attribute("contract_id").unwrap())
            .collect();

        let a_credit_event_type_covered = model.credit_event_type_covered.clone().unwrap().0
            .iter()
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();
        let credit_event_type_covered = a_credit_event_type_covered.get(0).unwrap();


        let observed_events = observer.events(model);

        let ce_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = observed_events
            .into_iter()
            .filter(|e| {
                contract_identifiers.contains(&e.contract_id.clone().unwrap())
                    && &e.event_time.unwrap() <= maturity
                    && e.states().contract_performance.clone().unwrap().to_stringx().unwrap()
                    == credit_event_type_covered.to_stringx().unwrap()
            })
            .collect();

        if !ce_events.is_empty() {
            let ce_event = &ce_events[0];
            events.retain(|e| e.event_type != EventType::MD);

            let e = EventFactory::create_event(
                &Some(ce_event.event_time.clone().unwrap()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = ce_event.event_time.clone().unwrap() + settlement_period;

            let e = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        Ok(events)
    }
}
impl fmt::Display for CEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CEC")
    }
}