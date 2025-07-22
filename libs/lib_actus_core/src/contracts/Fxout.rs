use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_events::events::EventFactory::EventFactory;
use lib_actus_events::events::EventType::EventType;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;


use crate::attributes::ContractReference::ContractReference;
use lib_actus_terms::ContractTerms::ContractTerms;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;

use crate::functions::fxout::pof::POF_MD1_FXOUT::POF_MD1_FXOUT;
use crate::functions::fxout::pof::POF_MD2_FXOUT::POF_MD2_FXOUT;
use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_STD_FXOUT::POF_STD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;

use crate::functions::fxout::stf::STF_MD1_FXOUT::STF_MD1_FXOUT;
use crate::functions::fxout::stf::STF_MD2_FXOUT::STF_MD2_FXOUT;
use crate::functions::fxout::stf::STF_STD_FXOUT::STF_STD_FXOUT;
use crate::functions::fxout::stf::STF_TD_FXOUT::STF_TD_FXOUT;



use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use lib_actus_terms::terms::grp_contract_identification::ContractType::ContractType;
use lib_actus_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_notional_principal::Currency2::Currency2;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use lib_actus_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use lib_actus_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use lib_actus_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use lib_actus_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use lib_actus_terms::terms::grp_settlement::delivery_settlement::D::D;
use lib_actus_terms::terms::grp_settlement::SettlementPeriod::SettlementPeriod;
use crate::traits::TraitContractModel::TraitContractModel;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::functions::fxout::stf::STF_PRD_FXOUT::STF_PRD_FXOUT;

#[derive(Debug, Clone, PartialEq)]
pub struct FXOUT {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for FXOUT {

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
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
            settlement_period: SettlementPeriod::provide_from_input_dict(sm, "settlementPeriod"),
            ..Default::default()
        };


        self.contract_terms = ct
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = risk_factors.clone(); // RiskFactorModel::new();
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>) {
        self.contract_structure = None;
    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new()) //ResultSet::new()
    }

    fn schedule(&mut self, to: Option<IsoDatetime>) {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let model = &self.contract_terms;
        // Purchase event
        
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_FXOUT)), //
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let e: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_FXOUT)), // STF_TD_STK
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        } else {
            // Settlement events
            if model.delivery_settlement == Some(DeliverySettlement::D(D)) || model.delivery_settlement.is_none() {
                let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &EventType::MD,
                    &model.currency,
                    Some(Rc::new(POF_MD1_FXOUT)),
                    Some(Rc::new(STF_MD1_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());

                let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &EventType::MD,
                    &Some(model.currency2.clone().unwrap().to_currency()),
                    Some(Rc::new(POF_MD2_FXOUT)),
                    Some(Rc::new(STF_MD2_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            }
            else {
                let shifted_maturity_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                    &(

                        model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().add_period(
                            model.settlement_period.clone().unwrap().value().clone()
                        )

                    ).value()
                );

                let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                    &Some(shifted_maturity_date),
                    &EventType::STD,
                    &model.currency,
                    Some(Rc::new(POF_STD_FXOUT)),
                    Some(Rc::new(STF_STD_FXOUT)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.push(e.to_iso_datetime_event());
            }
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.to_iso_datetime_event().compare_to(&status_event.to_iso_datetime_event()) != -1);

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

        events.retain(|e| e.to_iso_datetime_event().compare_to(&to_event) != 1);

        // Sort events according to their time of occurrence
        events.sort();

        self.contract_events = events;
    }

    fn apply(&mut self, result_set_toogle: bool) {

        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }
        let _maturity = &self.contract_terms.maturity_date.clone();
        self.init_state_space(_maturity);
        //let model = &self.contract_terms;
        // let events = &mut self.contract_events;
        let events = &mut self.contract_events.clone();

        // let mut events = events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);
            // event.eval(
            //     &mut states,
            //     model,
            //     &self.contract_risk_factors.clone().unwrap(),
            //     &DayCountConvention::new(Some("AAISDA"), None, None).ok(),
            //     model.business_day_adjuster.as_ref().unwrap(),
            // );
            i+=1;
        }

    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>)  {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
        states.status_date = model.status_date.clone();
        //Ok(states)
        self.states_space = states
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


impl fmt::Display for FXOUT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FXOUT")
    }
}