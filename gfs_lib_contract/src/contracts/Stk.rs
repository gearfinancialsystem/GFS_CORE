use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::Payoff;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_calendar::Calendar::Calendar;
use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_dividend::CycleAnchorDateOfDividendPayment::CycleAnchorDateOfDividendPayment;
use gfs_lib_terms::terms::grp_dividend::CycleOfDividendPayment::CycleOfDividendPayment;
use gfs_lib_terms::terms::grp_dividend::MarketObjectCodeOfDividends::MarketObjectCodeOfDividends;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::MarketValueObserved::MarketValueObserved;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::Quantity::Quantity;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::{IsoCycleConvertTo, IsoCycleConvertToOption, IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::IsoPeriod::IsoPeriod;
use gfs_lib_types::types::Value::Value;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::contracts::Pam::PAM;
use crate::events::EventSequence::EventSequence;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub struct STK {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Box<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Box<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
}

impl TraitContractModel for STK {

    fn new() -> Self {
        Self {
            contract_id: ContractID::new("init".to_string()).expect("init contract ID"),
            contract_terms: ContractTerms::default(),
            risk_factor_external_data: None,
            risk_factor_external_event: None,
            related_contracts: None,
            event_timeline: Vec::new(),
            states_space: StatesSpace::default(),
            status_date: None,
        }
    }

    fn init_contract_terms(&mut self, sm: &HashMap<String, Value>) {
        //let purchase_date = IsoDatetime::provide(sm, "purchaseDate");
        let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");

        let calendar = Calendar::provide_rc(sm, "calendar");

        //VERIFIER PAS PRESENT DANS LA LISTE DES TERMES
        let cycle_of_dividend_payment = CycleOfDividendPayment::provide_from_input_dict(sm, "cycleOfDividendPayment");

        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "BusinessDayAdjuster",
                calendar_clone.expect("df")
            )
        };

        let cycle_anchor_date_of_dividend_payment = {
            let a = if cycle_of_dividend_payment.is_none() {
                None
            } else {
                let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
                CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).ok()
            };
            let b = CycleAnchorDateOfDividendPayment::provide_from_input_dict(sm, "CycleAnchorDateOfDividendPayment");
            if b.is_none() { a } else { b }
        };

        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            quantity: Quantity::provide_from_input_dict(sm, "quantity"),
            purchase_date: purchase_date,
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            cycle_of_dividend_payment: cycle_of_dividend_payment,
            cycle_anchor_date_of_dividend_payment: cycle_anchor_date_of_dividend_payment,
            market_object_code_of_dividends: MarketObjectCodeOfDividends::provide_from_input_dict(sm, "marketObjectCodeOfDividends"),
            ..Default::default()
        };

        self.contract_terms = ct;
    }

    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Box<dyn TraitExternalData>>) {
        self.risk_factor_external_data = risk_factor_external_data;
    }

    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Box<dyn TraitExternalEvent>>) {
        self.risk_factor_external_event = risk_factor_external_event;
    }

    fn init_related_contracts(&mut self, sm: &HashMap<String, Value>) {
        self.related_contracts = None;
    }

    fn init_status_date(&mut self) {
        self.status_date = self.contract_terms.status_date;
    }

    /// Initialize the StatesSpace according to the model attributes
    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
        states.status_date = model.status_date.clone();

        self.states_space = states;
    }

    fn init_contract_event_timeline(&mut self, to: Option<PhantomIsoDatetimeW>) {
        let model = &self.contract_terms;
        let mut events = Vec::new();

        if model.purchase_date.is_some(){
            let e: ContractEvent = EventFactory::create_event(
                &model.purchase_date.convert_option::<ScheduleTime>(),
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_STK")),
                Some(StatesTransitionFunction::from_str("STF_PRD_STK")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }
        if model.cycle_of_dividend_payment.is_some(){
            if model.termination_date.is_none(){
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment.convert_option::<StartTime>(),
                            &Some(model.cycle_anchor_date_of_dividend_payment.clone().unwrap().value() + IsoPeriod::of_years(10)).convert_option::<EndTime>(), // definir les constantes
                            &Some(model.cycle_of_dividend_payment.unwrap().convert::<PhantomIsoCycleW>()),
                            &model.end_of_month_convention,
                            Some(true)
                        ).iter().map(|e| e.convert::<ScheduleTime>()).collect(),
                        &EventType::DV,
                        &model.currency,
                        Some(PayOffFunction::from_str("POF_DV_STK")),
                        Some(StatesTransitionFunction::from_str("STF_DV_STK")),
                        &model.business_day_adjuster,
                        &model.contract_id)
                );
            }
            else {
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment.convert_option::<StartTime>(),
                            &model.termination_date.convert_option::<EndTime>(),
                            &model.cycle_of_dividend_payment.convert_option::<PhantomIsoCycleW>(),
                            &model.end_of_month_convention,
                            Some(true)).iter().map(|e| e.convert::<ScheduleTime>()).collect(),
                        &EventType::DV,
                        &model.currency,
                        Some(PayOffFunction::from_str("POF_DV_STK")),
                        Some(StatesTransitionFunction::from_str("STF_DV_STK")),
                        &model.business_day_adjuster,
                        &model.contract_id)
                )
            }
        }
        if model.termination_date.is_some(){
            let termination: ContractEvent = EventFactory::create_event(
                &model.termination_date.convert_option::<ScheduleTime>(),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_STK")),
                Some(StatesTransitionFunction::from_str("STF_TD_STK")),
                &None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.compare_to(&termination) != 1
            });
            events.push(termination);
        }
        let tmpe = EventFactory::create_event(
            &Some(model.status_date.clone().unwrap().value().convert::<ScheduleTime>()),
            &EventType::TD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id
        );
        events.retain(|e| {
            e.compare_to({ &tmpe }) != -1
        });
        let tmpe = EventFactory::create_event(
            &Some(to.clone().clone().unwrap().convert::<ScheduleTime>()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id
        );
        events.retain(|e| {
            e.compare_to({ &tmpe }) != 1
        });

        events.sort();
        self.event_timeline = events.clone();
    }

    fn set_status_date(&mut self, status_date: Option<StatusDate>) {
        self.status_date = status_date;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            //println!("{:?}\n", a);
            self.event_timeline[id_ce].payoff = Some(Payoff::new(a).expect("ok"));
            //println!("payoff0{:?}\n", self.event_timeline[id_ce].payoff);
        }

        // on peut la retravailler pour etre plus direct et efficace
    }

    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &mut self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

    fn compute_payoff(&mut self) {
        let id_ce: usize = 0;
        self.eval_pof_contract_event(id_ce);
    }

    fn next(&mut self) {
        let id_ce: usize = 0;
        self.eval_pof_contract_event(id_ce);
    }

    fn add_event_to_contract_event_timeline(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        // reflechir a quoi pourrait bien servir reset
        self.contract_terms = ContractTerms::default();
        self.risk_factor_external_data = None;
        self.risk_factor_external_event = None;
        self.related_contracts = None;
        self.event_timeline = Vec::new();
        self.states_space = StatesSpace::default();
        self.status_date = None;
    }

    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {
        // Sort events according to their time sequence
        self.sort_events_timeline();
        // Initialize state space per status date
        let maturity = &self.contract_terms.maturity_date.clone();
        self.init_state_space(maturity);
        let events = &mut self.event_timeline.clone();

        let mut result_vec: Vec<TestResult> = Vec::new();
        let mut i: usize = 0;
        for event in events.iter_mut() {

            if date.is_some() {
                if event.event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                    break
                }
            }

            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            if extract_results == true {
                let curr_testresult = TestResult {
                    eventDate: event.event_time.expect("fe").to_string(),
                    eventType: event.event_type.to_string(),
                    payoff: self.event_timeline[i].payoff.clone().expect("ok").to_string(),
                    currency: event.currency.clone().expect("ef").0,
                    notionalPrincipal: self.states_space.notional_principal.clone().expect("ok").to_string(),
                    nominalInterestRate: self.states_space.nominal_interest_rate.clone().expect("ok").to_string(),
                    accruedInterest: self.states_space.accrued_interest.clone().expect("ok").to_string(),
                };
                result_vec.push(curr_testresult)
            }

            i+=1;
        }
        // Return evaluated events
        self.event_timeline = events.clone();

        // recup des resultats
        if extract_results == false {

            return None;
        }
        else {
            ////////////////////////////////////////////////////////
            // Remove pre-purchase events if purchase date is set //
            ////////////////////////////////////////////////////////
            result_vec.retain(|e| {
                if self.contract_terms.purchase_date.is_some() {
                    let purchase_event: ContractEvent = EventFactory::create_event(
                        &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
                        &EventType::PRD,
                        &self.contract_terms.currency,
                        None,
                        None,
                        &None,
                        &self.contract_terms.contract_id,
                    );
                    let epoch_millis = IsoDatetime::from_str(e.eventDate.as_str()).clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
                    let epoch_offset = epoch_millis + EventSequence::time_offset(&EventType::from_str(e.eventType.as_str()).expect("exist"));
                    EventType::from_str(e.eventType.as_str()).expect("exist") == EventType::AD || epoch_offset as f64 >= purchase_event.epoch_offset.unwrap().value()
                } else { true }
            });
            return Some(Ok(result_vec));
        }
    }

    fn sort_events_timeline(&mut self) {
        self.event_timeline.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));
    }

}
impl fmt::Display for STK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STK")
    }
}


impl fmt::Debug for STK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("STK")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for STK {
    fn clone(&self) -> Self {
        STK {
            contract_id: self.contract_id.clone(),
            contract_terms: self.contract_terms.clone(),
            risk_factor_external_data: None, // faire qqchose specifique ici ?
            risk_factor_external_event: None, // faire qqchose specifique ici ?
            related_contracts: None, // faire qqchose specifique ici ?
            event_timeline: self.event_timeline.clone(),
            states_space: self.states_space.clone(),
            status_date: self.status_date.clone(),
        }
    }
}
