
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_events::events::EventFactory::EventFactory;
use lib_actus_events::events::EventType::EventType;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_types::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use lib_actus_terms::terms::grp_contract_identification::ContractType::ContractType;
use lib_actus_terms::terms::grp_contract_identification::CreatorID::CreatorID;
use lib_actus_terms::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use lib_actus_terms::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use lib_actus_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use lib_actus_terms::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::cec::pof::POF_STD_CEC::POF_STD_CEC;
use crate::functions::cec::stf::STF_STD_CEC::STF_STD_CEC;
use crate::functions::cec::stf::STF_XD_CEC::STF_XD_CEC;
use crate::functions::ceg::pof::POF_MD_CEG::POF_MD_CEG;
use crate::functions::ceg::stf::STF_MD_CEG::STF_MD_CEG;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use lib_actus_terms::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use lib_actus_terms::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use lib_actus_terms::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_settlement::ExerciseDate::ExerciseDate;
use lib_actus_types::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[derive(Debug, Clone, PartialEq)]
pub struct CEC {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}


impl TraitContractModel for CEC {
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
        let calendar = Calendar::provide_rc(sm, "calendar");

        let contract_role = ContractRole::provide(sm, "contractRole");
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };

        let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
        let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
            Some(CreditEventTypeCovered::default())
        } else {
            credit_event_type_covered_tmp
        };

        // let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
        // let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
        //     let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
        //     CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        // } else {
        //     CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
        // };

        // let day_count_convention = if let Some(maturity_date) = &maturity_date {
        //     DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        // } else {
        //     None
        // };

        let calendar_clone = Some(Rc::clone(&calendar));
        let a = BusinessDayAdjuster::provide(
            sm,
            "businessDayAdjuster",
            calendar_clone.unwrap()
        );
        let mut business_day_adjuster = if a.is_some() {
            a
        } else {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::new("NOS", calendar_clone.unwrap()).ok()
        };


        //this.bdConvention = new Same();
        //this.scConvention = new ShiftCalc();

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

        let mut guaranteed_exposure = GuaranteedExposure::provide_from_input_dict(sm, "guaranteedExposure");
        guaranteed_exposure = if guaranteed_exposure.is_none() {
            GuaranteedExposure::new(Some("NO")).ok()
        } else {
            guaranteed_exposure
        };

        let mut coverage_of_credit_enhancement = CoverageOfCreditEnhancement::provide_from_input_dict(sm, "coverageOfCreditEnhancement");
        coverage_of_credit_enhancement = if coverage_of_credit_enhancement.is_none() {
            CoverageOfCreditEnhancement::new(1.0).ok()
        } else {
            coverage_of_credit_enhancement
        };

        let mut settlement_period =  SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod");
        settlement_period = if settlement_period.is_none() {
            let a = SettlementPeriod::parse_from_string("P0D").unwrap();
            Some(SettlementPeriod::new(a.years, a.months, a.days))
        } else {
            settlement_period
        };

        let mut exercise_amount = ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount");
        exercise_amount = if exercise_amount.is_none() {
            ExerciseAmount::new(0.0).ok()
        } else {
            exercise_amount
        };

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

            //contract_performance: ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
            //non_performing_date: NonPerformingDate::provide_from_input_dict(sm, "nonPerformingDate"),
            //grace_period: GracePeriod::provide_from_input_dict(sm, "gracePeriod"),
            //delinquency_period: DelinquencyPeriod::provide_from_input_dict(sm, "delinquencyPeriod"),
            //delinquency_rate: DelinquencyRate::provide_from_input_dict(sm, "delinquencyRate"),
            guaranteed_exposure: guaranteed_exposure,
            coverage_of_credit_enhancement: coverage_of_credit_enhancement,
            credit_event_type_covered: credit_event_type_covered,
            //cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
            //cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
            //fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
            //fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
            //fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            //cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            //cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            //nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            //day_count_convention: day_count_convention,
            currency: Currency::provide_from_input_dict(sm, "currency"),
            //maturity_date: maturity_date,
            //notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            //purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            //price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            //termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            //price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
            exercise_amount: exercise_amount,
            settlement_period: settlement_period,
            //cycle_of_dividend: CycleOfDividend::provide_from_input_dict(sm, "cycleOfDividend"),
            //cycle_anchor_date_of_dividend: CycleAnchorDateOfDividend::provide_from_input_dict(sm, "cycleAnchorDateOfDividend"),
            //next_dividend_payment_amount: NextDividendPaymentAmount::provide_from_input_dict(sm, "nextDividendPaymentAmount"),
            //ex_dividend_date: ExDividendDate::provide_from_input_dict(sm, "exDividendDate"),

            //market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
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
        let model = &self.contract_terms;
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

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {
        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let maturity = Self::maturity(&self.contract_terms);
        let mut events = Self::add_external_xd_event(&self.contract_terms, events, observer, &maturity.value()).unwrap();

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

        self.states_space = states;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &self.states_space,
                &self.contract_terms,
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                },
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
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                }
                ,
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

impl CEC {

    fn maturity(&self) -> MaturityDate {

        let covered_contract_refs = &self.contract_structure.clone().unwrap().0
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

    pub fn calculate_notional_principal(&self, time: &IsoDatetime) -> f64 {

        let covered_contract_refs = &self.contract_structure.clone().unwrap().0
            .iter()
            .filter(|e| e.reference_role == ReferenceRole::COVE)
            .map(|cr| cr.clone())
            .collect::<Vec<_>>();

        let states_at_time_point: Vec<StatesSpace> = covered_contract_refs
            .iter()
            .map(|c| c.get_state_space_at_time_point(time.clone(), &self.contract_terms))
            .collect();

        let role_sign = &self.contract_terms.contract_role.clone().unwrap().role_sign();
        let coverage = &self.contract_terms.coverage_of_credit_enhancement.clone().unwrap();

        match &self.contract_terms.guaranteed_exposure {
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
                        &self.contract_risk_factors.state_at(
                            code.clone(), 
                            time, 
                            &StatesSpace::default(),
                            &self.contract_terms, 
                            true))
                    .sum::<f64>()
            }
        }
    }

    pub fn calculate_market_value_covering_contracts(&self,
        time: &IsoDatetime,
    ) -> f64 {
        let covering_contract_refs = &self.contract_structure.clone().unwrap().0
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
                &self.contract_risk_factors.state_at(
                    code.clone(), 
                    time, 
                    &StatesSpace::default(),
                    &self.contract_terms, 
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