
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::capfl::pof::POF_NET_CAPFL::POF_NET_CAPFL;
use crate::functions::capfl::stf::STF_NET_CAPFL::STF_NET_CAPFL;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_reset_rate::LifeCap::LifeCap;
use crate::terms::grp_reset_rate::LifeFloor::LifeFloor;
use crate::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct CAPFL {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for CAPFL {
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
        let contract_role = ContractRole::provide(sm, "contractRole");

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

        let ct = ContractTerms {
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: contract_role,
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
            life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
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
        // Compute underlying event schedule
        let model = &self.contract_terms;
        let underlying_model = &self.contract_structure.clone().unwrap().0
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone().as_cm()))
            .map(|obj| {
                let mut m = obj.unwrap();
                m.contract_role = Some(ContractRole::new(Some("RPA")).expect("good contract role")); //   .add_attribute("contractRole", ContractRole::RPA);
                m
            })
            .ok_or("Underlying model not found")?;

        let umat = underlying_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
        let mut events = ContractType::schedule(
            Some(umat),
            &underlying_model,
        ).unwrap();

        // Purchase
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Termination
        if let Some(termination_date) = &model.termination_date {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.compare_to(&termination.to_iso_datetime_event()) != 1);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&status_event.to_iso_datetime_event()) != -1);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let events = &mut self.contract_events.clone();
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        // Evaluate events of underlying without cap/floor applied
        let underlying_model = &self.contract_structure.clone().unwrap()
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone()))
            .ok_or("Underlying model not found").unwrap();

        let underlying_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = ContractType::apply(events.clone(), &underlying_model.as_cm().unwrap(), observer).unwrap()
            .into_iter()
            .filter(|e| e.event_type == EventType::IP)
            .collect();//::<Vec<_>>();

        // Evaluate events of underlying with cap/floor applied
        let mut underlying_model_with_cap_floor = underlying_model.clone().as_cm().unwrap();
        underlying_model_with_cap_floor.life_cap = &self.contract_terms.life_cap.clone();
        underlying_model_with_cap_floor.life_floor = &self.contract_terms.life_floor.clone();

        let mut underlying_with_cap_floor_events = events
            .into_iter()
            .map(|e| e.copy())
            .collect();

        underlying_with_cap_floor_events = ContractType::apply(underlying_with_cap_floor_events, &underlying_model_with_cap_floor, observer).unwrap()
            .into_iter()
            .filter(|e| e.event_type == EventType::IP)
            .collect::<Vec<_>>();

        // Net schedules of underlying with and without cap/floor applied
        let mut merged_events = HashMap::new();

        for e in underlying_events.iter().chain(underlying_with_cap_floor_events.iter()) {
            let key = format!("{:?}{:?}", e.event_time, e.event_type);
            let existing_event = merged_events.get(&key);

            let new_event = match existing_event {
                Some(existing) => Self::netting_event(existing, e, model, observer),
                None => e.copy(),
            };

            merged_events.insert(key, new_event);
        }

        let mut events = Vec::from_iter(merged_events.into_values());
        events.sort();

        // Remove pre-purchase events if purchase date set
        if let Some(purchase_date) = &self.contract_terms.purchase_date {
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.compare_to(&purchase_event.to_iso_datetime_event()) != -1);
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        todo!()
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

impl CAPFL {
    pub fn netting_event(
        &self,
        e1: &ContractEvent<IsoDatetime, IsoDatetime>,
        e2: &ContractEvent<IsoDatetime, IsoDatetime>,
    ) -> ContractEvent<IsoDatetime, IsoDatetime> {
        let mut e = EventFactory::create_event(
            &e1.event_time,
            &e1.event_type,
            &e1.currency,
            Some(Rc::new(POF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            Some(Rc::new(STF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            &None,
            &self.contract_terms.contract_id,
        );

        e.eval(
            &mut StateSpace::default(),
            model,
            observer,
            &DayCountConvention::new(Some("AAISDA"), None, None).ok(),//&DayCountCalculator::new("AA", Box::new(NoHolidaysCalendar)),
            model.business_day_adjuster.as_ref().unwrap(),
        );

        e
    }
}

impl fmt::Display for CAPFL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CAPFL")
    }
}