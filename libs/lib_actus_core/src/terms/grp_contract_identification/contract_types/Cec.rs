use std::error::Error;
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
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::types::IsoDatetime::IsoDatetime;

pub struct CEC;

impl CEC {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Maturity
        if model.exerciseDate.is_none() {
            events.push(EventFactory::create_event(
                Some(maturity),
                EventType::MD,
                model.currency.as_ref(),
                Some(Rc::new(POF_MD_CEG)),
                Some(Rc::new(STF_MD_CEG)),
                model.contractID.as_ref(),
            ));
        }

        // Exercise
        if let Some(exercise_date) = &model.exerciseDate {
            events.push(EventFactory::create_event(
                Some(exercise_date.clone()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                model.contractID.as_ref(),
            ));

            let settlement_period = model.settlementPeriod.clone().unwrap();
            let settlement_date = exercise_date.clone() + settlement_period.clone();

            events.push(EventFactory::create_event_with_convention(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
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
        events = Self::add_external_xd_event(model, events, observer, &maturity).unwrap();

        let mut states = Self::init_state_space(model, observer, &maturity).unwrap();

        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.dayCountConvention.clone().unwrap(),
                &model.businessDayAdjuster.clone().unwrap(),
            );
        }

        events
    }

    fn maturity(model: &ContractModel) -> IsoDatetime {

        let covered_contract_refs = model.contractStructure.clone().unwrap()
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

    fn init_state_space(
        model: &ContractModel,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<StateSpace, Box<dyn Error>> {
        let mut states = StateSpace::default();
        states.maturityDate = Some(maturity.clone());
        states.statusDate = model.statusDate.clone();

        if states.statusDate.unwrap() > states.maturityDate.unwrap() {
            states.notionalPrincipal = Some(0.0);
        } else {
            states.notionalPrincipal = Some(Self::calculate_notional_principal(
                model,
                observer,
                &states.statusDate.unwrap(),
            ));
        }

        states.exerciseAmount = model.exerciseAmount;
        states.exerciseDate = model.exerciseDate.clone();

        Ok(states)
    }

    pub fn calculate_notional_principal(
        model: &ContractModel,
        observer: &RiskFactorModel,
        time: &IsoDatetime,
    ) -> f64 {

        let covered_contract_refs = model.contractStructure.clone().unwrap()
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();

        let states_at_time_point: Vec<StateSpace> = covered_contract_refs
            .iter()
            .map(|c| c.get_state_space_at_time_point(time.clone(), observer))
            .collect();

        let role_sign = &model.contractRole.clone().unwrap().role_sign();
        let coverage = model.coverageOfCreditEnhancement.clone().unwrap();

        match model.guaranteedExposure {
            Some(GuaranteedExposure::NO(NO)) => coverage
                * role_sign
                * states_at_time_point
                .iter()
                .map(|s| s.notionalPrincipal.unwrap_or(0.0))
                .sum::<f64>(),
            Some(GuaranteedExposure::NI(NI)) => coverage
                * role_sign
                * (states_at_time_point
                .iter()
                .map(|s| s.notionalPrincipal.unwrap_or(0.0))
                .sum::<f64>()
                + states_at_time_point
                .iter()
                .map(|s| s.accruedInterest.unwrap_or(0.0))
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
        let covering_contract_refs = model.contractStructure.clone().unwrap()
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
        mut events: Vec<ContractEvent>,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let contract_identifiers: Vec<String> = model.contractStructure.clone().unwrap()
            .iter()
            .map(|c| c.get_contract_attribute("contractID").unwrap())
            .collect();

        let a_credit_event_type_covered = model.creditEventTypeCovered.clone().unwrap()
            .iter()
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();
        let credit_event_type_covered = a_credit_event_type_covered.get(0).unwrap();


        let observed_events = observer.events(model);

        let ce_events: Vec<ContractEvent> = observed_events
            .into_iter()
            .filter(|e| {
                contract_identifiers.contains(&e.contractID.clone().unwrap())
                    && &e.eventTime.unwrap() <= maturity
                    && e.states().contractPerformance.clone().unwrap().to_stringx().unwrap()
                    == credit_event_type_covered.to_stringx().unwrap()
            })
            .collect();

        if !ce_events.is_empty() {
            let ce_event = &ce_events[0];
            events.retain(|e| e.eventType != EventType::MD);

            events.push(EventFactory::create_event(
                Some(ce_event.eventTime.clone().unwrap()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEC)),
                model.contractID.as_ref(),
            ));

            let settlement_period = model.settlementPeriod.clone().unwrap();
            let settlement_date = ce_event.eventTime.clone().unwrap() + settlement_period;

            events.push(EventFactory::create_event_with_convention(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEC)),
                Some(Rc::new(STF_STD_CEC)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        Ok(events)
    }
}
