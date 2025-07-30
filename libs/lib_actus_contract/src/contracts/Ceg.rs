use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::CreatorID::CreatorID;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use crate::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use crate::terms::grp_counterparty::GracePeriod::GracePeriod;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use crate::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use crate::terms::grp_dividend::ExDividendDate::ExDividendDate;
use crate::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
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
use crate::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::Value::Value;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;

use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[derive(Debug, Clone, PartialEq)]
pub struct CEG {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for CEG {
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
        // Déclarations simples sans dépendances
        let calendar = Calendar::provide_rc(sm, "calendar");
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };
        let contract_role = ContractRole::provide(sm, "contractRole");


        let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
        let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
            Some(CreditEventTypeCovered::default())
        } else {
            credit_event_type_covered_tmp
        };

        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
        };

        let day_count_convention = if let (Some(maturity_date)) = (&maturity_date) {
            DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };

        let business_day_adjuster =  {
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
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: contract_role,
            creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
            non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
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
            fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
            fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
            fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            currency: Currency::provide_from_input_dict(sm, "currency"),
            exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
            exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
            settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
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

            let settlement_period = &self.contract_terms.settlement_period.clone().unwrap();
            let settlement_date = exercise_date.clone() + settlement_period.clone().value().clone();
            let e: ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &self.contract_terms.currency,
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let model = &self.contract_terms;

        let maturity = Self::maturity(model);
        let mut events = Self::add_external_xd_event(model, events, observer, &maturity).unwrap();

        self.init_state_space(maturity);


        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
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

impl CEG {
    pub fn calculate_notional_principal(
        contract_terms: &ContractTerms,
        contract_structure : &Vec<ContractReference>,
        risk_factor_model: &RiskFactorModel,
        time: &IsoDatetime
    ) -> f64 {

        let covered_contract_refs = self.contract_structure.clone().unwrap().0
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();

        let states_at_time_point: Vec<StatesSpace> = covered_contract_refs
            .iter()
            .map(|c| c.get_state_space_at_time_point(time.clone(), observer))
            .collect();

        let role_sign = self.contract_terms.contract_role.clone().unwrap().role_sign();
        let coverage = self.contract_terms.coverage_of_credit_enhancement.clone().unwrap();

        match &self.contract_terms.guaranteed_exposure {
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
                    .map(|code|
                        self.contract_risk_factors.state_at(
                            code.clone(),
                            time,
                            states,
                            model,
                            true))
                    .sum::<f64>()
            }
        }
    }

    fn add_external_xd_event(
        contract_terms: &ContractTerms,
        contract_structure: &Vec<ContractReference>,
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        risk_factor_model: &RiskFactorModel,
        maturity: &IsoDatetime,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let contract_identifiers: Vec<String> = self.contract_structure.clone().unwrap().0
            .iter()
            .map(|c|     {
                let a = c.object.as_cm().unwrap().contract_id.clone().unwrap().value();
                a
            })
            .collect();

        let credit_event_type_covered = self.contract_terms.credit_event_type_covered.clone().unwrap().values()[0].clone();

        let observed_events = self.contract_risk_factors.events(&self.contract_terms);

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
                &self.contract_terms.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_CEG)),
                &None,
                &self.contract_terms.contract_id,
            );
            events.push(e.to_owned());

            let settlement_period = model.settlement_period.clone().unwrap();
            let settlement_date = ce_event.event_time.clone().unwrap() + settlement_period.value().clone();

            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &self.contract_terms.currency,
                Some(Rc::new(POF_STD_CEG)),
                Some(Rc::new(STF_STD_CEG)),
                &self.contract_terms.business_day_adjuster,
                &self.contract_terms.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        Ok(events)
    }

    fn maturity(&self) -> MaturityDate {
        if let Some(maturity_date) = &self.contract_terms.maturity_date.clone().map(|rc| (*rc).clone()) {
            maturity_date.clone()
        } else {
            let covered_contract_refs = &self.contract_structure.clone().unwrap()
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