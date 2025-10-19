use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::Payoff;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_settlement::delivery_settlement::S::S;
use gfs_lib_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::{IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::Value::Value;
use crate::attributes::ContractElem::ContractElem;
use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::Dependence::Dependence;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;
use crate::attributes::ContractRules::ContractRules;
use crate::attributes::ContractRules::ContractRules::SwapsRulesE;
use crate::attributes::ContractRules::SwapsRules;

use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::reference_type::ReferenceType::ReferenceType;
use crate::events::EventFactory::EventFactory;
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::functions::stk::StatesTransitionFunctionSTK::StatesTransitionFunctionSTK::STF_PRD_STK;
use crate::functions::swaps::PayOffFunctionSWAPS::PayOffFunctionSWAPS::POF_PRD_SWAPS;

pub struct SWAPS {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
}

impl TraitContractModel for SWAPS {

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

    fn init_contract_terms(&mut self, sm: HashMap<String, Value>) {

        // price at purchase date
        let mut price_at_purchase_date = PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate");
        if price_at_purchase_date.is_none() {
            price_at_purchase_date = Some(PriceAtPurchaseDate::new(0.0).unwrap());
        }

        // price at termination date
        let mut price_at_termination_date = PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate");
        if price_at_termination_date.is_none() {
            price_at_termination_date = Some(PriceAtTerminationDate::new(0.0).unwrap());
        }

        let ct = ContractTerms {
            contract_id: ContractID::provide_from_input_dict(&sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(&sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(&sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(&sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(&sm, "marketObjectCode"),
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            purchase_date: PurchaseDate::provide_from_input_dict(&sm, "purchaseDate"),
            price_at_purchase_date: price_at_purchase_date,
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            price_at_termination_date: price_at_termination_date,
            delivery_settlement: DeliverySettlement::provide_from_input_dict(&sm, "deliverySettlement"),
            contract_type: ContractType::provide_from_input_dict(&sm, "contractType"),
            //contract_structure: contract_structure,
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

    fn init_related_contracts(&mut self, sm: HashMap<String, Value>) {
        // self.related_contracts = None;
        let csvec = sm.get("contractStructure").clone().unwrap().clone().into_cs().unwrap();

        let mut sr = SwapsRules::default();

        let mut hs: HashMap<ContractID, ContractElem> = HashMap::new();
        for (i, cs) in csvec.iter().enumerate() {
            let ci = ContractID::new(cs.object.get(&"contractID".to_string()).expect("ok").clone()).expect("ok");
            hs.insert(ci ,{
                ContractElem {
                    contract_elem: ContractModel::new(
                        // a ameliorer
                {
                            let mut hmtmp: HashMap<String, Value> = HashMap::new();
                            for (k, v) in &cs.object {
                                hmtmp.insert(k.clone(), Value::from_string(v.clone()));
                            }
                            hmtmp

                        },
                        self.risk_factor_external_data.clone(),
                        self.risk_factor_external_event.clone()
                    ).expect("ok"),
                    dependence: Dependence::Owned
                }
            });
            if cs.referenceRole.clone() == "FIL" {
                sr.identifier_leg1 = ContractID::new(cs.object.get(&"contractID".to_string()).expect("ok").clone()).expect("ok");
                sr.reference_type_leg1 = ReferenceType::from_str(cs.referenceType.clone().as_str()).expect("ok");
                sr.reference_role_leg1 = ReferenceRole::from_str(cs.referenceRole.clone().as_str()).expect("ok");
            }
            else { // SEL
                sr.identifier_leg2 = ContractID::new(cs.object.get(&"contractID".to_string()).expect("ok").clone()).expect("ok");
                sr.reference_type_leg2 = ReferenceType::from_str(cs.referenceType.clone().as_str()).expect("ok");
                sr.reference_role_leg2 = ReferenceRole::from_str(cs.referenceRole.clone().as_str()).expect("ok");
            }

        }

        self.related_contracts = Some(RelatedContracts {
            contract_set: hs,
            contract_structure: Some(ContractRules::SwapsRulesE(sr)),
        });
        // self.related_contracts = None;
    }

    fn init_status_date(&mut self) {

        self.status_date = self.contract_terms.status_date;
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) { // event_at_t0: ContractEvent<IsoDatetime, IsoDatetime>,
        let model = &self.contract_terms;
        // let cs = &self.contract_structure.clone().expect("On attend un contract structure ici");
        let rc = self.related_contracts.clone().expect("On attend un related contracts ici");
        let cst = rc.contract_structure.clone().expect("On attend un contract structure ici");
        let cse = rc.contract_set.clone();

        //let first_leg_model = cs.iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        //let second_leg_model = cs.iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();

        let (id_first_leg, type_first_leg, role_first_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg1.clone(),
                v.reference_type_leg1.clone(),
                v.reference_role_leg1.clone()),
        };

        let (id_sec_leg, type_sec_leg, role_sec_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg2.clone(),
                v.reference_type_leg2.clone(),
                v.reference_role_leg2.clone()),
        };

        let ct_first_leg = cse.get(&id_first_leg).expect("ok");
        let ct_second_leg = cse.get(&id_sec_leg).expect("ok");

        let mut states = StatesSpace::default();

        states.status_date = model.status_date.clone();
        states.contract_performance = if model.contract_performance.is_some() {
            model.contract_performance
        } else { None };

        // recuperer la maturité, en fonction du types de contrat enfants autorisés
        // les deux contrats enfants doivent etres du meme ct (?)
        // cest ici quon defini ce quil est possible de faire comme sous jacent de swaps
        let mat = match (ct_first_leg.contract_elem.clone(), ct_second_leg.contract_elem.clone()) {
            (ContractModel::PAM(v1), ContractModel::PAM(v2)) => {
                let mat1 = v1.contract_terms.maturity_date.clone().unwrap();
                let mat2 = v2.contract_terms.maturity_date.clone().unwrap();
                if mat1 > mat2 {
                    mat1
                }
                else {
                    mat2
                }
            },
            (ContractModel::FXOUT(v1), ContractModel::FXOUT(v2)) => {
                let mat1 = v1.contract_terms.maturity_date.clone().unwrap();
                let mat2 = v2.contract_terms.maturity_date.clone().unwrap();
                if mat1 > mat2 {
                    mat1
                }
                else {
                    mat2
                }
            },
            (_, _) => {
                panic!("On ne peut pas avoir deux contrats enfants de type differents");
            }
        };
        states.maturity_date = MaturityDate::new(mat.clone().value()).ok();

        //states.accrued_interest = event_at_t0.states().accrued_interest;
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        self.states_space = states;

    }

    fn init_contract_event_timeline(&mut self, to : Option<PhantomIsoDatetimeW>) {

        // self.set_legs_contract_models();

        let model = &self.contract_terms;
        let mut events: Vec<ContractEvent> = Vec::new();

        let mut rc = self.related_contracts.clone().expect("On attend un related contracts ici");
        let mut cst = rc.contract_structure.clone().expect("On attend un contract structure ici");


        let (id_first_leg, type_first_leg, role_first_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg1.clone(),
                v.reference_type_leg1.clone(),
                v.reference_role_leg1.clone()),
        };

        let (id_sec_leg, type_sec_leg, role_sec_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg2.clone(),
                v.reference_type_leg2.clone(),
                v.reference_role_leg2.clone()),
        };

        let mut cse = rc.contract_set.clone();
        {
            let mut ct_first_leg = cse.get_mut(&id_first_leg).expect("ok");
            ct_first_leg.contract_elem.run_schedule(to);
            events.extend(ct_first_leg.contract_elem.get_current_timeline());
        }
        {
            let mut ct_second_leg = cse.get_mut(&id_sec_leg).expect("ok");
            ct_second_leg.contract_elem.run_schedule(to);
            events.extend(ct_second_leg.contract_elem.get_current_timeline());
        }

        // purchase event
        if model.purchase_date.is_some() {
            let e: ContractEvent = EventFactory::create_event(
                &model.purchase_date.convert_option::<ScheduleTime>(), // voir si le typage fort est correct
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_SWAPS")),
                Some(StatesTransitionFunction::from_str("STF_PRD_SWAPS")),
                &None,
                &model.contract_id);
            events.push(e);
        }

        // termination date
        if model.termination_date.is_some() {
            let termination: ContractEvent = EventFactory::create_event(
                &model.termination_date.convert_option::<ScheduleTime>(),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_SWAPS")),
                Some(StatesTransitionFunction::from_str("STF_TD_STK")),
                &None,
                &model.contract_id
            );
            events.retain(|e| !(e.compare_to(&termination) == 1));
            events.push(termination);
        }

        //let a = &model.status_date;
        let ee: ContractEvent = EventFactory::create_event(
            &model.status_date.convert_option::<ScheduleTime>(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id);
        events.retain(|e| e.compare_to(&ee) != -1);

        events.retain(|e| e.compare_to(
            &EventFactory::create_event(
                &Some(to.clone().unwrap().convert::<ScheduleTime>()),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id) ) != 1);

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
        //
        // let mut result_vec: Vec<TestResult> = Vec::new();
        //
        // let mut rc = self.related_contracts.clone().expect("On attend un related contracts ici");
        // let mut cst = rc.contract_structure.clone().expect("On attend un contract structure ici");
        //
        //
        // let (id_first_leg, type_first_leg, role_first_leg) = match cst.clone() {
        //     SwapsRulesE(v) => (
        //         v.identifier_leg1.clone(),
        //         v.reference_type_leg1.clone(),
        //         v.reference_role_leg1.clone()),
        // };
        //
        // let (id_sec_leg, type_sec_leg, role_sec_leg) = match cst.clone() {
        //     SwapsRulesE(v) => (
        //         v.identifier_leg2.clone(),
        //         v.reference_type_leg2.clone(),
        //         v.reference_role_leg2.clone()),
        // };
        //
        // let mut cse = rc.contract_set.clone();
        //
        // let mut ct_first_leg = cse.get_mut(&id_first_leg).expect("ok");
        // ct_first_leg.contract_elem.get_current_timeline();
        // let first_leg_events = ct_first_leg.contract_elem.get_current_timeline();
        //
        // let mut ct_second_leg = cse.get_mut(&id_sec_leg).expect("ok");
        // ct_second_leg.contract_elem.get_current_timeline();
        // let second_leg_events = ct_second_leg.contract_elem.get_current_timeline();
        //
        // self.sort_events_timeline();
        //
        // if self.contract_terms.delivery_settlement.clone().unwrap() == DeliverySettlement::S(S) {
        //     let mut a = self.filter_and_nett_congruent_events(
        //         &self.contract_terms.contract_id.clone()
        //     );
        //     self.event_timeline = a.clone();
        // }
        //
        // self.sort_events_timeline();
        //
        // // ici en gros itere sur les ce genere au niveau parent du swap, cest ce que verifie le if
        // self.event_timeline.iter().for_each(|event| {
        //     if event.get_contract_id() == self.contract_terms.contract_id.clone().unwrap() {
        //         if event.event_type == EventType::PRD || event.event_type == EventType::TD {
        //             let mut parent_state = StatesSpace::default();
        //
        //             //let first_leg_events = self.get_legs_events_by_refrole(ReferenceRole::FIL);
        //             //let second_leg_events = self.get_legs_events_by_refrole(ReferenceRole::SEL);
        //
        //             let f_l_events_at_timepoint = first_leg_events.clone().iter().filter(|e| {
        //                 e.event_time == event.event_time
        //             }).map(|e| e.clone()).collect::<Vec<_>>();
        //
        //             let s_l_events_at_timepoint = second_leg_events.clone().iter().filter(|e| {
        //                 e.event_time == event.event_time
        //             }).map(|e| e.clone()).collect::<Vec<_>>();
        //
        //             let fl_ipac: f64;
        //             let sl_ipac: f64;
        //             if f_l_events_at_timepoint.is_empty() {
        //                 fl_ipac = 0.0;
        //             }
        //             else {
        //                 fl_ipac = if f_l_events_at_timepoint.iter().any(|e| e.event_type == EventType::IP) {
        //                     0.0
        //                 } else {
        //                     f_l_events_at_timepoint.iter()
        //                         .find(|e| e.event_type == EventType::PR)
        //                         .map(|e| e.states().accrued_interest.clone().unwrap().value())
        //                         .unwrap_or(0.0)
        //                 };
        //             }
        //             sl_ipac = if s_l_events_at_timepoint.is_empty() {
        //                 0.0
        //             }
        //             else {
        //                 if s_l_events_at_timepoint.iter().any(|e| e.event_type == EventType::IP) {
        //                     0.0
        //                 } else {
        //                     s_l_events_at_timepoint.iter()
        //                         .find(|e| e.event_type == EventType::PR)
        //                         .map(|e| e.states().accrued_interest.clone().unwrap().value())
        //                         .unwrap_or(0.0)
        //                 }
        //             };
        //
        //             parent_state.accrued_interest = AccruedInterest::new(fl_ipac + sl_ipac).ok();
        //
        //         }
        //         else {
        //             //event.clone().eval(None, None, None, None, None);
        //             //A REFLECHIR
        //         }
        //
        //     }
        // });
        //
        // if self.contract_terms.purchase_date.is_some(){
        //     let purchase: ContractEvent = EventFactory::create_event(
        //         &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
        //         &EventType::PRD,
        //         &self.contract_terms.currency,
        //         Some(PayOffFunction::from_str("POF_PRD_SWAPS")),
        //         Some(StatesTransitionFunction::from_str("STF_PRD_STK")),
        //         &None,
        //         &self.contract_terms.contract_id
        //     );
        //     // Remove the filtered events from the main events list
        //     self.event_timeline.retain(|e| {
        //         e.compare_to(&purchase) != -1
        //     });
        //
        // }
        // self.sort_events_timeline();
        //
        // // self.contract_events = self.contract_events.clone();
        // //Ok(events)
        //
        //
        // // recup des resultats
        // if extract_results == false {
        //
        //     return None;
        // }
        // else {
        //     ////////////////////////////////////////////////////////
        //     // Remove pre-purchase events if purchase date is set //
        //     ////////////////////////////////////////////////////////
        //     result_vec.retain(|e| {
        //         if self.contract_terms.purchase_date.is_some() {
        //             let purchase_event: ContractEvent = EventFactory::create_event(
        //                 &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
        //                 &EventType::PRD,
        //                 &self.contract_terms.currency,
        //                 None,
        //                 None,
        //                 &None,
        //                 &self.contract_terms.contract_id,
        //             );
        //             let epoch_millis = IsoDatetime::from_str(e.eventDate.as_str()).clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
        //             let epoch_offset = epoch_millis + EventSequence::time_offset(&EventType::from_str(e.eventType.as_str()).expect("exist"));
        //             EventType::from_str(e.eventType.as_str()).expect("exist") == EventType::AD || epoch_offset as f64 >= purchase_event.epoch_offset.unwrap().value()
        //         } else { true }
        //     });
        //     return Some(Ok(result_vec));
        // }
        todo!()
    }

    fn sort_events_timeline(&mut self) {
        self.event_timeline.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));
    }

}

impl SWAPS {

    pub fn filter_and_nett_congruent_events(&mut self,
        parent_contract_ID: &Option<ContractID>) -> Vec<ContractEvent> {

        // first_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        // second_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        let mut rc = self.related_contracts.clone().expect("On attend un related contracts ici");
        let mut cst = rc.contract_structure.clone().expect("On attend un contract structure ici");

        let (id_first_leg, type_first_leg, role_first_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg1.clone(),
                v.reference_type_leg1.clone(),
                v.reference_role_leg1.clone()),
        };

        let (id_sec_leg, type_sec_leg, role_sec_leg) = match cst.clone() {
            SwapsRulesE(v) => (
                v.identifier_leg2.clone(),
                v.reference_type_leg2.clone(),
                v.reference_role_leg2.clone()),
        };

        let mut cse = rc.contract_set.clone();

        let mut ct_first_leg = cse.get_mut(&id_first_leg).expect("ok");
        ct_first_leg.contract_elem.sort_current_timeline();
        let first_leg_events = ct_first_leg.contract_elem.get_current_timeline();

        let mut ct_second_leg = cse.get_mut(&id_sec_leg).expect("ok");
        ct_second_leg.contract_elem.sort_current_timeline();
        let second_leg_events = ct_second_leg.contract_elem.get_current_timeline();


        let mut events = Vec::new();

        // Helper function to filter events by type
        let filter_events = |events: &[ContractEvent], event_type: EventType| -> Vec<ContractEvent> {
            events.iter()
                .filter(|event| event.event_type == event_type)
                .cloned()
                .collect()
        };

        // Define a macro to reduce repetition for each event type
        macro_rules! process_event_type {
            ($event_type:expr) => {
                let mut first_leg_x = filter_events(&first_leg_events, $event_type);
                let mut second_leg_x = filter_events(&second_leg_events, $event_type);
                SWAPS::net_singular_event(
                    parent_contract_ID.clone(),
                    &mut events,
                    &mut first_leg_x,
                    &mut second_leg_x,
                );
            };
        }

        // Process IED and MD events (which use netSingularEvent)
        process_event_type!(EventType::IED);
        process_event_type!(EventType::MD);

        // Process PR events
        let first_leg_pr = filter_events(&first_leg_events, EventType::PR);
        let second_leg_pr = filter_events(&second_leg_events, EventType::PR);
        SWAPS::net_congruent_events(
            first_leg_pr,
            second_leg_pr,
            &mut events,
            parent_contract_ID.clone(),
        );

        // Process IP events
        let first_leg_ip = filter_events(&first_leg_events, EventType::IP);
        let second_leg_ip = filter_events(&second_leg_events, EventType::IP);
        SWAPS::net_congruent_events(
            first_leg_ip,
            second_leg_ip,
            &mut events,
            parent_contract_ID.clone(),
        );

        events
    }

    pub fn net_singular_event(parent_contract_id: Option<ContractID>,
                              events: &mut Vec<ContractEvent>,
                              list_first_leg: &mut Vec<ContractEvent>,
                              list_second_leg: &mut Vec<ContractEvent>,){

        if !list_first_leg.is_empty() && !list_second_leg.is_empty() {
            let first_leg_event = &list_first_leg.clone()[0];
            let second_leg_event = &list_second_leg.clone()[0];

            if first_leg_event.event_time == second_leg_event.event_time {
                // Remove from events list - but we can't do exact object matching
                // So we'll use a more functional approach: filter events not matching our two
                let mut new_events = Vec::new();
                let mut first_found = false;
                let mut second_found = false;

                for event in events.drain(..) {
                    if !first_found && event.event_time == first_leg_event.event_time {
                        first_found = true;
                        continue; // Skip this event (don't add to new_events)
                    }
                    if !second_found && event.event_time == second_leg_event.event_time {
                        second_found = true;
                        continue; // Skip this event
                    }
                    new_events.push(event);
                }
                *events = new_events;

                // Remove the first element from both leg lists
                list_first_leg.remove(0);
                list_second_leg.remove(0);

                // Create and add the netting event
                let netting_event = SWAPS::netting_event(
                    Some(first_leg_event.clone()),
                    Some(second_leg_event.clone()),
                    parent_contract_id,
                );
                events.push(netting_event);
            }
        }
    }

    pub fn net_congruent_events(
        first_leg_events: Vec<ContractEvent>,
        second_leg_events: Vec<ContractEvent>,
        events: &mut Vec<ContractEvent>,
        parent_contract_ID: Option<ContractID>) {

        let mut first_leg = first_leg_events;
        let mut second_leg = second_leg_events;

        // Sort both lists by event time
        first_leg.sort_by(|a, b| a.event_time.cmp(&b.event_time));
        second_leg.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        let mut i = 0;
        let mut j = 0;

        // These will hold indices of events to remove
        let mut first_indices_to_remove = Vec::new();
        let mut second_indices_to_remove = Vec::new();

        // We'll iterate through both lists
        while i < first_leg.len() {
            // Check if there are more elements in second_leg
            while j < second_leg.len() {
                let first_event = &first_leg[i];
                let second_event = &second_leg[j];

                if first_event.event_time == second_event.event_time {
                    // Found matching events, create a netting event
                    let netting_event = SWAPS::netting_event(
                        Some(first_event.clone()),
                        Some(second_event.clone()),
                        parent_contract_ID.clone(),
                    );
                    events.push(netting_event);

                    // Mark these indices for removal
                    first_indices_to_remove.push(i);
                    second_indices_to_remove.push(j);

                    // Move both indices forward
                    i += 1;
                    j += 1;

                    // If we found a match, break out of inner loop
                    // equivalent to the 'break' in the Java code
                    break;
                } else if second_event.event_time < first_event.event_time {
                    // Second event is earlier, move to next in second leg
                    j += 1;
                } else {
                    // No match found for this first event, move to next
                    break;
                }
            }

            // If we didn't find a match after checking all second leg events
            // or if we're at the end of the second leg list
            if j >= second_leg.len() {
                i += 1;
                // Reset j to 0 for the next first leg event (though this might not match Java behavior)
                j = 0;
            }
        }

        // Remove marked events from first leg
        // Sort indices in descending order to avoid shifting issues when removing
        first_indices_to_remove.sort_unstable();
        first_indices_to_remove.dedup(); // in case of duplicates (shouldn't happen)
        for index in first_indices_to_remove.iter().rev() {
            if *index < first_leg.len() {
                first_leg.remove(*index);
            }
        }

        // Remove marked events from second leg
        second_indices_to_remove.sort_unstable();
        for index in second_indices_to_remove.iter().rev() {
            if *index < second_leg.len() {
                second_leg.remove(*index);
            }
        }

        // Add remaining events from both legs
        events.extend(first_leg);
        events.extend(second_leg);

    }

    pub fn netting_event(
        e1: Option<ContractEvent>,
        e2: Option<ContractEvent>,
        parent_contract_id: Option<ContractID>,
    ) -> ContractEvent{
        let netting = EventFactory::create_event(
            &e1.clone().unwrap().event_time.convert_option::<ScheduleTime>(),
            &e1.clone().unwrap().event_type,
            &e1.clone().unwrap().currency,
            Some(PayOffFunction::from_str("POF_NET_SWAPS")),
            Some(StatesTransitionFunction::from_str("TF_NET_SWAPS")),
            &None,
            &parent_contract_id.clone(),
        );
        netting
    }
}


impl fmt::Display for SWAPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SWAPS")
    }
}

