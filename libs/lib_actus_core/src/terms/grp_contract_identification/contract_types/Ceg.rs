use std::error::Error;
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
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;

pub struct CEG;

impl CEG {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();
        let maturity = Self::maturity(model);

        // Purchase
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_CEG)),
                model.contractID.as_ref(),
            ));
        }

        // Fees (if specified)
        if !(model.feeRate.is_none() || model.feeRate.unwrap() == 0.0) {
            let start_date = if model.cycleAnchorDateOfFee.is_none() && model.cycleOfFee.is_none() {
                None
            } else if model.cycleAnchorDateOfFee.is_none() {
                Some(model.purchaseDate.unwrap() + CycleUtils::parse_period(&model.cycleOfFee.as_ref().unwrap()).unwrap())


            } else {
                model.cycleAnchorDateOfFee.clone()
            };

            let end_date = if model.exerciseDate.is_none() {
                Some(maturity.clone())
            } else {
                model.exerciseDate.clone()
            };

            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    start_date,
                    end_date,
                    model.cycleOfFee.clone(),
                    model.endOfMonthConvention.clone().unwrap(),
                    false,
                ),
                EventType::FP,
                model.currency.as_ref(),
                Some(Rc::new(POF_FP_CEG)),
                Some(Rc::new(STF_FP_CEG)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            );

            events.extend(fee_events);
        }

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
                Some(Rc::new(STF_XD_CEG)),
                model.contractID.as_ref(),
            ));

            let settlement_period = model.settlementPeriod.clone().unwrap();
            let settlement_date = exercise_date.clone() + settlement_period;
            events.push(EventFactory::create_event_with_convention(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let maturity = Self::maturity(model);
        let mut events = Self::add_external_xd_event(model, events, observer, &maturity).unwrap();

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
        if let Some(maturity_date) = model.maturityDate.clone().map(|rc| (*rc).clone()) {
            maturity_date.clone()
        } else {
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
        } else if model.notionalPrincipal.is_some() {
            let role_sign = &model.contractRole.clone().unwrap().role_sign();
            states.notionalPrincipal = Some(
                model.coverageOfCreditEnhancement.unwrap()
                    * role_sign
                    * model.notionalPrincipal.unwrap()
            );
        } else {
            states.notionalPrincipal = Some(Self::calculate_notional_principal(
                &states,
                model,
                observer,
                &states.statusDate.unwrap(),
            ));
        }

        if model.feeRate.is_none() {
            states.feeAccrued = Some(0.0);
        } else if model.feeAccrued.is_some() {
            states.feeAccrued = model.feeAccrued;
        }

        states.exerciseAmount = model.exerciseAmount;
        states.exerciseDate = model.exerciseDate.clone();

        Ok(states)
    }

    pub fn calculate_notional_principal(
        states: &StateSpace,
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
        let coverage = model.coverageOfCreditEnhancement.unwrap();

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
                    .map(|c| c.get_contract_attribute("marketObjectCode").unwrap())
                    .collect();

                coverage
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
        mut events: Vec<ContractEvent>,
        observer: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let contract_identifiers: Vec<String> = model.contractStructure.clone().unwrap()
            .iter()
            .map(|c| c.get_contract_attribute("contractID").unwrap().to_string())
            .collect();

        let credit_event_type_covered = model.creditEventTypeCovered.clone().unwrap()[0].clone();

        let observed_events = observer.events(model);

        let ce_events: Vec<ContractEvent> = observed_events
            .into_iter()
            .filter(|e| {
                contract_identifiers.contains(&e.contractID.clone().unwrap())
                    && &e.eventTime.clone().unwrap() <= maturity
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
                Some(Rc::new(STF_XD_CEG)),
                model.contractID.as_ref(),
            ));

            let settlement_period = model.settlementPeriod.clone().unwrap();
            let settlement_date = ce_event.eventTime.clone().unwrap() + settlement_period;

            events.push(EventFactory::create_event_with_convention(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                model.businessDayAdjuster.as_ref().unwrap(),
                model.contractID.as_ref(),
            ));
        }

        Ok(events)
    }
}