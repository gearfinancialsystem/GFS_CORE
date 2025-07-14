
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::functions::ceg::pof::POF_FP_CEG::POF_FP_CEG;
use crate::functions::ceg::pof::POF_MD_CEG::POF_MD_CEG;
use crate::functions::ceg::pof::POF_STD_CEG::POF_STD_CEG;
use crate::functions::ceg::stf::STF_FP_CEG::STF_FP_CEG;
use crate::functions::ceg::stf::STF_MD_CEG::STF_MD_CEG;
use crate::functions::ceg::stf::STF_PRD_CEG::STF_PRD_CEG;
use crate::functions::ceg::stf::STF_STD_CEG::STF_STD_CEG;
use crate::functions::ceg::stf::STF_XD_CEG::STF_XD_CEG;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;

pub struct CEG;

impl TraitContractModel for CEG {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let maturity = Self::maturity(model);

        // Purchase
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_CEG)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Fees (if specified)
        if !(model.fee_rate.is_none() || model.fee_rate.clone().unwrap().value() == 0.0) {
            let start_date = if model.cycle_anchor_date_of_fee.is_none() && model.cycle_of_fee.is_none() {
                None
            } else if model.cycle_anchor_date_of_fee.is_none() {
                Some( (model.purchase_date.clone().unwrap() + model.cycle_of_fee.clone().unwrap().value().extract_period().unwrap() ).value() )


            } else {
                Some(model.cycle_anchor_date_of_fee.clone().unwrap().value())
            };

            let end_date = if model.exercise_date.is_none() {
                Some(maturity.clone().value())
            } else {
                Some(model.exercise_date.clone().unwrap().value())
            };

            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &start_date,
                    &end_date,
                    &model.cycle_of_fee,
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_CEG)),
                Some(Rc::new(STF_FP_CEG)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Maturity
        if model.exercise_date.is_none() {
            let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
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
            let e : ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(exercise_date.clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEG)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = exercise_date.clone() + settlement_period.clone().value().clone();
            let e: ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        Ok(events)
    }

    fn apply(
        events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let maturity = Self::maturity(model);
        let mut events = Self::add_external_xd_event(model, events, observer, &maturity).unwrap();

        let mut states = Self::init_state_space(model, observer, &Some(Rc::new(maturity) )).expect("Failed to initialize state_space");

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
        maturity: &Option<Rc<MaturityDate>>,
    ) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        states.maturity_date = Some(maturity.clone().unwrap().as_ref().clone());
        states.status_date = model.status_date.clone();

        if states.status_date.clone().unwrap().value() > states.maturity_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();
        } else if model.notional_principal.is_some() {
            let role_sign = &model.contract_role.clone().unwrap().role_sign();
            states.notional_principal =
                NotionalPrincipal::new(
                model.coverage_of_credit_enhancement.clone().unwrap().value()
                    * role_sign
                    * model.notional_principal.clone().unwrap().value(),
            ).ok();
        } else {
            states.notional_principal = NotionalPrincipal::new(Self::calculate_notional_principal(
                &states,
                &model,
                &observer,
                &states.status_date.clone().unwrap().value(),
            )).ok();
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }

        states.exercise_amount = model.exercise_amount.clone();
        states.exercise_date = model.exercise_date.clone();

        Ok(states)
    }


}

impl CEG {
    pub fn calculate_notional_principal(
        states: &StateSpace,
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

        match &model.guaranteed_exposure {
            Some(GuaranteedExposure::NO(NO)) => coverage.value()
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
                } )
                .sum::<f64>(),
            Some(GuaranteedExposure::NI(NI)) => coverage.value()
                * role_sign
                * (states_at_time_point
                .iter()
                .map(|s| {
                    if s.notional_principal.is_none() {
                        NotionalPrincipal::new(0.0).ok().unwrap().value()
                    }
                    else {
                        s.notional_principal.clone().unwrap().value()
                    }
                } )
                .sum::<f64>()
                + states_at_time_point
                .iter()
                .map(|s| {
                    if s.accrued_interest.is_none() {
                        AccruedInterest::new(0.0).ok().unwrap().value()
                    }
                    else {
                        s.accrued_interest.clone().unwrap().value()
                    }
                } )
                .sum::<f64>()),
            _ => {
                let market_object_codes: Vec<String> = covered_contract_refs
                    .iter()
                    .map(|c| {
                        c.object.as_cm().unwrap().market_object_code.clone().unwrap().value()
                    } )
                    .collect();

                coverage.value()
                    * role_sign
                    * market_object_codes
                    .iter()
                    .map(|code| observer.state_at(code, time, states, model, true).unwrap())
                    .sum::<f64>()
            }
        }
    }

    fn add_external_xd_event(
        model: &ContractModel,
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        observer: &RiskFactorModel,
        maturity: &MaturityDate,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let contract_identifiers: Vec<String> = model.contract_structure.clone().unwrap().0
            .iter()
            .map(|c|     {
                let a = c.object.as_cm().unwrap().contract_id.clone().unwrap().value();
                a
            })
            .collect();

        let credit_event_type_covered = model.credit_event_type_covered.clone().unwrap().values()[0].clone();

        let observed_events = observer.events(model);

        let ce_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = observed_events
            .into_iter()
            .filter(|e| {
                contract_identifiers.contains(&e.contract_id.clone().unwrap().value())
                    && &e.event_time.clone().unwrap() <= &maturity.value()
                    && e.states().contract_performance.clone().unwrap().to_stringx().unwrap()
                    == credit_event_type_covered.clone().to_string()
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
                Some(Rc::new(STF_XD_CEG)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_owned());

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = ce_event.event_time.clone().unwrap() + settlement_period.value().clone();

            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        Ok(events)
    }

    fn maturity(model: &ContractModel) -> MaturityDate {
        if let Some(maturity_date) = model.maturity_date.clone().map(|rc| (*rc).clone()) {
            maturity_date.clone()
        } else {
            let covered_contract_refs = model.contract_structure.clone().unwrap().0
                .iter()
                .filter(|e| e.reference_role == ReferenceRole::COVE)
                .map(|cr| cr.clone())
                .collect::<Vec<_>>();

            let mut maturity_dates: Vec<IsoDatetime> = covered_contract_refs
                .iter()
                .map(|c|
                         {
                             let a = c.object.as_cm().unwrap().maturity_date.clone().unwrap().deref().value();
                             a
                         }


                    //c.get_contract_attribute("maturityDate").unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap()

                )
                .collect();

            maturity_dates.sort();
            MaturityDate::new(*maturity_dates.last().clone().unwrap()).ok().unwrap()
        }
    }

}

impl fmt::Display for CEG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CEG")
    }
}