impl fmt::Debug for SWAPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PAM")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for SWAPS {
    fn clone(&self) -> Self {
        SWAPS {
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

// Implémentation manuelle de PartialEq
impl PartialEq for SWAPS {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for SWAPS {}

impl Hash for SWAPS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::util_tests::TestsUtils::{convert_value_map_to_string_map_ref, test_read_and_parse_json};
//     use crate::util_tests::TestsUtils::json_to_dico;
//     use crate::utils::Value::Value;
//     use std::error::Error;
//     use std::collections::{HashMap, HashSet};
//     use std::hash::Hash;
//     use chrono::ParseError;
//     use log::debug;
//     use crate::attributes::ContractReference::ContractReference;
//     use crate::exceptions::ContractTypeUnknownException::ContractError;
//
//
//     fn load_dico_tests() -> Vec<Value> {
//         let pathx = "/home/cet/Projects/ACTUS-CORE/actus-core-master-rust-project-v2/libs/gfs_lib_contract/tests_sets/actus-tests-swaps.json";
//         let json_value = test_read_and_parse_json(pathx).unwrap();
//         let dico_from_json = json_to_dico(json_value);
//         dico_from_json
//     }
//     fn are_contracts_equal(
//         mut contracts1: Vec<HashMap<String, Value>>,
//         mut contracts2: Vec<HashMap<String, Value>>
//     ) -> bool {
//         // 1. Vérifier que les vecteurs ont la même longueur
//         let a = contracts1.len();
//         let b = contracts2.len();
//
//         if a != b {
//             return false;
//         } else {
//             contracts1.sort_by(|a, b| {
//                 // Comparaison par eventTime (String) en premier
//                 let time_order = {
//                     let a_time = a.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     let b_time = b.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     match (a_time, b_time) {
//                         (Some(a), Some(b)) => a.cmp(b),
//                         (Some(_), None) => std::cmp::Ordering::Less,
//                         (None, Some(_)) => std::cmp::Ordering::Greater,
//                         (None, None) => std::cmp::Ordering::Equal,
//                     }
//                 };
//
//                 // Si les eventTime sont égaux, on compare par la valeur f64
//                 if time_order == std::cmp::Ordering::Equal {
//                     // Récupération des valeurs f64 avec gestion des erreurs
//                     let a_value: f64 = match a.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//                     let b_value: f64 = match b.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//
//                     // Comparaison des f64
//                     a_value.partial_cmp(&b_value).unwrap_or(std::cmp::Ordering::Equal)
//                 } else {
//                     time_order
//                 }
//             });
//             // contracts1.sort_by(|a, b| {
//             //     let a_time = a.get("eventTime").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     let b_time = b.get("eventTime").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     a_time.cmp(&b_time)
//             // });
//
//             contracts2.sort_by(|a, b| {
//                 // Comparaison par eventTime (String) en premier
//                 let time_order = {
//                     let a_time = a.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     let b_time = b.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     match (a_time, b_time) {
//                         (Some(a), Some(b)) => a.cmp(b),
//                         (Some(_), None) => std::cmp::Ordering::Less,
//                         (None, Some(_)) => std::cmp::Ordering::Greater,
//                         (None, None) => std::cmp::Ordering::Equal,
//                     }
//                 };
//
//                 // Si les eventTime sont égaux, on compare par la valeur f64
//                 if time_order == std::cmp::Ordering::Equal {
//                     // Récupération des valeurs f64 avec gestion des erreurs
//                     let a_value: f64 = match a.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//                     let b_value: f64 = match b.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//
//                     // Comparaison des f64
//                     a_value.partial_cmp(&b_value).unwrap_or(std::cmp::Ordering::Equal)
//                 } else {
//                     time_order
//                 }
//             });
//             // contracts2.sort_by(|a, b| {
//             //     let a_time = a.get("eventDate").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     let b_time = b.get("eventDate").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     a_time.cmp(&b_time)
//             // });
//
//             let mut vec_bool: Vec<bool> = vec![];
//             let mut i = 0;
//             for hm in contracts1.into_iter() {
//                 for (k, v)  in hm.iter() {
//
//                     match k.as_str() {
//                         "eventDate" => {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             }
//                             else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "eventType" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             }
//                             else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "payoff" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "currency" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             } else
//                             {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "notionalPrincipal" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 //let invest = hm.get("state");
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "nominalInterestRate" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "accruedInterest" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         _ => {}
//                     }
//                 }
//
//                 i = i + 1;
//             }
//
//         }
//         true
//
//     }
//
//     #[test]
//     fn test_swap_contracts()  {
//
//         let dico_tests = load_dico_tests();
//
//         //let dico_tests: Vec<HashMap<String, Value>> = vec![load_dico_tests()];
//         for el in dico_tests.iter() {
//
//             let curr_test = el.extract_hmap().unwrap();
//
//             let curr_identifier = curr_test.get("identifier").unwrap().as_string();
//             let curr_terms = curr_test.get("terms").unwrap().extract_hmap();
//             let curr_to = curr_test.get("to").unwrap().as_string();
//             let curr_data_observed = curr_test.get("dataObserved").unwrap().extract_hmap(); // verifier si cest None
//             let curr_events_observed = curr_test.get("eventsObserved").unwrap().extract_vec();
//             let curr_results = curr_test.get("results").unwrap().extract_vec().unwrap();
//             //let a = curr_results.get(0).unwrap().get("notionalPrincipal").unwrap().as_string().unwrap();
//             let to_date = if let Some(curr_to) = curr_to {
//                 IsoDatetime::parse_from_str(&curr_to, "%Y-%m-%dT%H:%M:%S").ok()
//             } else {
//                 None
//             };
//
//             let mut contract_model: Box<Result<ContractModel, ContractError>> = if let Some(ref curr_terms) = curr_terms {
//                 // Supposons que ContractModel::new retourne Result<ContractModel, String>
//                 match ContractModel::new(&curr_terms) {
//                     Ok(model) => Box::new(Ok(model)),
//                     Err(e) => Box::new(Err(ContractError::from(e))),
//                 }
//             } else {
//                 Box::new(Err(ContractError::MissingTerms))
//             };
//
//             let risk_factor_model = RiskFactorModel;
//
//
//             let mut vec_results: Vec<HashMap<String, Value>> = vec![];
//             if let Ok(cm) = contract_model.as_ref() {
//                 let mut events = Swap::schedule(&to_date.unwrap(), cm); //PrincipalAtMaturity::schedule(&to_date, cm);
//
//                 if let Ok(events_res) = events {
//                     let events2 = Swap::apply(events_res, cm, &risk_factor_model);
//
//                     for ce in events2.iter() {
//                         let mut sub_res_hm: HashMap<String, Value> = HashMap::new();
//                         sub_res_hm.insert("eventDate".to_string(), Value::String( ce.event_time.unwrap().format("%Y-%m-%dT%H:%M:%S").to_string() ));
//                         sub_res_hm.insert("eventType".to_string(), Value::String( ce.event_type.clone().to_string() ));
//                         sub_res_hm.insert("payoff".to_string(), Value::F64( ce.payoff.unwrap() ));
//                         sub_res_hm.insert("currency".to_string(),Value::String( ce.currency.clone().unwrap() ));
//                         let ci = ce.contract_id.clone().unwrap();
//
//                         let vContRef: Vec<ContractReference> = cm.contractStructure.clone()
//                             .unwrap_or_default()
//                             .iter()
//                             .filter(|cr| {
//                                 cr.object.as_cm()
//                                     .and_then(|cm_obj| cm_obj.contract_id.clone())
//                                     .map_or(false, |id| id == ci)
//                             })
//                             .cloned()
//                             .collect();
//
//                         let not2 = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.notionalPrincipal.clone())
//                             .unwrap_or_default();
//
//                         sub_res_hm.insert("notionalPrincipal".to_string(), Value::F64( not2 ));
//
//                         let nomInt2 = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.nominalInterestRate.clone())
//                             .unwrap_or_default();
//                         sub_res_hm.insert("nominalInterestRate".to_string(), Value::F64( nomInt2  ));
//
//                         let accInt = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.accruedInterest.clone())
//                             .unwrap_or_default();
//                         sub_res_hm.insert("accruedInterest".to_string(), Value::F64( accInt  ));
//
//                        
//                         vec_results.push(sub_res_hm);
//                     }
//
//
//                 }
//             }
//              // enlever pour faire les autres tests
//
//             let mut v1  = convert_value_map_to_string_map_ref(&vec_results);
//             let mut v2  = convert_value_map_to_string_map_ref(&curr_results.clone());
//             let t = are_contracts_equal(vec_results, curr_results.clone());
//             //assert!(are_contracts_equal(&vec_results, &curr_results));
//             break;
//
//         }
//
//     }
// }
