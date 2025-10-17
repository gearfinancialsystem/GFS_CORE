use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::Payoff;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
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
use gfs_lib_terms::terms::grp_notional_principal::Currency2::Currency2;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_settlement::delivery_settlement::D::D;
use gfs_lib_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use gfs_lib_terms::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoPeriod::TraitMarkerIsoPeriod;
use gfs_lib_types::traits::TraitConvert::{IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::Value::Value;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;

use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::contracts::Pam::PAM;
use crate::events::EventSequence::EventSequence;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub struct FXOUT {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Box<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Box<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
}

impl TraitContractModel for FXOUT {

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

        let calendar = Calendar::provide_rc(sm, "calendar");

        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };

        // Gestion des dépendances
        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
            "businessDayAdjuster",
            calendar_clone.unwrap()
        )
        };
        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        // purchase date
        let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");

        // priceatpurchasedate
        let mut price_at_purchase_date = PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate");
        if price_at_purchase_date.is_none() {
            price_at_purchase_date = PriceAtPurchaseDate::new(0.0).ok();
        }
        // termination date
        let termination_date = TerminationDate::provide_from_input_dict(sm, "terminationDate");

        // price at termination date
        let mut price_at_termination_date = PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate");
        if price_at_termination_date.is_none() {
            price_at_termination_date = PriceAtTerminationDate::new(0.0).ok();
        }
        // delivery settlement
        let delivery_settlement = DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement");

        // settlement period
        let mut settlement_period = SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod");
        if settlement_period.is_none() {
            settlement_period = SettlementPeriod::from_str("P0D").ok();
        }

        let ct = ContractTerms {
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            currency2: Currency2::provide_from_input_dict(sm, "currency2"),
            maturity_date: maturity_date,
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            notional_principal2: NotionalPrincipal2::provide_from_input_dict(sm, "notionalPrincipal2"),
            purchase_date: purchase_date,
            price_at_purchase_date: price_at_purchase_date,
            termination_date: termination_date,
            price_at_termination_date: price_at_termination_date,
            delivery_settlement: delivery_settlement,
            settlement_period: settlement_period,
            ..Default::default()
        };


        self.contract_terms = ct
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

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>)  {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
        states.status_date = model.status_date.clone();
        //Ok(states)
        self.states_space = states
    }

    fn init_contract_event_timeline(&mut self, to: Option<PhantomIsoDatetimeW> ) {
        let mut events: Vec<ContractEvent> = Vec::new();
        let model = &self.contract_terms;

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_FXOUT")),
                Some(StatesTransitionFunction::from_str("STF_PRD_STK")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let e: ContractEvent = EventFactory::create_event(
                &Some(termination_date.clone().convert::<ScheduleTime>()),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_FXOUT")),
                Some(StatesTransitionFunction::from_str("STF_TD_STK")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }
        else {
            // Settlement events
            if (model.delivery_settlement.is_none()) || (model.delivery_settlement == Some(DeliverySettlement::D(D))) {
                let e: ContractEvent = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<ScheduleTime>(),
                    &EventType::MD,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_MD1_FXOUT")),
                    Some(StatesTransitionFunction::from_str("STF_MD1_FXOUT")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e);

                let e: ContractEvent = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<ScheduleTime>(),
                    &EventType::MD,
                    &Some(model.currency2.clone().unwrap().to_currency()),
                    Some(PayOffFunction::from_str("POF_MD2_FXOUT")),
                    Some(StatesTransitionFunction::from_str("STF_MD2_FXOUT")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e);
            }
            else {
                let shifted_maturity_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                    &(
                        model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().add_period(
                            model.settlement_period.clone().unwrap().value().clone()
                        )
                    ).convert::<PhantomIsoDatetimeW>()
                );

                let e: ContractEvent = EventFactory::create_event(
                    &Some(shifted_maturity_date).convert_option::<ScheduleTime>(),
                    &EventType::STD,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_STD_FXOUT")),
                    Some(StatesTransitionFunction::from_str("STF_STD_FXOUT")),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e);
            }
        }

        // Remove all pre-status date events
        let status_event: ContractEvent = EventFactory::create_event(
            &model.status_date.convert_option::<ScheduleTime>(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );
        // println!("ok");
        events.retain(|e| e.compare_to(&status_event) != -1);

        if to.is_some() {
            // Remove all post to-date events
            let to_event = EventFactory::create_event(
                &Some(to.clone().unwrap().convert::<ScheduleTime>()),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            events.retain(|e| e.compare_to(&to_event) != 1);
        }
        
        // Sort events according to their time of occurrence
        events.sort();

        self.event_timeline = events;
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

    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> { // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>

        let events = &mut self.event_timeline.clone();
        // let mut events = events.clone();

        //////////////////////////////////////////////////
        // Sort events according to their time sequence //
        //////////////////////////////////////////////////
        self.sort_events_timeline();

        ////////////////////////////////////////////////////////////////////
        // Apply events according to their time sequence to current state //
        ////////////////////////////////////////////////////////////////////
        let mut result_vec: Vec<TestResult> = Vec::new();

        //println!("ZORG ZORG");

        let mut i: usize = 0;
        for event in events.iter_mut() {
            // let a = event.event_time.expect("fd");
            // let b = EventTime::new(date.expect("fo").value()).expect("ok");

            if date.is_some() {
                if event.event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                    break
                }
            }
            self.eval_pof_contract_event(i);
            //println!("nominalprincipal{:?}", self.states_space.notional_principal);
            //println!("payoff{:?}", self.event_timeline[i].payoff);
            self.eval_stf_contract_event(i);
            // let a = self.event_timeline[i].payoff.clone().expect("ok").to_string();
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

            i += 1;
        }

        ////////////////////////////////////////////////////////
        // Remove pre-purchase events if purchase date is set //
        ////////////////////////////////////////////////////////

        if self.contract_terms.purchase_date.is_some() {
            // let purchase_date = model.purchase_date;
            let purchase_event: ContractEvent = EventFactory::create_event(
                &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );
            events.retain(|e| {
                e.get_event_type() == EventType::AD || e >= &purchase_event
            });
        }
        /////////////////////////////
        // Return evaluated events //
        /////////////////////////////
        //Ok(events)
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


impl fmt::Display for FXOUT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FXOUT")
    }
}

impl fmt::Debug for FXOUT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FXOUT")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for FXOUT {
    fn clone(&self) -> Self {
        FXOUT {
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