use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::PayOff;
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
use gfs_lib_terms::terms::grp_contract_identification::CreatorID::CreatorID;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use gfs_lib_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::NextResetRate::NextResetRate;
use gfs_lib_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use gfs_lib_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use gfs_lib_terms::terms::grp_settlement::delivery_settlement::D::D;
use gfs_lib_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::{IsoCycleConvertToOption, IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::Value::Value;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::events::EventSequence::EventSequence;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub struct SWPPV {
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

impl TraitContractModel for SWPPV {
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
        let calendar = Calendar::provide_rc(&sm, "calendar");

        let maturity_date_tmp = MaturityDate::provide_from_input_dict(&sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };


        // Champs qui dépendent d'autres champs
        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestPayment::provide_from_input_dict(&sm, "cycleAnchorDateOfInterestPayment")
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(&sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };


        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfRateReset::from_str(&a).ok()
        } else {
            CycleAnchorDateOfRateReset::provide_from_input_dict(&sm,"cycleAnchorDateOfRateReset" )
        };

        let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide(&sm, "cyclePointOfInterestPayment");
        let cycle_point_of_rate_reset = if let Some(point) = &cycle_point_of_interest_payment {
            if point.to_string() == "B" {
                Some(CyclePointOfRateReset::new("E").expect("d"))
            } else {
                CyclePointOfRateReset::provide(&sm, "cyclePointOfRateReset")
            }
        } else {
            CyclePointOfRateReset::provide(&sm, "cyclePointOfRateReset")
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                &sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };
        let eomc = EndOfMonthConvention::provide_from_input_dict(&sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            accrued_interest: AccruedInterest::provide_from_input_dict(&sm, "accruedInterest"),
            accrued_interest2: AccruedInterest2::provide_from_input_dict(&sm, "accruedInterest2"),
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(&sm, "contractType"),
            status_date: StatusDate::provide_from_input_dict(&sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(&sm, "contractRole"),
            creator_id: CreatorID::provide_from_input_dict(&sm, "creatorID"),
            contract_id: ContractID::provide_from_input_dict(&sm, "contractID"),
            counterparty_id: CounterpartyID::provide_from_input_dict(&sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(&sm, "marketObjectCode"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(&sm, "nominalInterestRate"),
            nominal_interest_rate2: NominalInterestRate2::provide_from_input_dict(&sm, "nominalInterestRate2"),
            day_count_convention: day_count_convention,
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate"),
            maturity_date: maturity_date,
            notional_principal: NotionalPrincipal::provide_from_input_dict(&sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(&sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(&sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(&sm, "marketObjectCodeOfRateReset"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(&sm, "cyclePointOfInterestPayment"),
            fixing_period: FixingPeriod::provide_from_input_dict(&sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(&sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(&sm, "rateMultiplier"),
            delivery_settlement: DeliverySettlement::provide_from_input_dict(&sm, "deliverySettlement"),
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

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.nominal_interest_rate2 = model.nominal_interest_rate2.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
            states.accrued_interest2 = AccruedInterest2::new(role_sign * model.accrued_interest2.clone().unwrap().value()).ok();
            states.last_interest_period = Some(0.0);
        }

        self.states_space = states;
    }

    fn init_contract_event_timeline(&mut self, to : Option<PhantomIsoDatetimeW>) {
        let model = &self.contract_terms;
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_FXOUT")),
                Some(StatesTransitionFunction::from_str("STF_PRD_SWPPV")),
                &None,
                &model.contract_id,
            );
            events.push(e);
        }

        // Initial exchange event
        let e: ContractEvent = EventFactory::create_event(
            &model.initial_exchange_date.clone().convert_option::<ScheduleTime>(),
            &EventType::IED,
            &model.currency,
            Some(PayOffFunction::from_str("POF_IED_SWPPV")),
            Some(StatesTransitionFunction::from_str("STF_IED_SWPPV")),
            &None,
            &model.contract_id,
        );
        events.push(e);

        // Principal redemption event
        let e: ContractEvent = EventFactory::create_event(
            &model.maturity_date.clone().map(|rc| (*rc).clone().convert::<ScheduleTime>()),
            &EventType::MD,
            &model.currency,
            Some(PayOffFunction::from_str("POF_MD_SWPPV")),
            Some(StatesTransitionFunction::from_str("STF_MD_SWPPV")),
            &None,
            &model.contract_id,
        );
        events.push(e);

        // Interest payment events
        if model.delivery_settlement.is_none() || model.delivery_settlement == Some(DeliverySettlement::D(D)) {
            // In case of physical delivery (delivery of individual cash flows)
            let interest_schedule = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone().convert_option::<StartTime>(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                &model.cycle_of_interest_payment.clone().convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention.clone(),
                Some(true)
            );

            // Fixed rate events
            let fixed_rate_events = EventFactory::create_events(
                &interest_schedule.iter().map(|e| e.convert::<ScheduleTime>()).collect(),
                &EventType::IPFX,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IPFix_SWPPV")),
                Some(StatesTransitionFunction::from_str("STF_IPFix_SWPPV")),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(fixed_rate_events);

            // Floating rate events
            let floating_rate_events = EventFactory::create_events(
                &interest_schedule.iter().map(|e| e.convert::<ScheduleTime>()).collect(),
                &EventType::IPFL,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IPFloat_SWPPV")),
                Some(StatesTransitionFunction::from_str("STF_IPFloat_SWPPV")),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(floating_rate_events);
        } else {
            // In case of cash delivery (cash settlement)
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment.clone().convert_option::<StartTime>(),
                    &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                    &model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>(),
                    &model.end_of_month_convention,
                    Some(true)
                ).iter().map(|e| e.convert::<ScheduleTime>()).collect(),
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_SWPPV")),
                Some(StatesTransitionFunction::from_str("STF_IP_SWPPV")),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset.convert_option::<StartTime>(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                &model.cycle_of_rate_reset.convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(false),
            ).iter().map(|e| e.convert::<ScheduleTime>()).collect(),
            &EventType::RR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_RR_PAM")),
            Some(StatesTransitionFunction::from_str("STF_RR_SWPPV")),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(rate_reset_events);

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone().convert::<ScheduleTime>()),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_FXOUT")),
                Some(StatesTransitionFunction::from_str("STF_TD_SWPPV")),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            &model.status_date.clone().convert_option::<ScheduleTime>(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().clone().unwrap().convert::<ScheduleTime>()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        self.event_timeline = events.clone();
        self.sort_events_timeline();
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
            self.event_timeline[id_ce].payoff = Some(a.expect("ok"));
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

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &self.contract_terms.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone().convert::<ScheduleTime>()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            self.event_timeline.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        // self.event_timeline = events.clone();

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

impl fmt::Display for SWPPV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SWPPV")
    }
}


impl fmt::Debug for SWPPV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SWPPV")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for SWPPV {
    fn clone(&self) -> Self {
        SWPPV {
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
impl PartialEq for SWPPV {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for SWPPV {}

impl Hash for SWPPV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}
