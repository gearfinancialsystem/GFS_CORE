use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
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
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::traits::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
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
use crate::terms::grp_interest::ArrayCycleAnchorDateOfInterestPayment::ArrayCycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::ArrayCycleOfInterestPayment::ArrayCycleOfInterestPayment;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::optns::pof::POF_MD_OPTNS::POF_MD_OPTNS;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::optns::pof::POF_STD_OPTNS::POF_STD_OPTNS;
use crate::functions::optns::pof::POF_TD_OPTNS::POF_TD_OPTNS;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use crate::functions::optns::stf::STF_MD_OPTNS::STF_MD_OPTNS;
use crate::functions::optns::stf::STF_STD_OPTNS::STF_STD_OPTNS;
use crate::functions::optns::stf::STF_XD_OPTNS::STF_XD_OPTNS;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_optionality::OptionStrike1;
use crate::terms::grp_optionality::OptionStrike2::OptionStrike2;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct OPTNS {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for OPTNS {
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
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };
        let calendar = Calendar::provide_rc(sm, "calendar");
        let contract_role = ContractRole::provide(sm, "contractRole");


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
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
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
            cycle_anchor_date_of_interest_payment: CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
            array_cycle_anchor_date_of_interest_payment: ArrayCycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "arrayCycleAnchorDateOfInterestPayment"),
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            array_cycle_of_interest_payment: ArrayCycleOfInterestPayment::provide_from_input_dict(sm, "arrayCycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            exercise_amount: ExerciseAmount::provide_from_input_dict(sm, "exerciseAmount"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
            exercise_date: ExerciseDate::provide_from_input_dict(sm, "exerciseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            option_type: OptionType::provide_from_input_dict(sm, "optionType"),
            option_strike1: OptionStrike1::provide_from_input_dict(sm, "optionStrike1"),
            option_strike2: OptionStrike2::provide_from_input_dict(sm, "optionStrike2"),
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
        let mut events = Vec::new();
        let model = &self.contract_terms;
        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Exercise and Settlement events
        if let Some(exercise_date) = &model.exercise_date {
            let e : ContractEvent<ExerciseDate, ExerciseDate>= EventFactory::create_event(
                &Some(exercise_date.clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(exercise_date.clone().value() + model.clone().settlement_period.unwrap().value().clone())
            );
            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        } else {
            let e:ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value() + model.settlement_period.clone().unwrap().value().clone())
            );

            let e : ContractEvent<IsoDatetime, IsoDatetime >= EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Maturity event
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &model.maturity_date.clone().map(|rc| (*rc).clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_OPTNS)),
            Some(Rc::new(STF_MD_OPTNS)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_OPTNS)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        self.contract_events = events.clone();
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

        // Add external XD-event
        events.extend(self.contract_risk_factors.events(model));

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.status_date = model.status_date.clone();
        states.exercise_amount = model.exercise_amount.clone();
        states.exercise_date = model.exercise_date.clone();
        states.contract_performance = model.contract_performance.clone();

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
impl fmt::Display for OPTNS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OPTNS")
    }
}