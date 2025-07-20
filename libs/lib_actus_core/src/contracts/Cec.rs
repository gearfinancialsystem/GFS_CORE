use std::cmp::{Ordering, PartialOrd};
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractTerms::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::functions::cec::pof::POF_STD_CEC::POF_STD_CEC;
use crate::functions::cec::stf::STF_STD_CEC::STF_STD_CEC;
use crate::functions::cec::stf::STF_XD_CEC::STF_XD_CEC;
use crate::functions::ceg::pof::POF_MD_CEG::POF_MD_CEG;
use crate::functions::ceg::stf::STF_MD_CEG::STF_MD_CEG;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance::MA;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

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
                &Some(maturity.value()),
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
            let settlement_date = exercise_date + &settlement_period;

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
        observer: &DataObserver,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        let maturity = Self::maturity(model);
        events = Self::add_external_xd_event(model, events, observer, &maturity.value()).unwrap();

        let mut states = Self::init_state_space(model, observer, &Some(Rc::new(maturity))).expect("Failed to initialize state space");


        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        Ok(events)
    }

    fn init_state_space(
        model: &ContractModel,
        observer: &DataObserver,
        maturity: &Option<Rc<MaturityDate>>,
    ) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        states.maturity_date = Some(maturity.clone().unwrap().as_ref().clone());
        states.status_date = model.status_date.clone();

        if states.status_date.clone().unwrap().value() > states.clone().maturity_date.unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();
        } else {
            states.notional_principal = NotionalPrincipal::new(Self::calculate_notional_principal(
                model,
                observer,
                &states.status_date.clone().unwrap().value(),
            )).ok();
        }

        states.exercise_amount = model.exercise_amount.clone();
        states.exercise_date = model.exercise_date.clone();

        Ok(states)
    }


}

impl CEC {

    fn maturity(model: &ContractModel) -> MaturityDate {

        let covered_contract_refs = model.contract_structure.clone().unwrap().0
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();


        let mut maturity_dates: Vec<IsoDatetime> = covered_contract_refs
            .iter()
            .map(|c| {
                let a = c.object.as_cm().unwrap().maturity_date.clone().unwrap().clone().deref().clone().value();
                a
            }
            )
            .collect();

        maturity_dates.sort();
        MaturityDate::new(maturity_dates.last().unwrap().clone()).ok().unwrap()
    }

    pub fn calculate_notional_principal(
        model: &ContractModel,
        observer: &DataObserver,
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
            Some(GuaranteedExposure::NO(NO)) => {
                coverage.value()
                * role_sign
                * states_at_time_point
                .iter()
                .map(|s| {
                    if s.notional_principal.is_none() {
                        NotionalPrincipal::new(0.0).ok().unwrap().value()
                    }
                    else {
                        s.notional_principal.clone().unwrap().value()
                    }
                })
                .sum::<f64>()
            },
            Some(GuaranteedExposure::NI(NI)) => {
                coverage.value() * role_sign * (
                    states_at_time_point
                    .iter()
                    .map(|s| {
                        if s.notional_principal.is_none() {
                            NotionalPrincipal::new(0.0).ok().unwrap().value()
                        }
                        else {
                            s.notional_principal.clone().unwrap().value()
                        }
                        })
                    .sum::<f64>() + states_at_time_point
                    .iter()
                    .map(|s| {
                        if s.accrued_interest.is_none() {
                            AccruedInterest::new(0.0).ok().unwrap().value()
                        }
                        else {
                            s.accrued_interest.clone().unwrap().value()
                        }
                    })
                    .sum::<f64>())
                },
            _ => {
                let market_object_codes: Vec<String> = covered_contract_refs
                .iter()
                .map(|c| {
                    let a = c.object.as_cm();
                    c.object.as_cm().unwrap().market_object_code.clone().unwrap().value()
                })
                .collect();

                coverage.value()
                    * role_sign
                    * market_object_codes
                    .iter()
                    .map(|code| 
                        observer.state_at(
                            code.clone(), 
                            time, 
                            &StateSpace::default(), 
                            model, 
                            true))
                    .sum::<f64>()
            }
        }
    }

    pub fn calculate_market_value_covering_contracts(
        model: &ContractModel,
        observer: &DataObserver,
        time: &IsoDatetime,
    ) -> f64 {
        let covering_contract_refs = model.contract_structure.clone().unwrap().0
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVI)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();
        let market_object_codes: Vec<String> = covering_contract_refs
            .iter()
            .map(|e| {
                e.object.as_cm().unwrap().market_object_code.clone().unwrap().value()
            })
            .collect();

        market_object_codes
            .iter()
            .map(|code| 
                observer.state_at(
                    code.clone(), 
                    time, 
                    &StateSpace::default(), 
                    model, 
                    true))
            .sum()
    }

    fn add_external_xd_event(
        model: &ContractModel,
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        observer: &DataObserver,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let contract_identifiers: Vec<String> = model.contract_structure.clone().unwrap().0
            .iter()
            .map(|c| {
                c.object.as_cm().unwrap().contract_id.clone().unwrap().value()
            })
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
                contract_identifiers.contains(&e.contract_id.clone().unwrap().value())
                    && &e.event_time.unwrap() <= maturity
                    && e.states().contract_performance.clone().unwrap().to_stringx().unwrap()
                    == credit_event_type_covered.to_string()
            })
            .collect();

        if !ce_events.is_empty() {
            let ce_event = &ce_events[0];
            events.retain(|e| e.event_type != EventType::MD);

            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
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
            let event_time = ce_event.event_time.clone().unwrap();
            let settlement_date = event_time + *settlement_period;

            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
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