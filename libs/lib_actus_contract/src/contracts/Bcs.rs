use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::bcs::pof::POF_PRD_BCS::POF_PRD_BCS;
use crate::functions::bcs::pof::POF_TD_BCS::POF_TD_BCS;
use crate::functions::bcs::stf::STF_ME_BCS::STF_ME_BCS;
use crate::functions::bcs::stf::STF_TD_BCS::STF_TD_BCS;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::pam::pof::POF_AD_PAM::POF_AD_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use crate::terms::grp_boundary::BoundaryMonitoringAnchorDate::BoundaryMonitoringAnchorDate;
use crate::terms::grp_boundary::BoundaryMonitoringCycle::BoundaryMonitoringCycle;
use crate::terms::grp_boundary::BoundaryMonitoringEndDate::BoundaryMonitoringEndDate;
use crate::terms::grp_boundary::BoundaryValue::BoundaryValue;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::CreatorID::CreatorID;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use crate::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use crate::terms::grp_counterparty::GracePeriod::GracePeriod;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_counterparty::PrepaymentPeriod::PrepaymentPeriod;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use crate::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use crate::terms::grp_dividend::ExDividendDate::ExDividendDate;
use crate::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::Value::Value;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct BCS {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for BCS {
    fn new() -> Self {
        Self {
            contract_terms: ContractTerms::default(),
            contract_events: Vec::<ContractEvent<IsoDatetime, IsoDatetime>>::new(),
            contract_risk_factors: None,
            contract_structure: None,
            states_space: StatesSpace::default(),
            result_vec_toggle: false,
            result_vec: None,
        }
    }

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>) {
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp.clone() {
            Some(Rc::new(a))
        } else {
            None
        };
        let calendar = Calendar::provide_rc(sm, "calendar");
        let contract_role = ContractRole::provide(sm, "contractRole");
        let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");


        let w = BoundaryMonitoringAnchorDate::provide_from_input_dict(sm, "boundaryMonitoringAnchorDate");
        let boundary_monitoring_anchor_date = if let Some(boundary_monitoring_anchor_date) = w {
            Some(boundary_monitoring_anchor_date)
        } else {
            let aa = purchase_date.clone().unwrap().value().to_string();
            BoundaryMonitoringAnchorDate::from_str(&aa).ok()
        };

        let a = BoundaryMonitoringEndDate::provide_from_input_dict(sm, "BoundaryMonitoringEndDate");
        let boundary_monitoring_end_date = if let Some(boundary_monitoring_end_date) = a {
            Some(boundary_monitoring_end_date)
        } else {

            Some(BoundaryMonitoringEndDate::from_str(maturity_date_tmp.unwrap().value().to_string().as_str()).unwrap())
        };


        let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
        let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
            Some(CreditEventTypeCovered::default())
        } else {
            credit_event_type_covered_tmp
        };


        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };

        // let contract_structure = if let Some(contract_structure) = sm.get("contractStructure") {
        //     if let Some(structure_vec) = contract_structure.as_vec() {
        //         let contract_structure: Vec<ContractReference> = structure_vec.iter()
        //             .map(|d| ContractReference::new(d.as_hashmap().unwrap(), &contract_role.clone().unwrap()))
        //             .collect();
        //         Some(ContractStructure::new(contract_structure))
        //     } else {
        //         None
        //     }
        //
        // } else {None};
        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};
        let ct = ContractTerms {
            maturity_date: maturity_date,
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: contract_role,
            creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
            seniority: Seniority::provide_from_input_dict(sm, "seniority"),
            non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
            prepayment_period: PrepaymentPeriod::provide_from_input_dict(sm, "prepaymentPeriod"),
            grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
            delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
            delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
            guaranteed_exposure: GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure"),
            coverage_of_credit_enhancement: CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement"),
            credit_event_type_covered: credit_event_type_covered,
            cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
            cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
            next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
            ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),
            cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
            cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
            exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
            exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            boundary_value: BoundaryValue::provide_from_input_dict(sm, "boundaryValue"),
            boundary_direction: BoundaryDirection::provide_from_input_dict(sm, "boundaryDirection"),
            boundary_effect: BoundaryEffect::provide_from_input_dict(sm, "boundaryEffect"),
            boundary_leg_initially_active: BoundaryLegInitiallyActive::provide_from_input_dict(sm, "boundaryLegInitiallyActive"),
            boundary_monitoring_anchor_date: boundary_monitoring_anchor_date,
            boundary_monitoring_end_date: boundary_monitoring_end_date,
            boundary_monitoring_cycle: BoundaryMonitoringCycle::provide_from_input_dict(sm, "boundaryMonitoringCycle"),
            boundary_crossed_flag: BoundaryCrossedFlag::provide_from_input_dict(sm, "boundaryCrossedFlag"),
            ..Default::default()
        };


        self.contract_terms = ct;
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = None;
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>) {
        self.contract_structure = None;
    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new());
    }

    fn schedule(&mut self, to: Option<IsoDatetime>) {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let model = &self.contract_terms;
        // Purchase date event of master contract
        if model.purchase_date.is_some() {
            let e : ContractEvent<PurchaseDate, PurchaseDate>= EventFactory::create_event(
                &model.purchase_date.clone(),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Raw monitoring events
        let monitoring_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.boundary_monitoring_anchor_date,
                &model.boundary_monitoring_end_date,
                &model.boundary_monitoring_cycle,
                &model.end_of_month_convention,
                Some(true),
            ),
            &EventType::ME,
            &model.currency,
            Some(Rc::new(POF_AD_PAM)),
            Some(Rc::new(STF_ME_BCS)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(monitoring_events);

        self.contract_events = events;
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let model = &self.contract_terms;
        let _maturity = &model.maturity_date.clone();
        self.init_state_space(_maturity);
        let events = &mut self.contract_events.clone();
        // Sort the events according to their time sequence
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        // Apply events according to their time sequence to current state
        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        // Remove monitoring events
        events.retain(|e| e.event_type != EventType::ME);

        // Activating child legs based on boundaryEffect
        if self.states_space.boundary_crossed_flag.clone().unwrap().value() == true {
            match model.boundary_effect.as_ref().unwrap() {
                BoundaryEffect::INFIL(INFIL) => {
                    self.states_space.boundary_leg1_active_flag = Some(true);
                    self.states_space.boundary_leg2_active_flag = Some(false);
                }
                BoundaryEffect::INSEL(INSEL) => {
                    self.states_space.boundary_leg2_active_flag = Some(true);
                    self.states_space.boundary_leg1_active_flag = Some(false);
                }
                BoundaryEffect::OUT(OUT) => {
                    self.states_space.boundary_leg1_active_flag = Some(false);
                    self.states_space.boundary_leg2_active_flag = Some(false);
                }
                _ => {}
            }
        }

        // First leg model
        let first_leg_model = &self.contract_structure.clone().unwrap().0.iter()
            .find(|c| c.reference_role == ReferenceRole::FIL)
            .and_then(|c| c.object.clone().as_cm())
            .unwrap();

        let mut first_leg_schedule = Vec::new();

        // Second leg model
        let second_leg = &self.contract_structure.clone().unwrap().0.iter()
            .find(|c| c.reference_role == ReferenceRole::SEL)
            .and_then(|c| c.object.clone().as_cm());

        let mut second_leg_schedule = Vec::new();
        let second_leg_model = second_leg.unwrap();

        // Create children event schedule based on boundary conditions
        if self.states_space.boundary_leg1_active_flag.unwrap() == true {
            let m = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
            first_leg_schedule = ContractType::schedule(
                Some(m),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contract_type.clone() != "PAM" {
                let e: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                    &self.states_space.status_date,
                    &EventType::PRD,
                    &first_leg_model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    &None,
                    &first_leg_model.contract_id,
                );
                first_leg_schedule.push(e.to_iso_datetime_event());
            } else {
                first_leg_schedule.retain(|e| e.event_type != EventType::IED);
                let e: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                    &self.states_space.status_date,
                    &EventType::IED,
                    &first_leg_model.currency,
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    &None,
                    &first_leg_model.contract_id,
                );

                first_leg_schedule.push(e.to_iso_datetime_event());
            }

            first_leg_schedule.retain(|e| e.event_time.unwrap().value() >= states.status_date.clone().unwrap().value());

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer).unwrap();
            events.extend(first_leg_events);
        } else if self.states_space.boundary_leg1_active_flag.clone().unwrap() == false
            && model.boundary_leg_initially_active.is_some()
            && model.boundary_leg_initially_active.clone().unwrap().to_stringx().unwrap() == ReferenceRole::FIL.to_stringx().unwrap()
        {
            let m = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
            first_leg_schedule = ContractType::schedule(
                Some(m),
                &first_leg_model,
            ).unwrap();

            if first_leg_model.contract_type.clone() != "PAM" {
                let e : ContractEvent<PurchaseDate, PurchaseDate>= EventFactory::create_event(
                    &model.purchase_date,
                    &EventType::PRD,
                    &first_leg_model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    &None,
                    &first_leg_model.contract_id,
                );
                first_leg_schedule.push(e.to_iso_datetime_event());
            }

            let td_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &self.states_space.status_date,
                &EventType::TD,
                &first_leg_model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &None,
                &first_leg_model.contract_id,
            );

            first_leg_schedule.retain(|e| e.compare_to(&td_event.to_iso_datetime_event()) != 1);
            first_leg_schedule.push(td_event.to_iso_datetime_event());

            // Apply schedule of children
            let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model, observer);
            events.extend(first_leg_events.unwrap());
        }

        if self.states_space.boundary_leg2_active_flag.clone().unwrap() == true {
            let m = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
            second_leg_schedule = ContractType::schedule(
                Some(m),
                &second_leg_model,
            ).unwrap();

            if second_leg_model.contract_type.clone() != "PAM"{
                let e: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                    &self.states_space.status_date,
                    &EventType::PRD,
                    &second_leg_model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    &None,
                    &second_leg_model.contract_id,
                );
                second_leg_schedule.push(e.to_iso_datetime_event());
            } else {
                second_leg_schedule.retain(|e| e.event_type != EventType::IED);
                let e: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                    &self.states_space.status_date,
                    &EventType::IED,
                    &second_leg_model.currency,
                    Some(Rc::new(POF_IED_PAM)),
                    Some(Rc::new(STF_IED_PAM)),
                    &None,
                    &second_leg_model.contract_id,
                );
                second_leg_schedule.push(e.to_iso_datetime_event());
            }

            second_leg_schedule.retain(|e| e.event_time >= Some(states.status_date.clone().unwrap().value()));

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        } else if self.states_space.boundary_leg2_active_flag.clone().unwrap() == false
            && model.boundary_leg_initially_active.is_some()
            && model.boundary_leg_initially_active.as_ref().unwrap().to_stringx().unwrap() == ReferenceRole::SEL.to_stringx().unwrap()
        {
            if second_leg_model.contract_type.clone() != "PAM" {
                let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                    &model.purchase_date,
                    &EventType::PRD,
                    &second_leg_model.currency,
                    Some(Rc::new(POF_PRD_BCS)),
                    Some(Rc::new(STF_PRD_STK)),
                    &None,
                    &second_leg_model.contract_id,
                );
                second_leg_schedule.push(e.to_iso_datetime_event());
            }

            let td_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &self.states_space.status_date.clone(),
                &EventType::TD,
                &second_leg_model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &None,
                &second_leg_model.contract_id,
            );

            second_leg_schedule.retain(|e| e.compare_to(&td_event.to_iso_datetime_event()) != 1);
            second_leg_schedule.push(td_event.to_iso_datetime_event());
            let m = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
            second_leg_schedule = ContractType::schedule(
                Some(m),
                &second_leg_model,
            ).unwrap();

            // Apply schedule of children
            let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model, observer);
            events.extend(second_leg_events.unwrap());
        }

        // Termination of master contract
        if self.states_space.boundary_crossed_flag.clone().unwrap().value() == true && model.boundary_effect.clone().unwrap() != BoundaryEffect::INFIL(INFIL) {
            let e: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &self.states_space.status_date.clone(),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        } else {
            let e: ContractEvent<BoundaryMonitoringEndDate, BoundaryMonitoringEndDate> = EventFactory::create_event(
                &model.boundary_monitoring_end_date.clone(),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_BCS)),
                Some(Rc::new(STF_TD_BCS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Sort the events according to their time sequence
        events.sort();

        // Return post events states
        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        // Initialize state variables
        states.status_date = model.status_date.clone();
        states.contract_performance = model.contract_performance;
        states.boundary_crossed_flag = BoundaryCrossedFlag::new(false).ok();
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

        self.states_space = states;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &self.states_space,
                &self.contract_terms,
                &self.contract_structure,
                &self.contract_risk_factors,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            println!("{:?}", a);


            self.contract_events[id_ce].payoff = Some(a);
            // let curr_ce_clone = &curr_ce.clone();
            if self.result_vec_toggle == true {
                if let Some(rv) = &mut self.result_vec {
                    let mut a = ResultSet::new();
                    a.set_result_set(&self.states_space, &self.contract_events[id_ce]);

                    rv.push(a)
                }
            }
        }

        // on peut la retravailler pour etre plus direct et efficace
    }

    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let mut curr_ce= self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &mut self.states_space,
                &self.contract_terms,
                &self.contract_structure,
                &self.contract_risk_factors,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

}
impl fmt::Display for BCS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BCS")
    }
}
