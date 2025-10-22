use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::Payoff;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
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
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub curr_event_index: i32,
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
    pub first_event_date: Option<PhantomIsoDatetimeW>,
    pub last_event_date: Option<PhantomIsoDatetimeW>,
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
            curr_event_index: -1,
            states_space: StatesSpace::default(),
            status_date: None,
            first_event_date: None,
            last_event_date: None,
        }
    }

    fn init_contract_terms(&mut self, sm: HashMap<String, Value>) {
        //let purchase_date = IsoDatetime::provide(sm, "purchaseDate");
        let mut quantity = Quantity::provide_from_input_dict(&sm, "quantity");
        if quantity.is_none() {
            quantity = Some(Quantity::new(1.0).expect("ok"));
        }
        // purchase date
        let purchase_date = PurchaseDate::provide_from_input_dict(&sm, "purchaseDate");

        // calendar
        let calendar = Calendar::provide_rc(&sm, "calendar");

        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                &sm,
                "businessDayConvention",
                calendar_clone.expect("df")
            )
        };

        // price at purchase date
        let mut price_at_purchase_date = PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate");
        if price_at_purchase_date.is_none() {
            price_at_purchase_date = Some(PriceAtPurchaseDate::new(0.0).expect("ok"));
        }

        // price at termination date
        let mut price_at_termination_date = PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate");
        if price_at_termination_date.is_none() {
            price_at_termination_date = Some(PriceAtTerminationDate::new(0.0).expect("ok"));
        }

        // market value observec
        let mut market_value_observed = MarketValueObserved::provide_from_input_dict(&sm, "marketValueObserved");
        if market_value_observed.is_none() {
            market_value_observed = Some(MarketValueObserved::new(0.0).expect("ok"));
        }

        let cycle_of_dividend_payment = CycleOfDividendPayment::provide_from_input_dict(&sm, "cycleOfDividendPayment");
        let mut cycle_anchor_date_of_dividend_payment = CycleAnchorDateOfDividendPayment::provide_from_input_dict(&sm, "cycleAnchorDateOfDividendPayment");

        if cycle_anchor_date_of_dividend_payment.is_none() {
            if cycle_of_dividend_payment.is_none() {
                cycle_anchor_date_of_dividend_payment = None;
            }
            else {
                let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
                cycle_anchor_date_of_dividend_payment = Some(CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).expect("ok"));

            }
        }

        // let cycle_anchor_date_of_dividend_payment = {
        //     let a = if cycle_of_dividend_payment.is_none() {
        //         None
        //     } else {
        //         let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
        //         CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).ok()
        //     };
        //     let b = CycleAnchorDateOfDividendPayment::provide_from_input_dict(sm, "CycleAnchorDateOfDividendPayment");
        //     if b.is_none() { a } else { b }
        // };

        let eomc = EndOfMonthConvention::provide_from_input_dict(&sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            contract_type: ContractType::provide_from_input_dict(&sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(&sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(&sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(&sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(&sm, "CounterpartyID"),
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            quantity: quantity,
            purchase_date: purchase_date,
            price_at_purchase_date: price_at_purchase_date,
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            price_at_termination_date: price_at_termination_date,
            market_object_code: MarketObjectCode::provide_from_input_dict(&sm, "marketObjectCode"),
            market_value_observed: market_value_observed,
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            cycle_of_dividend_payment: cycle_of_dividend_payment,
            cycle_anchor_date_of_dividend_payment: cycle_anchor_date_of_dividend_payment,
            market_object_code_of_dividends: MarketObjectCodeOfDividends::provide_from_input_dict(&sm, "marketObjectCodeOfDividends"),
            ..Default::default()
        };

        self.contract_terms = ct;
    }

    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Arc<dyn TraitExternalData>>) {
        self.risk_factor_external_data = risk_factor_external_data;
    }

    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>) {
        self.risk_factor_external_event = risk_factor_external_event;
    }

    fn init_related_contracts(&mut self, _sm: HashMap<String, Value>) {
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
                !(e.compare_to(&termination) == 1)
            });
            events.push(termination);
        }
        // remove all pre-status date events
        let tmpe = EventFactory::create_event(
            &Some(model.status_date.clone().unwrap().value().convert::<ScheduleTime>()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id
        );
        events.retain(|e| {
            !(e.compare_to(&tmpe) == -1)
        });

        // remove all post to-date events
        if to.is_some() {
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
                !(e.compare_to(&tmpe) == 1)
            });
        }

        self.event_timeline = events.clone();
        self.sort_events_timeline();
        // for e in self.event_timeline.iter() {
        //     let aa = e.event_time.unwrap().to_string();
        //     println!("{:?}", aa);
        // }
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

    fn next_day(&mut self, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {
        // ici on met Vec<TestResult> car il peut y avoir plusieur event le meme jour
        // itere un jour apres lautre
        let period1d = *PhantomIsoPeriodW::new(0,0,1);
        let next_status_date = self.status_date.convert_option::<PhantomIsoDatetimeW>().unwrap().value()
            + period1d;
        let next_event_index = (self.curr_event_index + 1) as usize;
        let mut next_event_date = self.event_timeline.get(next_event_index).unwrap().get_schedule_time();

        if next_status_date < next_event_date.value() {
            self.status_date = StatusDate::new(next_status_date).ok();
            let oo = self.status_date.clone()?.to_string();
            None
        }
        else { // case >=, seul = doit etre matche
            let mut result_vec: Vec<TestResult> = Vec::new();
            let mut curr_next_event_index = next_event_index;
            while next_status_date == next_event_date.value() {
                let ww = next_status_date.to_string();
                let www = next_event_date.to_string();

                result_vec.push(self.next_event(extract_results).expect("ok").expect("ok"));
                curr_next_event_index += 1;
                if curr_next_event_index == self.event_timeline.len() {
                    break;
                }
                next_event_date = self.event_timeline.get(curr_next_event_index).unwrap().get_schedule_time();
            }
            self.status_date = StatusDate::new(next_status_date).ok();
            Some(Ok(result_vec))
        }

    }

    fn next_event(&mut self, extract_results: bool) -> Option<Result<TestResult, String>> {


        let next_event_index = (self.curr_event_index + 1) as usize;
        if next_event_index < self.event_timeline.len() {

            self.eval_pof_contract_event(next_event_index);
            self.eval_stf_contract_event(next_event_index);
            self.curr_event_index += 1;
            if extract_results == true {
                let curr_testresult = TestResult {
                    eventDate: self.event_timeline[next_event_index].event_time.expect("fe").to_string(),
                    eventType: self.event_timeline[next_event_index].event_type.to_string(),
                    payoff: self.event_timeline[next_event_index].payoff.clone().expect("ok").to_string(),
                    currency: self.event_timeline[next_event_index].currency.clone().expect("ef").0,
                    notionalPrincipal: self.states_space.notional_principal.clone().expect("ok").to_string(),
                    nominalInterestRate: self.states_space.nominal_interest_rate.clone().expect("ok").to_string(),
                    accruedInterest: self.states_space.accrued_interest.clone().expect("ok").to_string(),
                };
                Some(Ok(curr_testresult))
            }
            else {
                Some(Err("Err ave TestResult".to_string()))
            }

        } else {
            None
        }
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
        self.sort_events_timeline();
        let events_len = self.event_timeline.len();
        let mut result_vec: Vec<TestResult> = Vec::new();

        while self.curr_event_index + 1 < events_len as i32 { // i < events_len {
            if self.curr_event_index > -1 {
                if date.is_some() {
                    if self.event_timeline[self.curr_event_index as usize].event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                        break
                    }
                }
            }

            let curr_testresult: Option<Result<TestResult, String>> = self.next_event(extract_results);
            if extract_results == true {
                if curr_testresult.clone().unwrap().is_ok() {
                    result_vec.push(curr_testresult.clone().unwrap().unwrap());
                }
            }
        }
        // Return evaluated events
        //self.event_timeline = events.clone();
        self.sort_events_timeline();

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
            curr_event_index: self.curr_event_index.clone(),
            states_space: self.states_space.clone(),
            status_date: self.status_date.clone(),
            first_event_date: self.first_event_date.clone(),
            last_event_date: self.last_event_date.clone(),
        }
    }
}

// Implémentation manuelle de PartialEq
impl PartialEq for STK {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for STK {}

impl Hash for STK {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}